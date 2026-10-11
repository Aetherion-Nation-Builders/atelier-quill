# The brand layer

Atelier Quill is VectorCraft with a different name on the door. We keep the change small on
purpose: upstream lands around a hundred changes a day, and every file we touch is a file that
can conflict when we pull them in. So:

**Renamed (what a person sees):** the window title, menus, dialogs, the start screen, the About
box, Help-menu links, error messages in the UI crate, installer and package names, the web page
title, and where settings live (`~/Library/Application Support/Atelier Quill`,
`~/.config/vectorcraft` stays on Linux for now).

**Not renamed (machine names):** crate names (`vectorcraft-*`), the binaries (`vectorcraft`,
`vectorcraft-cli`), command ids, the `ai.storyteller.vectorcraft` app id and icon file names in
packaging, the `.vectorcraft` format, environment variables, and the docs under `docs/` and `book/`.
These change only when we decide to stop tracking upstream, in one deliberate pass.

## Where the name lives

| Place | File |
|---|---|
| The constants | `crates/ui-egui/src/identity.rs` (`PRODUCT`, `SHORT`, `SUITE`, `VENDOR`) |
| Translations, every language | rewritten on load in `crates/ui-egui/src/i18n/catalog.rs` (`identity::rewrite`), so the upstream `.tsv` catalogs stay untouched; our own strings and their translations are in `crates/ui-egui/src/i18n/brand.tsv` (`cargo xtask i18n-coverage` reads it too) |
| English UI strings | literal `"Atelier Quill"` in `crates/ui-egui/src` and `apps/` (the catalogs' keys are rewritten to match) |
| macOS app menu | `crates/ui-egui/src/native_menu.rs` `APP_NAME` |
| Links | `crates/ui-egui/src/links.rs` (menu ids keep their upstream names, the URLs are ours) |
| Windows file properties | `apps/vectorcraft/build.rs`, `apps/vectorcraft-cli/build.rs` |
| macOS bundle | `packaging/macos/Info.plist.in` (display name, copyright; identifier unchanged) |
| Windows installer | `packaging/windows/vectorcraft.wxs` (names, manufacturer, links; registry keys unchanged) |
| Linux desktop entry, AppStream, nfpm | `packaging/linux/ai.storyteller.vectorcraft.*`, `nfpm.yaml` (names, vendor, links; ids unchanged) |
| Help links | `crates/engine/src/cmd/help.rs` (URLs and labels; command ids unchanged) |
| Browser build | `apps/vectorcraft-web/index.html` |
| README and NOTICE | README: our header above upstream's feature sections; NOTICE: ours on top of theirs, kept on merge by `.gitattributes` |

Still upstream's name, to be done in later passes: error messages and parameter docs inside the
engine and pdf crates (`crates/engine`, `crates/pdf`, including the "VectorCraft Default" PDF
preset name), the macOS DMG window background (`packaging/macos/dmg/`, which carries the old icon
and name and needs resvg to regenerate), the `docs/` manuals and screenshots, and the release
workflow's artifact names. The legacy DrawCraft name stays wherever old files and preferences
need it. The app icon is already ours (`assets/app-icon/README.md`).

## Renaming the product

Change `PRODUCT`/`SHORT` in `identity.rs`, then search the tree for the old name in string
literals (`grep -rn '"[^"]*Atelier Quill' crates/ui-egui/src apps packaging`), and run
`cargo test -p vectorcraft-ui-egui identity`.
