use std::collections::{HashMap, BTreeMap};
use std::fs::{self, File};
use std::io::{self, Write, Seek};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bedrock_world::{
    BedrockWorld, Dimension,
    parsed::{ParsedChunkData, ParsedChunkRecordValue},
    chunk::{SubChunk, SubChunkFormat, LegacyTerrain},
    world::{WorldScanOptions, WorldThreadingOptions},
};
use crate::block_map::BlockMap;

const CHUNK_WIDTH: usize = 16;
const CHUNK_DEPTH: usize = 16;
const CHUNK_HEIGHT: usize = 128;
const CHUNK_BLOCK_COUNT: usize = CHUNK_WIDTH * CHUNK_DEPTH * CHUNK_HEIGHT;
const CHUNK_DATA_COUNT: usize = CHUNK_BLOCK_COUNT / 2;
const REGION_WIDTH: i32 = 32;
const SECTOR_BYTES: usize = 4096;
const SECTOR_INTS: usize = SECTOR_BYTES / 4;

#[derive(Debug)]
pub enum ConvertError {
    Io(io::Error),
    Bedrock(String),
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::Io(e) => write!(f, "IO error: {}", e),
            ConvertError::Bedrock(s) => write!(f, "Bedrock error: {}", s),
        }
    }
}

impl std::error::Error for ConvertError {}
impl From<io::Error> for ConvertError {
    fn from(e: io::Error) -> Self { ConvertError::Io(e) }
}

pub type ProgressFn = Arc<dyn Fn(f32, &str) + Send + Sync>;

#[derive(Clone, Debug)]
pub struct ConvertOptions {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub infinite: bool,
    pub convert_overworld: bool,
    pub convert_nether: bool,
    pub level_name: String,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            input_path: PathBuf::new(),
            output_path: PathBuf::new(),
            infinite: false,
            convert_overworld: true,
            convert_nether: false,
            level_name: String::new(),
        }
    }
}

struct RegionWriter {
    file: Option<File>,
    offsets: [i32; SECTOR_INTS],
    empty_sector: [u8; SECTOR_BYTES],
    used_sectors: Vec<bool>,
}

impl RegionWriter {
    fn new_single(dir: &Path, dim_suffix: &str) -> io::Result<Self> {
        let filename = format!("chunks{}.dat", dim_suffix);
        Self::new_with_filename(dir, filename)
    }

    fn new_region(dir: &Path, rx: i32, rz: i32, dim_suffix: &str) -> io::Result<Self> {
        let filename = format!("chunks{}.r.{}.{}.dat", dim_suffix, rx, rz);
        Self::new_with_filename(dir, filename)
    }

