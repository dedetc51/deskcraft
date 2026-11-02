# deskcraft Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Publier le code source de deskcraft et préparer des builds Windows/macOS et un canal de mise à jour signé, sans créer de release avant accord distinct.

**Architecture:** Partir du tag RustDesk 1.5.1, garder les fonctions natives, séparer l'identité et les données de deskcraft, puis adapter le chemin de mise à jour signé déjà présent dans l'amont. Les workflows de compilation ne possèdent pas les secrets de publication ; un workflow manuel protégé signe et publie les artefacts.

**Tech Stack:** Rust 1.81, Flutter 3.24.5, vcpkg figé, GitHub Actions, Ed25519, SHA-256, Developer ID/notarytool, Authenticode.

**Spec:** `docs/superpowers/specs/2026-11-02-deskcraft-design.md`

## Global Constraints

- Dépôt `dedetc51/deskcraft` public ; première GitHub Release uniquement après accord explicite.
- Windows 10/11 x64 EXE/MSI ; macOS 12.3+ arm64 et x86_64 DMG.
- deskcraft coexiste avec RustDesk ; aucune migration implicite des données.
- Aucune clé privée ni aucun certificat dans Git, les logs CI ou les artefacts non publiés.
- Échec de signature, empreinte, notarisation ou architecture : aucune installation.

## Review Focus

- Réponse de mise à jour pointant vers `rustdesk/rustdesk` : rejet avant téléchargement.
- Métadonnées signées par la clé RustDesk ou pour le paquet `rustdesk` : rejet.
- Artefact d'une mauvaise architecture ou d'un mauvais format : rejet.
- Version inférieure à la version installée : rejet ; retour stable via nouveau numéro supérieur.
- Secrets absents ou signature de plateforme impossible : build de test conservé, release stable bloquée.

---

### Task 1: Environnement et provenance

**Files:** importer le dépôt RustDesk 1.5.1 dans `work/deskcraft`, ajouter le spec et ce plan sous `docs/superpowers/`, conserver `LICENSE`, `.gitmodules` et la source amont.

**Interfaces:** produit un dépôt Git public traçable et un poste local avec versions d'outils vérifiées.

- [ ] Installer chaque outil local autorisé et vérifier sa version ; documenter les échecs liés aux droits ou à Xcode.
- [ ] Cloner le tag 1.5.1 avec sous-modules, vérifier commit et licence.
- [ ] Créer `dedetc51/deskcraft` public et pousser le commit de base, sans release.
- [ ] Vérifier le dépôt distant et l'absence de secrets.

### Task 2: Identité et mise à jour isolée

**Files:** `src/common.rs`, `src/update_metadata.rs`, `src/updater/artifact.rs`, `src/update_metadata_tests.rs`, `flutter/lib/desktop/pages/desktop_home_page.dart`, `res/update/generate_update_metadata.py`, autres chemins seulement si nécessaires.

**Interfaces:** le client accepte uniquement les métadonnées Ed25519 de deskcraft dans `dedetc51/deskcraft` et les artefacts associés.

- [ ] Ajouter des tests de refus des URL, clés, paquet et artefacts RustDesk ; vérifier leur échec initial.
- [ ] Paramétrer dépôt de release, endpoint de découverte, identité de paquet et clé publique deskcraft ; supprimer la découverte via `api.rustdesk.com`.
- [ ] Vérifier les tests de signature, SHA-256, architecture, format et version ; exécuter la suite pertinente.
- [ ] Vérifier l'isolation des chemins de données et identifiants d'installation, puis corriger uniquement les collisions constatées.

### Task 3: Builds et publication contrôlée

**Files:** `.github/workflows/deskcraft-build.yml`, `.github/workflows/deskcraft-release.yml`, scripts de signature et documentation des secrets.

**Interfaces:** les builds produisent EXE/MSI/DMG de test ; la publication exige une validation d'environnement et tous les artefacts signés.

- [ ] Adapter la chaîne amont pour Windows x64, macOS arm64 et x86_64, avec versions et actions figées.
- [ ] Ajouter tests et contrôles des artefacts, permissions minimales, provenance et blocage des secrets en PR.
- [ ] Ajouter signature Authenticode, Developer ID, notarisation et métadonnées Ed25519 au workflow manuel protégé ; échec fermé si un secret manque.
- [ ] Valider les fichiers YAML et scripts, puis exécuter les builds CI sans publication et inspecter chaque résultat.

### Task 4: Retour stable et personnalisation future

**Files:** tests de mise à jour, `docs/RELEASE.md`, `docs/BRANDING.md`, dossier d'assets documenté.

**Interfaces:** procédure de promotion d'un ancien code sous une nouvelle version et inventaire graphique sans changement des fonctions natives.

- [ ] Ajouter un test de refus de downgrade et documenter la reconstruction d'une version stable avec numéro supérieur.
- [ ] Documenter rotation de clé, gestion des secrets, signature, notarisation et publication manuelle.
- [ ] Inventorier les assets et identifiants à personnaliser ; conserver les originaux jusqu'à livraison des nouveaux visuels.
- [ ] Vérifier les tests et la diff, pousser le code et demander l'accord avant toute release publique.
