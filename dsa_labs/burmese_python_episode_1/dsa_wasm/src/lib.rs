use std::collections::LinkedList;
use std::sync::Mutex;
use lazy_static::lazy_static;

// We use a global Mutex so we don't have to deal with 
// complex pointer passing between Python and WASM for now.
lazy_static! {
    static ref LIST: Mutex<LinkedList<i32>> = Mutex::new(LinkedList::new());
}

#[unsafe(no_mangle)]
pub extern "C" fn insert_val(val: i32) {
    let mut list = LIST.lock().unwrap();
    list.push_front(val);
}

#[unsafe(no_mangle)]
pub extern "C" fn remove_front() -> i32 {
    let mut list = LIST.lock().unwrap();
    list.pop_front().unwrap_or(-1) // Return -1 if empty
}


#[unsafe(no_mangle)]
pub extern "C" fn bench_internal(iters: i32) {
    let mut list = std::collections::LinkedList::new();
    // We use a simple loop. Rust's LLVM compiler will 
    // likely optimize this into a masterpiece.
    for i in 0..iters {
        list.push_front(i);
    }
    for _ in 0..iters {
        list.pop_front();
    }
}






// use wasm_bindgen::prelude::*;
// use std::collections::LinkedList;

// #[wasm_bindgen]
// pub struct RustList {
//     inner: LinkedList<i32>,
// }

// #[wasm_bindgen]
// impl RustList {
//     #[wasm_bindgen(constructor)]
//     pub fn new() -> Self {
//         Self { inner: LinkedList::new() }
//     }

//     pub fn insert_val(&mut self, val: i32) {
//         self.inner.push_back(val);
//     }

//     pub fn remove_front(&mut self) -> Option<i32> {
//         self.inner.pop_front()
//     }
    
//     pub fn get_len(&self) -> usize {
//         self.inner.len()
//     }
// }