    fn new_with_filename(dir: &Path, filename: String) -> io::Result<Self> {
        let path = dir.join(&filename);
        let mut file = File::create(&path)?;
        let offsets = [0i32; SECTOR_INTS];
        let offset_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(offsets.as_ptr() as *const u8, SECTOR_BYTES)
        };
        file.write_all(offset_bytes)?;
        let empty_sector = [0u8; SECTOR_BYTES];
        let mut used_sectors = vec![false; SECTOR_INTS];
        used_sectors[0] = true;
        Ok(Self { file: Some(file), offsets, empty_sector, used_sectors })
    }

    fn write_chunk(&mut self, lx: i32, lz: i32, blocks: &[u8], data: &[u8]) -> io::Result<()> {
        assert_eq!(blocks.len(), CHUNK_BLOCK_COUNT);
        assert_eq!(data.len(), CHUNK_DATA_COUNT);
        assert!(lx >= 0 && lx < REGION_WIDTH);
        assert!(lz >= 0 && lz < REGION_WIDTH);

        let mut chunk_data = Vec::with_capacity(CHUNK_BLOCK_COUNT + CHUNK_DATA_COUNT + 16384 * 2 + 256);
        chunk_data.extend_from_slice(blocks);
        chunk_data.extend_from_slice(data);
        chunk_data.extend_from_slice(&[0u8; 16384]); // skyLight
        chunk_data.extend_from_slice(&[0u8; 16384]); // blockLight
        chunk_data.extend_from_slice(&[0u8; 256]);   // updateMap

        let size = chunk_data.len() as i32 + 4;
        let sectors_needed = (size as usize / SECTOR_BYTES) + 1;
        if sectors_needed > 256 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "chunk too large"));
        }

        let idx = (lz * REGION_WIDTH + lx) as usize;
        let old_offset = self.offsets[idx];
        let old_sector = (old_offset >> 8) as usize;
        let old_count = (old_offset & 0xFF) as usize;

        let mut slot = 1usize;
        let mut run = 0usize;
        while run < sectors_needed {
            if slot + run >= self.used_sectors.len() { break; }
            if !self.used_sectors[slot + run] { run += 1; } else { slot += run + 1; run = 0; }
        }
        if slot + run >= self.used_sectors.len() {
            let extend = sectors_needed - run;
            let file = self.file.as_mut().unwrap();
            file.seek(io::SeekFrom::End(0))?;
            for _ in 0..extend {
                file.write_all(&self.empty_sector)?;
                self.used_sectors.push(false);
            }
        }

        if old_count > 0 {
            for i in 0..old_count {
                if old_sector + i < self.used_sectors.len() {
                    self.used_sectors[old_sector + i] = false;
                }
            }
        }

        for i in 0..sectors_needed { self.used_sectors[slot + i] = true; }
        self.offsets[idx] = ((slot as i32) << 8) | (sectors_needed as i32);

        let file = self.file.as_mut().unwrap();
        file.seek(io::SeekFrom::Start((slot * SECTOR_BYTES) as u64))?;
        file.write_all(&size.to_le_bytes())?;
        file.write_all(&chunk_data)?;

        file.seek(io::SeekFrom::Start((idx * 4) as u64))?;
        file.write_all(&self.offsets[idx].to_le_bytes())?;
        Ok(())
    }

    fn finish(&mut self) -> io::Result<()> {
        if let Some(mut f) = self.file.take() {
            f.seek(io::SeekFrom::Start(0))?;
            let bytes: &[u8] = unsafe {
                std::slice::from_raw_parts(self.offsets.as_ptr() as *const u8, SECTOR_BYTES)
            };
            f.write_all(bytes)?;
            f.sync_all()?;
        }
        Ok(())
    }
}

fn set_nibble(data: &mut [u8], x: usize, y: usize, z: usize, val: u8) {
    let idx = (x << 11 | z << 7 | y) >> 1;
    if idx >= data.len() { return; }
    if y % 2 == 0 {
        data[idx] = (data[idx] & 0xF0) | (val & 0x0F);
    } else {
        data[idx] = (data[idx] & 0x0F) | ((val & 0x0F) << 4);
    }
}

fn get_state_value(states: &BTreeMap<String, bedrock_world::nbt::NbtTag>, key: &str) -> Option<String> {
    match states.get(key) {
        Some(bedrock_world::nbt::NbtTag::String(s)) => Some(s.clone()),
        Some(bedrock_world::nbt::NbtTag::Byte(b)) => Some(b.to_string()),
        Some(bedrock_world::nbt::NbtTag::Int(i)) => Some(i.to_string()),
        Some(bedrock_world::nbt::NbtTag::Short(s)) => Some(s.to_string()),
        _ => None,
    }
}

