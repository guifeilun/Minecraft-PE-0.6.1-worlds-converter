use std::collections::HashMap;

pub struct BlockMap {
    map: HashMap<&'static str, u8>,
}

impl BlockMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        // 0: 空气
        map.insert("minecraft:air", 0);

        // 1: 石头
        map.insert("minecraft:stone", 1);

        // 2: 草方块
        map.insert("minecraft:grass", 2);
        map.insert("minecraft:grass_block", 2);

        // 3: 泥土
        map.insert("minecraft:dirt", 3);

        // 4: 圆石
        map.insert("minecraft:cobblestone", 4);

        // 5: 木板
        map.insert("minecraft:planks", 5);
        map.insert("minecraft:oak_planks", 5);
        map.insert("minecraft:spruce_planks", 5);
        map.insert("minecraft:birch_planks", 5);
        map.insert("minecraft:jungle_planks", 5);
        map.insert("minecraft:acacia_planks", 5);
        map.insert("minecraft:dark_oak_planks", 5);

        // 6: 树苗
        map.insert("minecraft:sapling", 6);

        // 7: 基岩
        map.insert("minecraft:bedrock", 7);

        // 8-11: 水/岩浆
        map.insert("minecraft:flowing_water", 8);
        map.insert("minecraft:water", 9);
        map.insert("minecraft:flowing_lava", 10);
        map.insert("minecraft:lava", 11);

        // 12: 沙子
        map.insert("minecraft:sand", 12);

        // 13: 沙砾
        map.insert("minecraft:gravel", 13);

        // 14-16: 矿石
        map.insert("minecraft:gold_ore", 14);
        map.insert("minecraft:iron_ore", 15);
        map.insert("minecraft:coal_ore", 16);

        // 17: 木头
        map.insert("minecraft:log", 17);
        map.insert("minecraft:oak_log", 17);
        map.insert("minecraft:spruce_log", 17);
        map.insert("minecraft:birch_log", 17);
        map.insert("minecraft:jungle_log", 17);
        map.insert("minecraft:acacia_log", 17);
        map.insert("minecraft:dark_oak_log", 17);
        map.insert("minecraft:log2", 17);

        // 18: 树叶
        map.insert("minecraft:leaves", 18);
        map.insert("minecraft:oak_leaves", 18);
        map.insert("minecraft:spruce_leaves", 18);
        map.insert("minecraft:birch_leaves", 18);
        map.insert("minecraft:jungle_leaves", 18);
        map.insert("minecraft:acacia_leaves", 18);
        map.insert("minecraft:dark_oak_leaves", 18);
        map.insert("minecraft:leaves2", 18);

        // 20: 玻璃
        map.insert("minecraft:glass", 20);

        // 21-22: 青金石
        map.insert("minecraft:lapis_ore", 21);
        map.insert("minecraft:lapis_block", 22);

        // 23: 发射器
        map.insert("minecraft:dispenser", 23);

        // 24: 砂岩
        map.insert("minecraft:sandstone", 24);

        // 25: 音符盒
        map.insert("minecraft:noteblock", 25);
        map.insert("minecraft:note_block", 25);

        // 26: 床
        map.insert("minecraft:bed", 26);

        // 27-28: 铁轨
        map.insert("minecraft:golden_rail", 27);
        map.insert("minecraft:powered_rail", 27);
        map.insert("minecraft:detector_rail", 28);

        // 29: 粘性活塞
        map.insert("minecraft:sticky_piston", 29);

        // 30: 蜘蛛网
        map.insert("minecraft:web", 30);
        map.insert("minecraft:cobweb", 30);

        // 31: 高草
        map.insert("minecraft:tallgrass", 31);
        map.insert("minecraft:grass", 31);

        // 33: 活塞
        map.insert("minecraft:piston", 33);

        // 35: 羊毛
        map.insert("minecraft:wool", 35);
        map.insert("minecraft:white_wool", 35);
        map.insert("minecraft:orange_wool", 35);
        map.insert("minecraft:magenta_wool", 35);
        map.insert("minecraft:light_blue_wool", 35);
        map.insert("minecraft:yellow_wool", 35);
        map.insert("minecraft:lime_wool", 35);
        map.insert("minecraft:pink_wool", 35);
        map.insert("minecraft:gray_wool", 35);
        map.insert("minecraft:silver_wool", 35);
        map.insert("minecraft:light_gray_wool", 35);
        map.insert("minecraft:cyan_wool", 35);
        map.insert("minecraft:purple_wool", 35);
        map.insert("minecraft:blue_wool", 35);
        map.insert("minecraft:brown_wool", 35);
        map.insert("minecraft:green_wool", 35);
        map.insert("minecraft:red_wool", 35);
        map.insert("minecraft:black_wool", 35);

        // 37-40: 花/蘑菇
        map.insert("minecraft:yellow_flower", 37);
        map.insert("minecraft:dandelion", 37);
        map.insert("minecraft:red_flower", 38);
        map.insert("minecraft:poppy", 38);
        map.insert("minecraft:brown_mushroom", 39);
        map.insert("minecraft:red_mushroom", 40);

        // 41-42: 金块/铁块
        map.insert("minecraft:gold_block", 41);
        map.insert("minecraft:iron_block", 42);

        // 43-44: 石台阶
        map.insert("minecraft:double_stone_slab", 43);
        map.insert("minecraft:stone_slab", 44);

        // 45: 砖块
        map.insert("minecraft:brick_block", 45);
        map.insert("minecraft:bricks", 45);

        // 46: TNT
        map.insert("minecraft:tnt", 46);

        // 47: 书架
        map.insert("minecraft:bookshelf", 47);

        // 48: 苔石
        map.insert("minecraft:mossy_cobblestone", 48);

        // 49: 黑曜石
        map.insert("minecraft:obsidian", 49);

        // 50: 火把
        map.insert("minecraft:torch", 50);

        // 51: 火
        map.insert("minecraft:fire", 51);

        // 52: 刷怪笼
        map.insert("minecraft:mob_spawner", 52);
        map.insert("minecraft:spawner", 52);

        // 53: 木楼梯
        map.insert("minecraft:oak_stairs", 53);
        map.insert("minecraft:wooden_stairs", 53);
        map.insert("minecraft:spruce_stairs", 53);
        map.insert("minecraft:birch_stairs", 53);
        map.insert("minecraft:jungle_stairs", 53);
        map.insert("minecraft:acacia_stairs", 53);
        map.insert("minecraft:dark_oak_stairs", 53);

        // 54: 箱子
        map.insert("minecraft:chest", 54);

        // 55: 红石粉
        map.insert("minecraft:redstone_wire", 55);
        map.insert("minecraft:redstone", 55);

        // 56-57: 钻石
        map.insert("minecraft:diamond_ore", 56);
        map.insert("minecraft:diamond_block", 57);

        // 58: 工作台
        map.insert("minecraft:crafting_table", 58);

        // 59: 小麦
        map.insert("minecraft:wheat", 59);

        // 60: 耕地
        map.insert("minecraft:farmland", 60);

        // 61-62: 熔炉
        map.insert("minecraft:furnace", 61);
        map.insert("minecraft:lit_furnace", 62);

        // 63: 告示牌
        map.insert("minecraft:standing_sign", 63);
        map.insert("minecraft:sign", 63);

        // 64: 木门
        map.insert("minecraft:wooden_door", 64);
        map.insert("minecraft:oak_door", 64);

        // 65: 梯子
        map.insert("minecraft:ladder", 65);

        // 66: 铁轨
        map.insert("minecraft:rail", 66);

        // 67: 石楼梯
        map.insert("minecraft:cobblestone_stairs", 67);
        map.insert("minecraft:stone_stairs", 67);

        // 68: 墙上的告示牌
        map.insert("minecraft:wall_sign", 63);

        // 69: 拉杆
        map.insert("minecraft:lever", 69);

        // 70: 石质压力板
        map.insert("minecraft:stone_pressure_plate", 70);

        // 71: 铁门
        map.insert("minecraft:iron_door", 71);

        // 72: 木质压力板
        map.insert("minecraft:wooden_pressure_plate", 72);
        map.insert("minecraft:oak_pressure_plate", 72);

        // 73-74: 红石矿
        map.insert("minecraft:redstone_ore", 73);
        map.insert("minecraft:lit_redstone_ore", 74);

        // 75-76: 红石火把
        map.insert("minecraft:unlit_redstone_torch", 75);
        map.insert("minecraft:redstone_torch", 76);

        // 77: 石按钮
        map.insert("minecraft:stone_button", 77);

        // 78: 顶雪
        map.insert("minecraft:snow_layer", 78);
        map.insert("minecraft:top_snow", 78);

        // 79: 冰
        map.insert("minecraft:ice", 79);

        // 80: 雪块
        map.insert("minecraft:snow", 80);
        map.insert("minecraft:snow_block", 80);

        // 81: 仙人掌
        map.insert("minecraft:cactus", 81);

        // 82: 粘土
        map.insert("minecraft:clay", 82);

        // 83: 甘蔗
        map.insert("minecraft:reeds", 83);
        map.insert("minecraft:sugar_cane", 83);

        // 84: 唱片机
        map.insert("minecraft:jukebox", 84);

        // 85: 栅栏
        map.insert("minecraft:fence", 85);
        map.insert("minecraft:oak_fence", 85);

        // 86: 南瓜
        map.insert("minecraft:pumpkin", 86);

        // 87: 地狱岩
        map.insert("minecraft:netherrack", 87);

        // 88: 灵魂沙
        map.insert("minecraft:soul_sand", 88);

        // 89: 荧石
        map.insert("minecraft:glowstone", 89);

        // 90: 传送门
        map.insert("minecraft:portal", 90);

        // 91: 南瓜灯
        map.insert("minecraft:lit_pumpkin", 91);
        map.insert("minecraft:jack_o_lantern", 91);

        // 92: 蛋糕
        map.insert("minecraft:cake", 92);

        // 93-94: 中继器
        map.insert("minecraft:unpowered_repeater", 93);
        map.insert("minecraft:powered_repeater", 94);
        map.insert("minecraft:repeater", 93);

        // 95: 隐形基岩
        map.insert("minecraft:invisiblebedrock", 95);
        map.insert("minecraft:invisible_bedrock", 95);
        map.insert("minecraft:barrier", 95);

        // 96: 活板门
        map.insert("minecraft:trapdoor", 96);
        map.insert("minecraft:oak_trapdoor", 96);
        map.insert("minecraft:wooden_trapdoor", 96);

        // 97: 怪物蛋
        map.insert("minecraft:monster_egg", 97);

        // 98: 石砖
        map.insert("minecraft:stonebrick", 98);
        map.insert("minecraft:stone_bricks", 98);

        // 99-100: 蘑菇块
        map.insert("minecraft:brown_mushroom_block", 99);
        map.insert("minecraft:red_mushroom_block", 100);

        // 101: 铁栏杆
        map.insert("minecraft:iron_bars", 101);

        // 102: 玻璃板
        map.insert("minecraft:glass_pane", 102);

        // 103: 西瓜
        map.insert("minecraft:melon_block", 103);
        map.insert("minecraft:melon", 103);

        // 104-105: 茎
        map.insert("minecraft:pumpkin_stem", 104);
        map.insert("minecraft:melon_stem", 105);

        // 106: 藤蔓
        map.insert("minecraft:vine", 106);

        // 107: 栅栏门
        map.insert("minecraft:fence_gate", 107);
        map.insert("minecraft:oak_fence_gate", 107);

        // 108-109: 楼梯
        map.insert("minecraft:brick_stairs", 108);
        map.insert("minecraft:stone_brick_stairs", 109);

        // 110: 菌丝
        map.insert("minecraft:mycelium", 110);

        // 111: 睡莲
        map.insert("minecraft:waterlily", 111);
        map.insert("minecraft:lily_pad", 111);

        // 112-114: 地狱砖
        map.insert("minecraft:nether_brick", 112);
        map.insert("minecraft:nether_brick_fence", 113);
        map.insert("minecraft:nether_brick_stairs", 114);

        // 115: 地狱疣
        map.insert("minecraft:nether_wart", 115);

        // 116-118: 附魔台/酿造台/炼药锅
        map.insert("minecraft:enchanting_table", 116);
        map.insert("minecraft:brewing_stand", 117);
        map.insert("minecraft:cauldron", 118);

        // 123-124: 红石灯
        map.insert("minecraft:redstone_lamp", 123);
        map.insert("minecraft:lit_redstone_lamp", 124);

        // 125-126: 木台阶
        map.insert("minecraft:double_wooden_slab", 125);
        map.insert("minecraft:wooden_slab", 126);

        // 127: 可可豆
        map.insert("minecraft:cocoa", 127);

        // 128: 砂岩楼梯
        map.insert("minecraft:sandstone_stairs", 128);

        // 133: 绿宝石块
        map.insert("minecraft:emerald_block", 133);

        // 139: 圆石墙
        map.insert("minecraft:cobblestone_wall", 139);

        // 140: 花盆
        map.insert("minecraft:flower_pot", 140);

        // 141-142: 胡萝卜/土豆
        map.insert("minecraft:carrots", 141);
        map.insert("minecraft:potatoes", 142);

        // 143: 木按钮
        map.insert("minecraft:wooden_button", 143);
        map.insert("minecraft:oak_button", 143);

        // 144: 头颅
        map.insert("minecraft:skull", 144);
        map.insert("minecraft:head", 144);

        // 145: 铁砧
        map.insert("minecraft:anvil", 145);

        // 146: 陷阱箱
        map.insert("minecraft:trapped_chest", 146);

        // 151: 阳光传感器
        map.insert("minecraft:daylight_detector", 151);

        // 152: 红石块
        map.insert("minecraft:redstone_block", 152);

        // 153: 石英矿
        map.insert("minecraft:quartz_ore", 153);
        map.insert("minecraft:nether_quartz_ore", 153);

        // 155: 石英块
        map.insert("minecraft:quartz_block", 155);

        // 156: 石英楼梯
        map.insert("minecraft:quartz_stairs", 156);

        // 245: 切石机
        map.insert("minecraft:stonecutter", 245);
        map.insert("minecraft:stonecutter_block", 245);

        // 246: 发光黑曜石
        map.insert("minecraft:glowingobsidian", 246);

        // 247: 下界反应堆
        map.insert("minecraft:netherreactor", 247);
        map.insert("minecraft:nether_reactor_core", 247);

        // 248: 更新方块 1
        map.insert("minecraft:info_update", 248);
        map.insert("minecraft:info_updateGame1", 248);

        // 249: 更新方块 2
        map.insert("minecraft:info_update2", 249);
        map.insert("minecraft:info_updateGame2", 249);
        map.insert("minecraft:reserved6", 249);

        // 253: 草方块（携带版）
        map.insert("minecraft:grass_carried", 253);

        // 254: 树叶（携带版）
        map.insert("minecraft:leaves_carried", 254);

        // 255: 保留
        map.insert("minecraft:info_reserved6", 255);

        Self { map }
    }

    pub fn get(&self, name: &str) -> u8 {
        *self.map.get(name).unwrap_or(&249)
    }

    pub fn reverse(&self, id: u8) -> Option<&'static str> {
        self.map.iter()
            .find(|(_, &v)| v == id)
            .map(|(&k, _)| k)
    }

    pub fn supports(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl Default for BlockMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_blocks() {
        let m = BlockMap::new();
        assert_eq!(m.get("minecraft:air"), 0);
        assert_eq!(m.get("minecraft:stone"), 1);
        assert_eq!(m.get("minecraft:grass_block"), 2);
        assert_eq!(m.get("minecraft:dirt"), 3);
        assert_eq!(m.get("minecraft:cobblestone"), 4);
        assert_eq!(m.get("minecraft:planks"), 5);
        assert_eq!(m.get("minecraft:bedrock"), 7);
    }

    #[test]
    fn test_unknown_block() {
        let m = BlockMap::new();
        assert_eq!(m.get("minecraft:deepslate"), 249);
        assert_eq!(m.get("minecraft:copper_ore"), 249);
        assert_eq!(m.get("minecraft:cherry_log"), 249);
        assert_eq!(m.get("minecraft:end_stone"), 249);
        assert_eq!(m.get("minecraft:emerald_ore"), 249);
    }

    #[test]
    fn test_barrier() {
        let m = BlockMap::new();
        assert_eq!(m.get("minecraft:barrier"), 95);
    }

    #[test]
    fn test_wool_variants() {
        let m = BlockMap::new();
        assert_eq!(m.get("minecraft:white_wool"), 35);
        assert_eq!(m.get("minecraft:red_wool"), 35);
        assert_eq!(m.get("minecraft:black_wool"), 35);
    }

    #[test]
    fn test_log_variants() {
        let m = BlockMap::new();
        assert_eq!(m.get("minecraft:oak_log"), 17);
        assert_eq!(m.get("minecraft:spruce_log"), 17);
        assert_eq!(m.get("minecraft:birch_log"), 17);
    }

    #[test]
    fn test_reverse() {
        let m = BlockMap::new();
        assert_eq!(m.reverse(1), Some("minecraft:stone"));
        assert_eq!(m.reverse(0), Some("minecraft:air"));
    }

    #[test]
    fn test_supports() {
        let m = BlockMap::new();
        assert!(m.supports("minecraft:stone"));
        assert!(!m.supports("minecraft:deepslate"));
    }
}