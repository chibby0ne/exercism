// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            // Only revive when health is 0, and set health to 100
            0 => {
                // If the players level is >= 10 then set mana to 100, else set mana to None
                Some(Self {
                    health: 100,
                    mana: if self.level < 10 { None } else { Some(100) },
                    level: self.level,
                })
            }
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(pool) => {
                // If player has enough mana in pool, then substract cost to the mana pool and
                // return the damage inflicted which is 2 * mana_cost
                // Else don't change mana pool but return 0 damage inflicted
                if pool >= mana_cost {
                    self.mana = Some(pool - mana_cost);
                    mana_cost * 2
                } else {
                    0
                }
            }
            None => {
                // When no pool is available make as much damage to health as mana_cost and return 0 as damage inflicted
                // Take care of substract with overflow
                self.health -= min(self.health, mana_cost);
                0
            }
        }
    }
}