fn legacy_id_to_name(id: u8) -> Option<&'static str> {
    Some(match id {
        0 => "minecraft:air",
        1 => "minecraft:stone",
        2 => "minecraft:grass",
        3 => "minecraft:dirt",
        4 => "minecraft:cobblestone",
        5 => "minecraft:planks",
        6 => "minecraft:sapling",
        7 => "minecraft:bedrock",
        8 => "minecraft:flowing_water",
        9 => "minecraft:water",
        10 => "minecraft:flowing_lava",
        11 => "minecraft:lava",
        12 => "minecraft:sand",
        13 => "minecraft:gravel",
        14 => "minecraft:gold_ore",
        15 => "minecraft:iron_ore",
        16 => "minecraft:coal_ore",
        17 => "minecraft:log",
        18 => "minecraft:leaves",
        19 => "minecraft:sponge",
        20 => "minecraft:glass",
        21 => "minecraft:lapis_ore",
        22 => "minecraft:lapis_block",
        23 => "minecraft:dispenser",
        24 => "minecraft:sandstone",
        25 => "minecraft:noteblock",
        26 => "minecraft:bed",
        27 => "minecraft:golden_rail",
        28 => "minecraft:detector_rail",
        29 => "minecraft:sticky_piston",
        30 => "minecraft:web",
        31 => "minecraft:tallgrass",
        32 => "minecraft:deadbush",
        33 => "minecraft:piston",
        34 => "minecraft:piston_head",
        35 => "minecraft:wool",
        36 => "minecraft:piston_extension",
        37 => "minecraft:yellow_flower",
        38 => "minecraft:red_flower",
        39 => "minecraft:brown_mushroom",
        40 => "minecraft:red_mushroom",
        41 => "minecraft:gold_block",
        42 => "minecraft:iron_block",
        43 => "minecraft:double_stone_slab",
        44 => "minecraft:stone_slab",
        45 => "minecraft:brick_block",
        46 => "minecraft:tnt",
        47 => "minecraft:bookshelf",
        48 => "minecraft:mossy_cobblestone",
        49 => "minecraft:obsidian",
        50 => "minecraft:torch",
        51 => "minecraft:fire",
        52 => "minecraft:mob_spawner",
        53 => "minecraft:oak_stairs",
        54 => "minecraft:chest",
        55 => "minecraft:redstone_wire",
        56 => "minecraft:diamond_ore",
        57 => "minecraft:diamond_block",
        58 => "minecraft:crafting_table",
        59 => "minecraft:wheat",
        60 => "minecraft:farmland",
        61 => "minecraft:furnace",
        62 => "minecraft:lit_furnace",
        63 => "minecraft:standing_sign",
        64 => "minecraft:wooden_door",
        65 => "minecraft:ladder",
        66 => "minecraft:rail",
        67 => "minecraft:stone_stairs",
        68 => "minecraft:wall_sign",
        69 => "minecraft:lever",
        70 => "minecraft:stone_pressure_plate",
        71 => "minecraft:iron_door",
        72 => "minecraft:wooden_pressure_plate",
        73 => "minecraft:redstone_ore",
        74 => "minecraft:lit_redstone_ore",
        75 => "minecraft:unlit_redstone_torch",
        76 => "minecraft:redstone_torch",
        77 => "minecraft:stone_button",
        78 => "minecraft:snow_layer",
        79 => "minecraft:ice",
        80 => "minecraft:snow",
        81 => "minecraft:cactus",
        82 => "minecraft:clay",
        83 => "minecraft:reeds",
        84 => "minecraft:jukebox",
        85 => "minecraft:fence",
        86 => "minecraft:pumpkin",
        87 => "minecraft:netherrack",
        88 => "minecraft:soul_sand",
        89 => "minecraft:glowstone",
        90 => "minecraft:portal",
        91 => "minecraft:lit_pumpkin",
        92 => "minecraft:cake",
        93 => "minecraft:unpowered_repeater",
        94 => "minecraft:powered_repeater",
        95 => "minecraft:invisiblebedrock",
        96 => "minecraft:trapdoor",
        97 => "minecraft:monster_egg",
        98 => "minecraft:stonebrick",
        99 => "minecraft:brown_mushroom_block",
        100 => "minecraft:red_mushroom_block",
        101 => "minecraft:iron_bars",
        102 => "minecraft:glass_pane",
        103 => "minecraft:melon_block",
        104 => "minecraft:pumpkin_stem",
        105 => "minecraft:melon_stem",
        106 => "minecraft:vine",
        107 => "minecraft:fence_gate",
        108 => "minecraft:brick_stairs",
        109 => "minecraft:stone_brick_stairs",
        110 => "minecraft:mycelium",
        111 => "minecraft:waterlily",
        112 => "minecraft:nether_brick",
        113 => "minecraft:nether_brick_fence",
        114 => "minecraft:nether_brick_stairs",
        115 => "minecraft:nether_wart",
        116 => "minecraft:enchanting_table",
        117 => "minecraft:brewing_stand",
        118 => "minecraft:cauldron",
        123 => "minecraft:redstone_lamp",
        124 => "minecraft:lit_redstone_lamp",
        125 => "minecraft:double_wooden_slab",
        126 => "minecraft:wooden_slab",
        127 => "minecraft:cocoa",
        128 => "minecraft:sandstone_stairs",
        133 => "minecraft:emerald_block",
        139 => "minecraft:cobblestone_wall",
        140 => "minecraft:flower_pot",
        141 => "minecraft:carrots",
        142 => "minecraft:potatoes",
        143 => "minecraft:wooden_button",
        144 => "minecraft:skull",
        145 => "minecraft:anvil",
        146 => "minecraft:trapped_chest",
        151 => "minecraft:daylight_detector",
        152 => "minecraft:redstone_block",
        153 => "minecraft:quartz_ore",
        155 => "minecraft:quartz_block",
        156 => "minecraft:quartz_stairs",
        245 => "minecraft:stonecutter",
        246 => "minecraft:glowingobsidian",
        247 => "minecraft:netherreactor",
        248 => "minecraft:info_update",
        249 => "minecraft:info_update2",
        253 => "minecraft:grass_carried",
        254 => "minecraft:leaves_carried",
        255 => "minecraft:info_reserved6",
        _ => return None,
    })
}

