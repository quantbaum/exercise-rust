// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;
use std::borrow::BorrowMut;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut counter_map: HashMap<&str, i32> = HashMap::new();

    for word in magazine{
        let count: &mut i32 = counter_map.entry(word).or_insert(0);
        *count += 1;
    }
    let mut can_construct: bool = true;
    for word in note{
        let count = counter_map.get(word);
        match count {
            Some(x) => {
                match x {
                    0 => {
                        counter_map.insert(word, 0);
                        can_construct = false;
                        break;
                    },
                    _ => counter_map.insert(word, x - 1)
                }
            },
            _ => {can_construct = false;
                break;
            }
        };
    }

    can_construct
}
