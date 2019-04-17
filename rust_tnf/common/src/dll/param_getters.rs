use crate::{
    defines::param::{CritterParam, Param as PAR},
    engine_types::{game_options::game_state, mutual::CritterMutual},
    primitives::*,
};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn getParam_Strength(cr: &CritterMutual, _: uint) -> int {
    let mut val: int = cr.param(PAR::ST_STRENGTH) + cr.param(PAR::ST_STRENGTH_EXT);
    if cr.param( PAR::PE_ADRENALINE_RUSH ) > 0 && getParam_Timeout( cr, PAR::TO_BATTLE as uint ) > 0 // Adrenaline rush perk
        && cr.param( PAR::ST_CURRENT_HP ) <= (
                cr.param( PAR::ST_MAX_LIFE ) +
                cr.param( PAR::ST_STRENGTH ) +
                cr.param( PAR::ST_ENDURANCE ) * 2
            ) / 2
    {
        val += 1;
    }
    clamp(val, 1, 10)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn getParam_Timeout(cr: &CritterMutual, index: uint) -> int {
    let full_second = game_state().map(|g| g.FullSecond).unwrap_or(0);
    let param = cr.Params[index as usize] as uint;
    if param > full_second {
        (param - full_second) as int
    } else {
        0
    }
}

fn clamp<T: std::cmp::Ord>(val: T, min: T, max: T) -> T
where
    T: Sized,
{
    assert!(min <= max);
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
