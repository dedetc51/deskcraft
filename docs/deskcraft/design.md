# deskcraft — conception du client de contrôle à distance

## Intention

Créer un client de contrôle à distance nommé **deskcraft**, dérivé de RustDesk open source, pour Windows et macOS Apple Silicon/Intel. Conserver les fonctions natives de l'application (contrôle à distance, transfert, presse-papiers, audio, accès non surveillé et paramètres réseau) tout en maîtrisant entièrement la compilation, la distribution et le canal des mises à jour.

## Point de départ vérifié le 2 novembre 2026

- Source de référence : `rustdesk/rustdesk` version stable `1.5.1`, commit `23e5bb471a040d62f0a6ad52591fb30784265b73`, licence AGPL-3.0. Le sous-module `libs/hbb_common` doit être récupéré récursivement et figé au commit déclaré par cette version.
- Le code amont contient déjà un format de métadonnées de mise à jour signé Ed25519, une vérification SHA-256 des artefacts et une logique de mise à jour Windows/macOS. Il est actuellement lié au domaine de contrôle, au dépôt GitHub, à l'identité de paquet et à la clé publique RustDesk. Les scripts de publication amont sont aussi liés à `rustdesk/rustdesk`.
- Les builds amont emploient Flutter 3.24.5 sur Windows x64 et macOS, Flutter 3.44.9 sur Windows ARM64, Rust 1.81 sur macOS, vcpkg figé et deux runners macOS séparés pour x86_64 et arm64. deskcraft cible Windows x64 et macOS arm64/x86_64 au premier lot. Windows ARM64 pourra être ajouté après ces trois builds vérifiés.
- Mac local : Apple Silicon, macOS 27.0, outils Xcode CLI et Git/gh présents, compte GitHub connecté `dedetc51`, environ 45 Gio libres. Rust 1.81, Flutter 3.24.5, vcpkg, CMake, Ninja, NASM 2.16.03 et create-dmg sont installés en espace utilisateur et vérifiés. Homebrew est bloqué par les droits sur /opt/homebrew ; Xcode complet et les certificats de signature sont absents.

## Architecture proposée

### Sources et identité

Créer un dépôt GitHub public `dedetc51/deskcraft`, avec historique de provenance du tag RustDesk 1.5.1, sous-modules, licence, notices et attribution conservés. La première version du code change seulement les points nécessaires à l'identité de deskcraft et aux mises à jour. Les fonctionnalités natives restent activées et les changements de comportement hors mise à jour sont évités. Ajouter un répertoire de préparation graphique (inventaire des icônes, noms, identifiants d'application, écran d'accueil et règles de remplacement), sans redessiner l'interface à ce stade.

Choix validé : deskcraft s'installe à côté de RustDesk. Il possède ses propres identifiants d'application, chemins de données et identités d'installation, sans reprendre silencieusement la configuration ou le service d'une installation RustDesk existante. Les utilisateurs pourront configurer explicitement le même serveur ou importer leurs réglages dans une étape ultérieure.

### Compilation

GitHub Actions : génération du pont Rust/Flutter, puis matrice Windows x64, macOS arm64 et macOS x86_64. Partir des étapes amont à la version figée, réduire aux plateformes demandées, pinner les actions et dépendances, limiter les permissions du `GITHUB_TOKEN`, interdire les secrets aux PR non fiables. Les artefacts de CI sont conservés comme résultats de vérification ; seule une exécution de publication approuvée peut créer une GitHub Release. Vérifier la version intégrée et les tests de mise à jour pour chaque plateforme. La compilation locale sur le Mac est une vérification supplémentaire après installation autorisée des outils.

Périmètre validé du premier lot : Windows 10/11 x64 en EXE et MSI ; macOS 12.3 ou plus récent en deux DMG distincts, arm64 et x86_64. Les mises à jour ciblent l'architecture et le format déjà installés. Les seuils seront validés sur les runners et appareils de test avant publication.

### Canal de mise à jour deskcraft