fn extract_from_subchunk(
    sub: &SubChunk,
    block_map: &BlockMap,
    blocks: &mut [u8],
    data: &mut [u8],
    replaced: &mut usize,
) {
    let sub_y = sub.y as i32;
    if sub_y < 0 { return; }

    if let SubChunkFormat::Paletted { storages, .. } = &sub.format {
        if let Some(storage) = storages.first() {
            let indices = match &storage.indices {
                Some(v) => v,
                None => return,
            };
            for (i, &idx) in indices.iter().enumerate() {
                let sx = (i >> 8) & 0xF;
                let sz = (i >> 4) & 0xF;
                let sy = i & 0xF;
                let y = sub_y as usize * 16 + sy;
                if y >= CHUNK_HEIGHT { continue; }
                let idx_usize = idx as usize;
                if idx_usize >= storage.states.len() { continue; }
                let state = &storage.states[idx_usize];
                let id = block_map.get(&state.name);
                if id == 249 && !block_map.supports(&state.name) { *replaced += 1; }
                let off = (sx as usize) << 11 | (sz as usize) << 7 | y;
                blocks[off] = id;

                let mut meta = 0u8;
                let name = state.name.as_str();
                
                if name.ends_with("_door") {
                    if get_state_value(&state.states, "upper_block_bit").as_deref() == Some("true") || get_state_value(&state.states, "upper_block_bit").as_deref() == Some("1") { meta |= 8; }
                    let dir = get_state_value(&state.states, "direction").unwrap_or_else(|| "0".into());
                    match dir.as_str() {
                        "0" => meta |= 1, "1" => meta |= 2, "2" => meta |= 3, "3" => meta |= 0, _ => {}
                    }
                } else if name.ends_with("_bed") {
                    if get_state_value(&state.states, "head_piece_bit").as_deref() == Some("true") || get_state_value(&state.states, "head_piece_bit").as_deref() == Some("1") { meta |= 8; }
                    let dir = get_state_value(&state.states, "direction").unwrap_or_else(|| "0".into());
                    match dir.as_str() {
                        "0" => meta |= 0, "1" => meta |= 1, "2" => meta |= 2, "3" => meta |= 3, _ => {}
                    }
                } else if name.ends_with("_button") {
                    let face = get_state_value(&state.states, "facing_direction");
                    match face.as_deref() {
                        Some("0") => meta = 0, Some("1") => meta = 1, Some("2") => meta = 2,
                        Some("3") => meta = 3, Some("4") => meta = 4, Some("5") => meta = 5,
                        _ => meta = 0
                    }
                    if get_state_value(&state.states, "button_pressed_bit").as_deref() == Some("true") || get_state_value(&state.states, "button_pressed_bit").as_deref() == Some("1") {
                        meta |= 8;
                    }
                } else if name == "minecraft:torch" || name == "minecraft:redstone_torch" || name == "minecraft:unlit_redstone_torch" {
                    let face = get_state_value(&state.states, "torch_facing_direction");
                    match face.as_deref() {
                        Some("west") => meta = 1, Some("east") => meta = 2, Some("north") => meta = 3,
                        Some("south") => meta = 4, _ => meta = 5
                    }
                } else if name == "minecraft:lever" {
                    let face = get_state_value(&state.states, "facing_direction");
                    match face.as_deref() {
                        Some("1") => meta = 1, Some("2") => meta = 2, Some("3") => meta = 3,
                        Some("4") => meta = 4, Some("5") => meta = 5, _ => meta = 0
                    }
                    if get_state_value(&state.states, "open_bit").as_deref() == Some("true") || get_state_value(&state.states, "open_bit").as_deref() == Some("1") { meta |= 8; }
                } else if name.ends_with("_stairs") {
                    let dir = get_state_value(&state.states, "weirdo_direction").unwrap_or_else(|| "0".into());
                    match dir.as_str() {
                        "0" => meta |= 2, "1" => meta |= 1, "2" => meta |= 3, "3" => meta |= 0, _ => {}
                    }
                    if get_state_value(&state.states, "upside_down_bit").as_deref() == Some("true") || get_state_value(&state.states, "upside_down_bit").as_deref() == Some("1") { meta |= 4; }
                } else if name.ends_with("_log") || name.ends_with("_wood") {
                    let dir = get_state_value(&state.states, "pillar_axis").unwrap_or_else(|| "y".into());
                    meta = match dir.as_str() { "x" => 4, "z" => 8, _ => 12 };
                } else if name == "minecraft:redstone_wire" {
                    let power = get_state_value(&state.states, "redstone_signal").and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
                    meta = power & 0xF;
                } else if name.ends_with("_trapdoor") {
                    if get_state_value(&state.states, "upside_down_bit").as_deref() == Some("true") || get_state_value(&state.states, "upside_down_bit").as_deref() == Some("1") { meta |= 8; }
                    if get_state_value(&state.states, "open_bit").as_deref() == Some("true") || get_state_value(&state.states, "open_bit").as_deref() == Some("1") { meta |= 4; }
                    let dir = get_state_value(&state.states, "direction").unwrap_or_else(|| "0".into());
                    match dir.as_str() {
                        "0" => meta |= 0, "1" => meta |= 1, "2" => meta |= 2, "3" => meta |= 3, _ => {}
                    }
                }

                set_nibble(data, sx as usize, y, sz as usize, meta);
            }
        }
    }
}

