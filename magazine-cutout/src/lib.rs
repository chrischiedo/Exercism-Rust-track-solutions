// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = HashMap::<&str, u32>::new();

    for word in magazine {
        let item = words.entry(word).or_insert(0);
        *item += 1;
    }

    for word in note {
        let item = words.entry(word).or_insert(0);
        if *item == 0 {
            return false;
        }
        *item -= 1;
    }

    true
}
