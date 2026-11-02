use super::*;
use hbb_common::{
    base64::engine::general_purpose::STANDARD,
    sodiumoxide::{self, crypto::sign},
    ResultType,
};
use serde_json::{json, Value};

const KEY_ID: &str = "test-ed25519-main";
const SHA256: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

struct Fixture {
    metadata: Vec<u8>,
    signature: Vec<u8>,
    public_key: [u8; 32],
}

fn artifact() -> Value {
    json!({"platform":"windows","arch":"x86_64","format":"exe","url":"https://github.com/dedetc51/deskcraft/releases/download/v1.4.6/deskcraft-1.4.6-x86_64.exe","file_name":"deskcraft-1.4.6-x86_64.exe","size":123456,"sha256":SHA256})
}

fn metadata() -> Value {
    json!({"schema_version":1,"app":"deskcraft","package_id":"deskcraft","version":"1.4.6","release_id":"v1.4.6","published_at":"2026-05-14T00:00:00Z","signature_key_id":KEY_ID,"artifacts":[artifact()]})
}

fn set_value(metadata: &mut Value, key: &str, value: Value, artifact_field: bool) {
    let target = if artifact_field {
        &mut metadata["artifacts"][0]
    } else {
        metadata
    };
    target
        .as_object_mut()
        .expect("JSON object")
        .insert(key.to_string(), value);
}

fn sign_fixture(metadata: Value, signature_key_id: &str) -> Fixture {
    sodiumoxide::init().expect("test sodiumoxide init");
    let metadata = serde_json::to_vec(&metadata).expect("metadata JSON");
    let (public_key, secret_key) = sign::gen_keypair();
    let mut signed = UPDATE_METADATA_SIGNATURE_CONTEXT.to_vec();
    signed.extend_from_slice(&metadata);
    let sig = sign::sign_detached(&signed, &secret_key);
    let sig_json = json!({"schema_version":1,"algorithm":"ed25519","key_id":signature_key_id,"signature":STANDARD.encode(sig.to_bytes())});
    Fixture {
        metadata,
        signature: serde_json::to_vec(&sig_json).expect("signature JSON"),
        public_key: public_key.0,
    }
}

fn valid_fixture() -> Fixture {
    sign_fixture(metadata(), KEY_ID)
}
fn key(public_key: [u8; 32]) -> TrustedUpdateKey {
    TrustedUpdateKey {
        key_id: KEY_ID,
        algorithm: "ed25519",
        public_key,
    }
}
fn query<'a>() -> UpdateArtifactQuery<'a> {
    UpdateArtifactQuery {
        platform: "windows",
        arch: "x86_64",
        format: "exe",
    }
}
fn requirements<'a>(artifact: UpdateArtifactQuery<'a>) -> MetadataRequirements<'a> {
    MetadataRequirements {
        expected_version: "1.4.6",
        expected_release_id: "v1.4.6",
        artifact,
    }
}
fn verify_with_query(
    fixture: &Fixture,
    query: &UpdateArtifactQuery<'_>,
) -> ResultType<VerifiedUpdateArtifact> {
    let trusted_keys = [key(fixture.public_key)];
    verify_update_metadata_with_options(
        &fixture.metadata,
        &fixture.signature,
        VerificationOptions {
            requirements: requirements(*query),
            expected_package_id: PUBLIC_UPDATE_PACKAGE_ID,
            artifact_url_policy: ArtifactUrlPolicy::ExactPrefix(
                "https://github.com/dedetc51/deskcraft/releases/download/v1.4.6/",
            ),
            trusted_keys: &trusted_keys,
        },
    )
}

fn verify(fixture: &Fixture) -> ResultType<VerifiedUpdateArtifact> {
    verify_with_query(fixture, &query())
}

#[test]
fn accepts_valid_metadata_and_signature() {
    let artifact = verify(&valid_fixture()).expect("verified artifact");
    assert_eq!("deskcraft-1.4.6-x86_64.exe", artifact.file_name);
    assert_eq!(123456, artifact.size);
    assert_eq!(SHA256, artifact.sha256);
    assert_eq!("1.4.6", artifact.version);
}

#[test]
fn rejects_bad_signature_or_tampered_metadata() {
    let mut fixture = valid_fixture();
    fixture.metadata.push(b' ');
    assert!(verify(&fixture).is_err());
    for (key, value) in [
        ("schema_version", json!(2)),
        ("algorithm", json!("rsa")),
        ("signature", json!("not base64")),
        ("signature", json!(STANDARD.encode([1u8; 63]))),
    ] {
        let mut fixture = valid_fixture();
        let mut sig: Value = serde_json::from_slice(&fixture.signature).expect("signature");
        sig.as_object_mut()
            .expect("signature object")
            .insert(key.to_string(), value);
        fixture.signature = serde_json::to_vec(&sig).expect("signature bytes");
        assert!(verify(&fixture).is_err(), "{}", key);
    }
}

