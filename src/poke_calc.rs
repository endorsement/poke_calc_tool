// use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Ability {
    id: i32,
    name: String,
    description: String,
}

pub enum FieldType {
    None,
    ElectricTerrain,
    GrassyTerrain,
    MistyTerrain,
    PsychicTerrain,
}

pub enum WeatherType {
    None,
    Sun,
    Rain,
    Sand,
    Hail,
    HarshSunshine,
    HeavyRain,
    StrongWinds,
}

pub enum GenderCondition {
    SameGender,
    DifferentGender,
    NoGender
}

pub enum Types{
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
    None
}

impl Default for Types{
    fn default() -> Self {
        Types::None
    }
}

pub enum Nature{
    Hardy,
    Lonely,
    Brave,
    Adamant,
    Naughty,
    Bold,
    Docile,
    Relaxed,
    Impish,
    Lax,
    Timid,
    Hasty,
    Serious,
    Jolly,
    Naive,
    Modest,
    Mild,
    Quiet,
    Bashful,
    Rash,
    Calm,
    Gentle,
    Sassy,
    Careful,
    Quirky
}

impl Default for Nature {
    fn default() -> Self {
        Nature::Hardy
    }
}

pub enum IVSpreads{
    Max,
    MinAtk,
    MinSpd,
    MinAtkSpd
}

#[derive(Default)]
pub struct DmgCalcPokemonData {
    pub id: i32,
    pub forme: i32,
    pub level: i32,

    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub special_attack: i32,
    pub special_defense: i32,
    pub speed: i32,

    pub hp_iv: i32,
    pub attack_iv: i32,
    pub defense_iv: i32,
    pub special_attack_iv: i32,
    pub special_defense_iv: i32,
    pub speed_iv: i32,

    pub hp_ev: i32,
    pub attack_ev: i32,
    pub defense_ev: i32,
    pub special_attack_ev: i32,
    pub special_defense_ev: i32,
    pub speed_ev: i32,

    pub attack_st: i32,
    pub defense_st: i32,
    pub special_attack_st: i32,
    pub special_defense_st: i32,
    pub speed_st: i32,

    pub nature: Nature,
    pub item: i32, // item id
    pub types: Vec<Types>, // type
    pub ability: i32, // ability id
}

impl DmgCalcPokemonData {
    pub fn get_hp(&self) -> i32 {
        let mut hp = self.hp * 2;
        hp += self.hp_iv;
        hp += (self.hp_ev / 4) as i32;
        hp *= self.level;
        hp /= 100;
        hp += 10;
        hp += self.level;
        hp
    }
    pub fn get_atk(&self) -> i32 {
        let mut atk = self.attack * 2;
        atk += self.attack_iv;
        atk += (self.attack_ev / 4) as i32;
        atk *= self.level;
        atk /= 100;
        atk += 5;
        match self.nature {
            Nature::Adamant | Nature::Lonely | Nature::Naughty | Nature::Brave => (atk as f64 * 1.1) as i32,
            Nature::Bold | Nature::Modest | Nature::Calm | Nature::Timid => (atk as f64 * 0.9) as i32,
            _ => (atk as f64 * 1.0) as i32,
        }
    }
    pub fn get_def(&self) -> i32{
        let mut def = self.defense * 2;
        def += self.defense_iv;
        def += (self.defense_ev / 4) as i32;
        def *= self.level;
        def /= 100;
        def += 5;
        match self.nature{
            Nature::Bold | Nature::Impish | Nature::Lax | Nature::Relaxed => (def as f64 * 1.1) as i32,
            Nature::Lonely | Nature::Mild | Nature::Gentle | Nature::Hasty => (def as f64 * 0.9) as i32,
            _ => (def as f64 * 1.0) as i32,
        }
    }
    pub fn get_spatk(&self) -> i32{
        let mut spatk = self.special_attack * 2;
        spatk += self.special_attack_iv;
        spatk += (self.special_attack_ev / 4) as i32;
        spatk *= self.level;
        spatk /= 100;
        spatk += 5;
        match self.nature{
            Nature::Modest | Nature::Mild | Nature::Quiet | Nature::Rash => (spatk as f64 * 1.1) as i32,
            Nature::Adamant | Nature::Impish | Nature::Careful | Nature::Jolly => (spatk as f64 * 0.9) as i32,
            _ => (spatk as f64 * 1.0) as i32,
        }
    }
    pub fn get_spdef(&self) -> i32{
        let mut spdef = self.special_defense * 2;
        spdef += self.special_defense_iv;
        spdef += (self.special_defense_ev / 4) as i32;
        spdef *= self.level;
        spdef /= 100;
        spdef += 5;
        match self.nature{
            Nature::Careful | Nature::Sassy | Nature::Calm | Nature::Gentle => (spdef as f64 * 1.1) as i32,
            Nature::Naughty | Nature::Lax | Nature::Rash | Nature::Naive => (spdef as f64 * 0.9) as i32,
            _ => (spdef as f64 * 1.0) as i32,
        }
    }
    pub fn get_spd(&self) -> i32{
        let mut spd = self.speed * 2;
        spd += self.speed_iv;
        spd += (self.speed_ev / 4) as i32;
        spd *= self.level;
        spd /= 100;
        spd += 5;
        match self.nature{
            Nature::Timid | Nature::Hasty | Nature::Jolly | Nature::Naive => (spd as f64 * 1.1) as i32,
            Nature::Brave | Nature::Relaxed | Nature::Quiet | Nature::Sassy => (spd as f64 * 0.9) as i32,
            _ => (spd as f64 * 1.0) as i32,
        }
    }
    pub fn get_atk_rank(&self) -> (i32, i32) {
        self.get_rank(self.attack_st)
    }
    pub fn get_def_rank(&self) -> (i32, i32) {
        self.get_rank(self.defense_st)
    }
    pub fn get_spatk_rank(&self) -> (i32, i32) {
        self.get_rank(self.special_attack_st)
    }
    pub fn get_spdef_rank(&self) -> (i32, i32) {
        self.get_rank(self.special_defense_st)
    }
    pub fn get_spd_rank(&self) -> (i32, i32) {
        self.get_rank(self.speed_st)
    }