fn extract_from_legacy_terrain(
    lt: &LegacyTerrain,
    block_map: &BlockMap,
    blocks: &mut [u8],
    data: &mut [u8],
    replaced: &mut usize,
) {
    for y in 0..CHUNK_HEIGHT as u8 {
        for z in 0..CHUNK_DEPTH as u8 {
            for x in 0..CHUNK_WIDTH as u8 {
                let id = match lt.block_id_at(x, y, z) {
                    Some(v) => v,
                    None => continue,
                };
                if id == 0 { continue; }
                let name = match legacy_id_to_name(id) {
                    Some(n) => n,
                    None => { *replaced += 1; continue; }
                };
                let new_id = block_map.get(name);
                if new_id == 249 && !block_map.supports(name) { *replaced += 1; }
                let off = (x as usize) << 11 | (z as usize) << 7 | (y as usize);
                blocks[off] = new_id;

                if let Some(meta) = lt.block_data_at(x, y, z) {
                    set_nibble(data, x as usize, y as usize, z as usize, meta);
                }
            }
        }
    }
}

pub fn read_bedrock_level_name(input_path: &Path) -> Option<String> {
    let candidates = [
        input_path.join("levelname.txt"),
        input_path.join("db").join("levelname.txt"),
    ];
    for p in &candidates {
        if let Ok(s) = fs::read_to_string(p) {
            let t = s.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
    }
    None
}

pub fn convert(opts: &ConvertOptions, progress: ProgressFn) -> Result<(), ConvertError> {
    fs::create_dir_all(&opts.output_path)?;
    let block_map = BlockMap::new();

    let world = BedrockWorld::open_blocking(&opts.input_path, Default::default())
        .map_err(|e| ConvertError::Bedrock(e.to_string()))?;

    progress(0.0, "Listing chunks...");
    let positions = world.list_chunk_positions_blocking(
        WorldScanOptions {
            threading: WorldThreadingOptions::Single,
            ..WorldScanOptions::default()
        }
    ).map_err(|e| ConvertError::Bedrock(e.to_string()))?;

    let total = positions.len();
    if total == 0 { return Err(ConvertError::Bedrock("No chunks in world".into())); }
    progress(0.02, &format!("{} chunks total", total));

    let mut single_regions: HashMap<&'static str, RegionWriter> = HashMap::new();
    let mut regions: HashMap<(&'static str, i32, i32), RegionWriter> = HashMap::new();

    let mut converted = 0usize;
    let mut skipped = 0usize;
    let mut replaced = 0usize;

    for (i, pos) in positions.iter().enumerate() {
        let dim_suffix: &'static str = match &pos.dimension {
            Dimension::Overworld => {
                if !opts.convert_overworld { skipped += 1; continue; }
                ""
            }
            Dimension::Nether => {
                if !opts.convert_nether { skipped += 1; continue; }
                "_nether"
            }
            _ => { skipped += 1; continue; }
        };

        if !opts.infinite {
            if pos.x < 0 || pos.x >= 16 || pos.z < 0 || pos.z >= 16 {
                skipped += 1; continue;
            }
        }

        let chunk: ParsedChunkData = match world.parse_chunk_blocking(*pos) {
            Ok(c) => c,
            Err(_) => { skipped += 1; continue; }
        };

        let mut blocks = vec![0u8; CHUNK_BLOCK_COUNT];
        let mut data = vec![0u8; CHUNK_DATA_COUNT];

        for record in &chunk.records {
            match &record.value {
                ParsedChunkRecordValue::SubChunk(sub) => {
                    extract_from_subchunk(sub, &block_map, &mut blocks, &mut data, &mut replaced);
                }
                ParsedChunkRecordValue::LegacyTerrain(lt) => {
                    extract_from_legacy_terrain(lt, &block_map, &mut blocks, &mut data, &mut replaced);
                }
                _ => {}
            }
        }

        if opts.infinite {
            let rx = pos.x.div_euclid(REGION_WIDTH);
            let rz = pos.z.div_euclid(REGION_WIDTH);
            let lx = pos.x.rem_euclid(REGION_WIDTH);
            let lz = pos.z.rem_euclid(REGION_WIDTH);
            let key = (dim_suffix, rx, rz);
            let region = regions.entry(key).or_insert_with(|| {
                RegionWriter::new_region(&opts.output_path, rx, rz, dim_suffix)
                    .expect("cannot create region file")
            });
            region.write_chunk(lx, lz, &blocks, &data)?;
        } else {
            let region = single_regions.entry(dim_suffix).or_insert_with(|| {
                RegionWriter::new_single(&opts.output_path, dim_suffix)
                    .expect("cannot create chunks file")
            });
            region.write_chunk(pos.x, pos.z, &blocks, &data)?;
        }

        converted += 1;
        let pct = 0.05 + 0.9 * (i as f32 / total as f32);
        progress(pct, &format!("Converted {}/{} chunks", i + 1, total));
    }

    progress(0.95, "Flushing region files...");
    for (_, r) in single_regions.iter_mut() { r.finish()?; }
    for (_, region) in regions.iter_mut() { region.finish()?; }

    let level_name = if opts.level_name.is_empty() {
        read_bedrock_level_name(&opts.input_path).unwrap_or_else(|| "World".to_string())
    } else {
        opts.level_name.clone()
    };
    progress(0.98, &format!("Writing level.dat (name: {})...", level_name));
    write_level_dat(&opts.output_path, opts.infinite, 0, &level_name)?;

    progress(1.0, &format!(
        "Done: {} converted, {} skipped, {} blocks replaced", converted, skipped, replaced
    ));
    Ok(())
}

fn write_level_dat(output_dir: &Path, infinite: bool, seed: i64, level_name: &str) -> io::Result<()> {
    let mut nbt = Vec::new();
    nbt.push(10);
    nbt.extend_from_slice(&(0u16).to_le_bytes());
    write_nbt_long(&mut nbt, "RandomSeed", seed);
    write_nbt_int(&mut nbt, "GameType", 1);
    write_nbt_byte(&mut nbt, "AllowCommands", 0);
    write_nbt_int(&mut nbt, "SpawnX", 0);
    write_nbt_int(&mut nbt, "SpawnY", 64);
    write_nbt_int(&mut nbt, "SpawnZ", 0);
    write_nbt_long(&mut nbt, "Time", 0);
    write_nbt_long(&mut nbt, "SizeOnDisk", 0);
    write_nbt_long(&mut nbt, "LastPlayed", 0);
    write_nbt_string(&mut nbt, "LevelName", level_name);
    write_nbt_int(&mut nbt, "StorageVersion", 3);
    write_nbt_int(&mut nbt, "WorldType", if infinite { 1 } else { 0 });
    write_nbt_int(&mut nbt, "Dimension", 0);
    write_nbt_int(&mut nbt, "Platform", 2);
    nbt.push(0);

    let dat_path = output_dir.join("level.dat");
    let mut file = File::create(&dat_path)?;
    file.write_all(&3i32.to_le_bytes())?;
    file.write_all(&(nbt.len() as i32).to_le_bytes())?;
    file.write_all(&nbt)?;
    file.sync_all()?;
    Ok(())
}

fn write_nbt_long(buf: &mut Vec<u8>, name: &str, value: i64) {
    buf.push(4);
    buf.extend_from_slice(&(name.len() as u16).to_le_bytes());
    buf.extend_from_slice(name.as_bytes());
    buf.extend_from_slice(&value.to_le_bytes());
}
fn write_nbt_int(buf: &mut Vec<u8>, name: &str, value: i32) {
    buf.push(3);
    buf.extend_from_slice(&(name.len() as u16).to_le_bytes());
    buf.extend_from_slice(name.as_bytes());
    buf.extend_from_slice(&value.to_le_bytes());
}
fn write_nbt_byte(buf: &mut Vec<u8>, name: &str, value: i8) {
    buf.push(1);
    buf.extend_from_slice(&(name.len() as u16).to_le_bytes());
    buf.extend_from_slice(name.as_bytes());
    buf.push(value as u8);
}
fn write_nbt_string(buf: &mut Vec<u8>, name: &str, value: &str) {
    buf.push(8);
    buf.extend_from_slice(&(name.len() as u16).to_le_bytes());
    buf.extend_from_slice(name.as_bytes());
    buf.extend_from_slice(&(value.len() as u16).to_le_bytes());
    buf.extend_from_slice(value.as_bytes());
}