#[test]
fn rejects_schema_policy_artifact_and_url_mismatches() {
    let cases = [
        ("schema_version", json!(2), false), ("app", json!("rustdesk"), false),
        ("version", json!("1.4.7"), false), ("release_id", json!("v1.4.7"), false),
        ("package_id", json!("rustdesk"), false), ("signature_key_id", json!("different-key"), false),
        ("arch", json!("x86"), true),
        ("sha256", json!("not-a-sha256"), true),
        ("url", json!("https://github.com/dedetc51/deskcraft/releases/download/v1.4.7/deskcraft-1.4.6-x86_64.exe"), true),
        ("url", json!("https://github.com/rustdesk/rustdesk/releases/download/v1.4.6/deskcraft-1.4.6-x86_64.exe"), true),
        ("url", json!("https://github.com/dedetc51/deskcraft/releases/download/v1.4.6/other.exe"), true),
    ];
    for (key, value, artifact_field) in cases {
        let mut metadata = metadata();
        set_value(&mut metadata, key, value, artifact_field);
        assert!(verify(&sign_fixture(metadata, KEY_ID)).is_err(), "{}", key);
    }
}

#[test]
fn rejects_artifact_file_name_path_components() {
    for file_name in [
        "",
        "nested/rustdesk.exe",
        r"nested\rustdesk.exe",
        "..",
        "rust desk.exe",
        "rustdesk*.exe",
        "rustdesk\n.exe",
        "rüstdesk.exe",
    ] {
        let mut invalid_metadata = metadata();
        invalid_metadata["artifacts"][0]["file_name"] = json!(file_name);
        invalid_metadata["artifacts"][0]["url"] = json!(format!(
            "https://github.com/dedetc51/deskcraft/releases/download/v1.4.6/{file_name}"
        ));
        let fixture = sign_fixture(invalid_metadata, KEY_ID);

        let error = verify_with_query(&fixture, &query())
            .err()
            .expect("invalid file name");

        assert_eq!("invalid update artifact file name", error.to_string());
    }
}

#[test]
fn verifies_metadata_generated_by_python_release_script_fixture() {
    const PYTHON_KEY_ID: &str = "2026-deskcraft-main";
    let metadata = br#"{"app":"deskcraft","artifacts":[{"arch":"x86_64","file_name":"deskcraft-1.4.6-x86_64.exe","format":"exe","platform":"windows","sha256":"eaf217e35324159cfbbc97d604cf5a3e7c6fe725ed5f64d75955dac79db5aabf","size":9,"url":"https://github.com/dedetc51/deskcraft/releases/download/v1.4.6/deskcraft-1.4.6-x86_64.exe"}],"package_id":"deskcraft","published_at":"2026-05-14T00:00:00Z","release_id":"v1.4.6","schema_version":1,"signature_key_id":"2026-deskcraft-main","version":"1.4.6"}"#;
    let signature = br#"{"algorithm":"ed25519","key_id":"2026-deskcraft-main","schema_version":1,"signature":"uOO0MHs+Q4bdKAkcCcEVgrUXLjHHjGZZ/iUKIDwJav0wF0aJpDXn08QEIGfx+HPcO4ejknrv0M1uWvJlEcjVBg=="}"#;
    let trusted_key = TrustedUpdateKey {
        key_id: PYTHON_KEY_ID,
        algorithm: "ed25519",
        public_key: [
            204, 21, 218, 12, 208, 165, 93, 158, 15, 79, 85, 222, 108, 120, 36, 80, 64, 141, 239, 252, 22, 126, 34, 135, 211, 168, 195, 120, 170, 184, 103, 4,
        ],
    };

    let trusted_keys = [trusted_key];
    let artifact = verify_update_metadata_with_options(
        metadata,
        signature,
        VerificationOptions {
            requirements: requirements(query()),
            expected_package_id: PUBLIC_UPDATE_PACKAGE_ID,
            artifact_url_policy: ArtifactUrlPolicy::ExactPrefix(
                "https://github.com/dedetc51/deskcraft/releases/download/v1.4.6/",
            ),
            trusted_keys: &trusted_keys,
        },
    )
    .expect("Python-generated update metadata verifies");

    assert_eq!("deskcraft-1.4.6-x86_64.exe", artifact.file_name);
}

#[path = "update_metadata/offline_tests.rs"]
mod offline_tests;