    pub fn set_ivs(mut self, spreads: IVSpreads) {
        match spreads {
            IVSpreads::Max => {
                self.hp_iv = 31;
                self.attack_iv = 31;
                self.defense_iv = 31;
                self.special_attack_iv = 31;
                self.special_defense_iv = 31;
                self.speed_iv = 31;
            }
            IVSpreads::MinAtk => {
                self.hp_iv = 31;
                self.attack_iv = 0;
                self.defense_iv = 31;
                self.special_attack_iv = 31;
                self.special_defense_iv = 31;
                self.speed_iv = 31;
            }
            IVSpreads::MinAtkSpd => {
                self.hp_iv = 31;
                self.attack_iv = 0;
                self.defense_iv = 31;
                self.special_attack_iv = 31;
                self.special_defense_iv = 31;
                self.speed_iv = 0;
            }
            IVSpreads::MinSpd => {
                self.hp_iv = 31;
                self.attack_iv = 31;
                self.defense_iv = 31;
                self.special_attack_iv = 31;
                self.special_defense_iv = 31;
                self.speed_iv = 0;
            }
        }
    }

    fn get_rank(&self, rank_st:i32) -> (i32, i32){
        match rank_st {
            -6 => (2, 8),
            -5 => (2, 7),
            -4 => (2, 6),
            -3 => (2, 5),
            -2 => (2, 4),
            -1 => (2, 3),
            0 => (2, 2),
            1 => (3, 2),
            2 => (4, 2),
            3 => (5, 2),
            4 => (6, 2),
            5 => (7, 2),
            6 => (8, 2),
            _ => (2, 2)
        }

    }
}

pub struct FieldCondition {
    pub field_type: FieldType,
    pub weather_type: WeatherType,
    pub magic_room: bool,
    pub wonder_room: bool,
    pub gravity: bool,
    pub foresight: bool,
    pub reflect: bool,
    pub light_screen: bool,
    pub aurora_veil: bool,
    pub battery: bool,
    pub power_spot:bool,
}

fn pokemon_dt_test() {

    // Zacian (crowned sword)
    let a = DmgCalcPokemonData {
        id: 0,
        forme: 1,

        level: 50,
        hp: 92,
        attack: 170,
        defense: 115,
        special_attack: 80,
        special_defense: 115,
        speed: 148,

        hp_iv: 31,
        attack_iv: 31,
        defense_iv: 31,
        special_attack_iv: 31,
        special_defense_iv: 31,
        speed_iv: 31,

        hp_ev: 236,
        attack_ev: 236,
        defense_ev: 4,
        special_attack_ev: 0,
        special_defense_ev: 4,
        speed_ev: 44,

        attack_st: 1,
        defense_st: 0,
        special_attack_st: 0,
        special_defense_st: 0,
        speed_st: 0,

        nature: Nature::Adamant,
        item: 0,
        types: vec![Types::Fairy, Types::Steel],
        ability: 0
    };

    println!("HP   {:?}", a.get_hp());
    println!("ATK  {:?}", a.get_atk());
    println!("DEF  {:?}", a.get_def());
    println!("SPA  {:?}", a.get_spatk());
    println!("SDF  {:?}", a.get_spdef());
    println!("SPD  {:?}", a.get_spd());
}

fn calc_test() {
    println!("Zekrom self distinct");
    for elm in 0..16 {
        println!("{}, {}", elm, (dmg_calc(50, 200, 197, 140) as f64 *((100 as f64 - elm as f64) / 100 as f64) as f64).floor());
    }
    println!("Zekrom giga impact");
    for elm in 0..16 {
        println!("{}, {}", elm, (dmg_calc(50, 150, 197, 140) as f64 *((100 as f64 - elm as f64) / 100 as f64) as f64).floor());
    }
}

fn dmg_calc(lvat:i32, k_move:i64, k_at:i64, k_df:i64) -> i64 {
    let mut dmg_ = 0_f64;

    dmg_ = (lvat as f64 * 2 as f64 / 5 as f64 + 2 as f64).floor();
    dmg_ = (dmg_ * k_move as f64 * k_at as f64 / k_df as f64).floor();
    (dmg_ / 50 as f64 + 2 as f64).floor() as i64
}

fn k_move_ability(atk_ability_id: i32, def_ability_id: i32, ability_enable: bool) -> i32 {
    if ability_enable {
        match atk_ability_id {
            79 => 5448,
            174 => 4915,
            182 => 4915,
            184 => 4915,
            186 => if (def_ability_id == 188) {3072} else {5448}, // Dark-Aura
            187 => if (def_ability_id == 188) {3072} else {5448}, // Fairy-Aura
            _ => 4096
        }
    } else{
        4096
    }
}

fn k_move_calc(ability_magnification: i32, move_dmg: i64) -> i64 {
    let mut k_move = 0_f64;
    0
}
