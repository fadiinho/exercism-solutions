// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let player_mana = match self.level {
            0..=9 => None,
            _ => Some(100),
        };

        if self.health == 0 {
            return Some(Player { health: 100, mana: player_mana, level: self.level })
        }

        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.level < 10 {
            if mana_cost > self.health {
                self.health = 0;
                0
            } else {
                self.health -= mana_cost;
                0
            }
        } else if self.mana < Some(mana_cost) {
            0
        } else {
            let new_mana = self.mana.unwrap() - mana_cost;
            self.mana = Some(new_mana);
            mana_cost * 2
        }
    }
}
