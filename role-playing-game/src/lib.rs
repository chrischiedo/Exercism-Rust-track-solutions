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
        if self.health == 0 && self.level >= 10 {
            Some(Self {
                health: 100,
                mana: Some(100),
                level: self.level,
            })
        } else if self.health > 0 {
            None
        } else {
            Some(Self { health: 100, mana: None, level: self.level })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana == None {
            self.health = if self.health >= mana_cost {
                self.health - mana_cost
            } else {
                0
            };
            0
        } else if self.mana.unwrap() > mana_cost {
            self.mana = Some(self.mana.unwrap() - mana_cost);
            2 * mana_cost
        } else {
            0
        }
    }
}
