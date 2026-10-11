# Atelier Quill app icon

**The hidden sun behind a quill**: an ivory disc, a quill in ochre drawn across it nib-down,
and the stroke it leaves, on a volcanic black field. Three colours, from the Nation's palette:

| Colour | Hex | Used for |
|---|---|---|
| Volcanic black | `#141210` | the field, the quill's spine, and the gap around the quill |
| Ivory | `#efe6d2` | the sun |
| Ochre | `#d9a441` | the quill and its stroke |

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112`, as all the Atelier icons. The
macOS files add Apple's 824/1024 margin (`vectorcraft-macos-512.png`, `.icns`); the others are
the full tile. `vectorcraft.svg` is the master; the file names keep their upstream `vectorcraft`
names so packaging stays untouched.

**Provenance:** drawn for Atelier Quill by hand as SVG (2026). It replaces VectorCraft's dragon
icon, which stays in VectorCraft's own repository. Licence: `LICENSE.txt` (MIT OR Apache-2.0).

**Regenerating:** `packaging/icons.sh` (needs resvg; iconutil on macOS for the .icns). The
committed renders were made from the same SVG with cairosvg and Pillow.
