use dioxus::prelude::*;

use crate::Item;

pub fn get_index(list_signal: Signal<Vec<Item>>, id: i64) -> i64 {
    let mut index: i64 = -1;
    let tid = id;
    let mut item = 0;
    while item < list_signal.read().len() as i64 {
        if list_signal.read()[item as usize].id == tid {
            index = item;
            println!("debug")
        }
        item += 1;
    }
    index
}
