# Branding inventory

Brand assets are placeholders inherited from RustDesk until a visual identity is supplied. Keep their original attribution and replace only assets for which rights are available.

| Surface | Source to customize |
| --- | --- |
| Windows application icon | `flutter/windows/runner/resources/app_icon.ico` |
| macOS application icon | `flutter/macos/Runner/AppIcon.icns` |
| Flutter icon and logo | `flutter/assets/icon.svg`; optional `flutter/assets/logo.png`, `flutter/assets/logo_light.png`, `flutter/assets/logo_dark.png` |
| Windows version information | `flutter/windows/runner/Runner.rc` |
| macOS bundle identity | `flutter/macos/Runner/Configs/AppInfo.xcconfig`, `flutter/macos/Runner/Info.plist` |
| Installer identity | `res/msi/preprocess.py`, `.github/workflows/flutter-build.yml` |

Use `Deskcraft` for the installed app and `deskcraft` for executable and release asset names. Keep the bundle ID `com.dedetc51.deskcraft` unless a future signing identity requires a coordinated migration. Icon replacements must include all required sizes; validate on both light and dark desktop themes before publication.