Remplacer tous les points d'entrée de mise à jour RustDesk par un canal deskcraft : découverte de la dernière version stable, lien affiché dans l'interface, hôte et chemin GitHub des Releases, identité de paquet, clé publique intégrée et génération des métadonnées. Le client refuse toute réponse dont l'URL ne correspond pas exactement au dépôt deskcraft prévu. Les métadonnées signées Ed25519 lient version, identifiant de release, plateforme, architecture, nom, taille et SHA-256 de l'artefact. Le téléchargement se fait en HTTPS ; la signature et l'empreinte sont vérifiées avant installation. Échec de vérification = aucune installation.

La clé privée de signature des métadonnées reste hors du dépôt et n'est accessible qu'au job de publication protégé par un environnement GitHub avec approbation. La clé publique est intégrée au client et peut être exposée. Les certificats Apple/Windows et clés de notarisation suivent la même séparation. Prévoir une rotation par chevauchement de deux clés publiques dans une version signée par l'ancienne clé, puis retrait de l'ancienne dans une version ultérieure.

La découverte des versions ne pointe jamais vers `api.rustdesk.com`. Le dépôt source est public sur autorisation de l’utilisateur ; la création d’une GitHub Release et la distribution des applications feront l’objet d’un accord distinct. La source correspondante devra être mise à disposition conformément à l'AGPL-3.0 lors de la distribution.

### Signature, notarisation et retour stable

macOS : signer l'application avec un certificat Developer ID, activer le hardened runtime avec les droits nécessaires, créer le DMG, soumettre via `notarytool`, agrafer le ticket et vérifier `codesign`/`spctl` avant publication. Windows : signer EXE/MSI avec un certificat Authenticode via un mécanisme secret adapté, puis vérifier la signature. Ne publier aucun artefact destiné à l'auto-mise à jour si sa signature de plateforme ou sa notarisation requise échoue.

Les releases et métadonnées signées sont immuables. Pour revenir à un état stable, reconstruire l'ancien code comme **nouvelle version numérotée plus haut**, signer et publier celle-ci ; on ne remplace pas en place un ancien asset ni ne force un téléchargement vers un numéro inférieur. Conserver au moins une release stable précédente et un journal de provenance des commits et empreintes. Un test doit démontrer le retour stable et le refus d'un downgrade non autorisé.

## Hors périmètre du premier lot

- Serveur RustDesk auto-hébergé, migration des utilisateurs et personnalisation graphique effective.
- Windows ARM64, Windows 32 bits, Linux, Android, iOS.
- GitHub Release publique ou import de secrets sans autorisation explicite.

## Étapes et critères de sortie

1. **Inventaire et accord** : confirmer l'identité GitHub, la stratégie de visibilité et les installations locales. Vérifier chaque outil après installation ; ne pas installer Xcode complet tant que l'espace disque et le besoin de build local ne sont pas validés.
2. **Sources** : créer le dépôt public autorisé, importer le tag et le sous-module, vérifier licence, provenance et état Git propre.
3. **Mise à jour isolée** : tests qui prouvent le refus des URL et signatures amont, puis implémentation deskcraft et tests de téléchargement/validation/rollback.
4. **CI** : builds des trois cibles, artefacts vérifiés, scripts de signature, notarisation et contrôle de secrets. Aucun build non signé ne devient release stable.
5. **Publication contrôlée** : une fois les certificats, clés et builds disponibles, demander l’accord avant la première release. Vérifier sur appareils Windows et Mac réels l'installation, la mise à jour et le retour stable avant diffusion générale.

## Décisions à confirmer

- Utiliser `dedetc51/deskcraft` comme dépôt source et de distribution public, sur autorisation explicite de l’utilisateur.
- Fourniture d'un compte Apple Developer avec certificat Developer ID et accès notarisation, ainsi que d'un certificat de signature de code Windows. Sans ces éléments, les builds de test restent non distribuables via le canal automatique.
- Le nom public **deskcraft** est confirmé. L'identifiant de bundle macOS et le nom de l'éditeur Windows seront fixés après connaissance de l'entité propriétaire des certificats.
