# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-10-05

### Added

- Initial release
- Bedrock LevelDB to PE 0.6.1 world conversion
- Finite (256x256) and infinite world support
- Overworld and Nether dimension selection
- Auto-read world name from source save
- Legacy `LegacyTerrain` and modern paletted sub-chunk support
- Block mapping from Bedrock block names to PE 0.6.1 numeric IDs
- `level.dat` generation in PE 0.6.1 format (little-endian NBT)
- egui-based GUI with file pickers and progress bar