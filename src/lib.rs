#![no_std]
#![allow(static_mut_refs)]
use gstd::{exec, msg};
use pebbles_game_io::*;

static mut GAME_STATE: Option<GameState> = None;

#[cfg(test)]
pub fn get_random_u32() -> u32 {
    2
}

#[cfg(not(test))]
pub fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}



