```
# Bedrock to PE 0.6.1 Converter
Convert Minecraft Bedrock Edition worlds to Minecraft PE 0.6.1 format.

A desktop tool built with Rust + egui that reads modern Bedrock LevelDB worlds and writes PE 0.6.1 compatible `chunks.dat` / `chunks.r.X.Y.dat` files.

## Features
- Convert Bedrock LevelDB worlds to PE 0.6.1 storage format
- Support **finite** (256x256) and **infinite** worlds
- Select dimensions to convert: **Overworld** and/or **Nether**
- Auto‑read world name from source save (`levelname.txt`)
- Support legacy `LegacyTerrain` records and modern paletted sub‑chunks
- Custom block mapping from Bedrock block names to PE 0.6.1 numeric IDs
- Generates `level.dat` in PE 0.6.1 format (little‑endian NBT)

## Requirements
- Windows 10/11
- Rust 1.75+ (for building from source)

## Build
```powershell
cargo build --release
```

Output: `target/release/bedrock-to-pe061.exe`

## Usage

1. Run `bedrock-to-pe061.exe`
2. Select the input folder: your Bedrock world's db directory
3. Select the output folder: an empty folder for the PE 0.6.1 world
4. Choose world type:
   - Finite (256x256) — classic PE 0.6.1 world
   - Infinite — infinite world (32x32 chunks per region file)
5. Choose dimensions to convert:
   - Overworld
   - Nether
6. Enter or confirm the world name (auto‑filled from source if available)
7. Click Start Convert
8. Copy the output folder into your PE 0.6.1 `minecraftWorlds` directory

## Output Format

### Finite worlds

- `chunks.dat` — overworld chunk data
- `chunks_nether.dat` — nether chunk data
- `level.dat` — world metadata

### Infinite worlds

- `chunks.r.<rx>.<rz>.dat` — overworld region files (32x32 chunks each)
- `chunks_nether.r.<rx>.<rz>.dat` — nether region files
- `level.dat` — world metadata

## Block Mapping

Blocks are mapped from Bedrock block names (e.g. `minecraft:stone`) to PE 0.6.1 numeric IDs. Unmapped blocks are replaced with `info_update2` (ID 249) as a placeholder.

See `src/block_map.rs` for the full mapping table.

## Project Structure

```
src/
├── main.rs        — egui UI and app entry point
├── converter.rs   — world conversion logic
└── block_map.rs   — Bedrock → PE 0.6.1 block ID mapping
```

## Dependencies

- [https://crates.io/crates/bedrock-world](https://crates.io/crates/bedrock-world) — Bedrock LevelDB world parsing
- [https://crates.io/crates/eframe](https://crates.io/crates/eframe) / [https://crates.io/crates/egui](https://crates.io/crates/egui) — GUI
- [https://crates.io/crates/rfd](https://crates.io/crates/rfd) — native file dialogs

## Known Limitations

- PE 0.6.1 only supports Y range 0..127; 1.21 worlds use -64..320, so terrain above Y=127 is dropped and below Y=0 is skipped
- Block entities (chests, signs, furnaces) are not converted
- Entities and players are not converted
- Only the first storage layer of multi‑storage paletted sub‑chunks is read
- Biome data is not converted

## License

MIT

## Acknowledgments

- [https://github.com/bedrock-dev/bedrock-world](https://github.com/bedrock-dev/bedrock-world) for Bedrock LevelDB parsing
- [https://github.com/guifeilun/Minecraft0.6.1-improvement-win32-pc-port/) for the PE 0.6.1 port reference

## Disclaimer

This project is for educational and personal use only. Minecraft is a trademark of Mojang AB. This project is not affiliated with Mojang.