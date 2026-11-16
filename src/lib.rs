
use smash::{
    app::{sv_animcmd, sv_math, sv_system, utility},
    hash40,
    lib::{lua_const::BATTLE_OBJECT_CATEGORY_FIGHTER, L2CAgent, L2CValue},
};

// Non-inclusive upper bounds matching FaultyPine's original array
const HITBOX_ARG_RANGES: [i32; 36] = [
    0,   // 0: ID
    0,   // 1: Part
    0,   // 2: Bone
    21,  // 3: Damage (0..20)
    361, // 4: Angle (0..360)
    101, // 5: KBG (0..100)
    0,   // 6: FKB
    81,  // 7: BKB (0..80)
    16,  // 8: Size (0..15)
    0, 0, 0, 0, 0, 0, // 9..14: X, Y, Z, X2, Y2, Z2
    5,   // 15: Hitlag (0..4)
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let is_fighter = utility::get_category(boma) == BATTLE_OBJECT_CATEGORY_FIGHTER;

    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    l2c_agent.clear_lua_stack();

    for i in 0..36 {
        let range = HITBOX_ARG_RANGES[i];
        if is_fighter && range > 0 {
            let rand_val = sv_math::rand(hash40("agent"), range);
            // Damage (3), Size (8), and Hitlag (15) are floats; Angle (4), KBG (5), and BKB (7) are ints
            let mut val = match i {
                3 | 8 | 15 => L2CValue::new_num(rand_val as f32),
                _ => L2CValue::new_int(rand_val as u64),
            };
            l2c_agent.push_lua_stack(&mut val);
        } else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
        }
    }

    original!()(lua_state);
}

#[skyline::main(name = "random_attack_values")]
pub fn main() {
    skyline::install_hooks!(attack_replace);
}

