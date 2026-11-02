# deskcraft release process

## Current state

`deskcraft-ci.yml` builds Windows x64 EXE/MSI and macOS arm64/x86_64 DMGs as **unsigned CI artifacts**. The `release` GitHub environment requires a reviewer. `deskcraft-release-readiness.yml` is a manual credential and key check; it has no write permission and cannot publish. A stable release must remain blocked until platform signing and notarization credentials exist, those stages are implemented and verified, and a human approves publication.

The protected environment contains `DESKCRAFT_UPDATE_ED25519_SEED` and `DESKCRAFT_UPDATE_ED25519_PUBLIC_KEY`. The key ID in `src/update_metadata.rs` is `2026-deskcraft-main`. Never place the private seed in source, CI logs, an artifact, or a release. The local seed is in `~/.local/share/deskcraft/keys` with owner-only permissions. Back it up in an encrypted vault outside this Mac before production use.

## Required signing material

- Windows: code-signing certificate PFX and password (`DESKCRAFT_WINDOWS_PFX`, `DESKCRAFT_WINDOWS_PFX_PASSWORD`) or a hardware-backed signing service. Sign and verify both EXE and MSI with Authenticode and a trusted timestamp.
- macOS: Developer ID Application certificate P12 and password (`DESKCRAFT_APPLE_DEVELOPER_ID_P12`, `DESKCRAFT_APPLE_DEVELOPER_ID_PASSWORD`), plus App Store Connect notarization API key, key ID, and issuer (`DESKCRAFT_APPLE_NOTARY_KEY`, `DESKCRAFT_APPLE_NOTARY_KEY_ID`, `DESKCRAFT_APPLE_NOTARY_ISSUER`). Sign the full app bundle with hardened runtime, package each DMG, submit to notarytool, staple the ticket, and verify with `codesign`, `spctl`, and `stapler`.

No platform signing certificate or Apple account is currently available. The release pipeline must be completed and tested once those credentials exist; the readiness workflow deliberately fails until then.

## Publish gate

1. Build the exact commit intended for release on all three runners and pass Rust, Dart, metadata, and updater tests. Confirm installer identity and coexistence on real Windows and macOS devices.
2. Sign each Windows installer; sign, notarize and staple each macOS DMG; verify every resulting package. Keep signing credentials restricted to the protected `release` environment. Never expose them to pull request workflows.
3. Generate `deskcraft-update.json` and `deskcraft-update.json.sig` using `res/update/generate_update_metadata.py`. It binds version, tag, platform, architecture, format, filename, size and SHA-256 to the deskcraft release URL. Verify it independently with the public key and against the exact signed binaries.
4. After explicit approval, create the immutable `vX.Y.Z` GitHub Release with only the signed packages and the two metadata files. Verify the release assets against local hashes. Publish the corresponding AGPL source commit and notices.
5. Test update discovery and installation on each platform. Keep the previous stable release available.

## Rollback

Clients accept only a newer numeric version. To restore an older stable codebase, rebuild that code as a **new, higher** version, sign and notarize it, create new signed metadata and publish after approval. Do not replace release assets in place or repoint a tag. The macOS installer also keeps the previous app bundle during a failed update transaction and restores it on failure.

For key rotation, first ship a version containing both old and new public keys, signed by the old key; then use the new key for later releases. Remove the old key only after installed clients have migrated.
