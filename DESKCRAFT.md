# deskcraft

deskcraft is an AGPL-3.0 licensed fork of [RustDesk](https://github.com/rustdesk/rustdesk), based on upstream tag `1.5.1` (`23e5bb471a040d62f0a6ad52591fb30784265b73`). The original license is retained. This repository starts with a source snapshot because the initial transfer of upstream Git history timed out; the exact upstream commit is documented for provenance.

The first target is Windows 10/11 x64 (EXE and MSI) and macOS 12.3+ (separate Apple Silicon and Intel DMGs). The desktop identity and update channel are specific to deskcraft so that it can coexist with RustDesk. The remote control features remain based on the upstream implementation.

The CI workflow builds **unsigned test artifacts**. No deskcraft stable release is available yet. Platform code-signing certificates, Apple notarization credentials, completed runner builds, and device validation are required before publishing one. See [release process](docs/DESKCRAFT_RELEASE.md) and [branding inventory](docs/DESKCRAFT_BRANDING.md).

Update metadata uses a project-specific Ed25519 key. The public key is embedded in the client; its private half is stored outside the repository and as a protected GitHub environment secret. The client accepts update artifacts only from this repository with matching signed metadata and SHA-256 hashes. A GitHub release or an unsigned CI artifact alone is not an approved update.
