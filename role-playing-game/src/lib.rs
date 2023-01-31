// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::cmp::{max};

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // unimplemented!("Revive this player")
        match self{
            x if x.health == 0 => Some(Player{health: 100
                                         , mana: if x.level < 10 {None} else {Some(100)}
                                         , level: x.level}),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // unimplemented!("Cast a spell of cost {}", mana_cost)
        let mut damage:u32 = 0;
        match self.mana{
            Some(mana) => {
                match mana{
                    x if x < mana_cost => damage = 0,
                    _ => {self.mana = Some(mana - mana_cost); damage = mana_cost * 2}
                }
            },
            None => {
                self.health = max(0, self.health as i32 - mana_cost as i32) as u32;
                damage = 0
            }
        }
        damage
    }
}
