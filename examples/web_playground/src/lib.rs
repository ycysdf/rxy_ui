use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use wasm_bindgen::prelude::*;
use web_sys::{window, Node, Comment};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// 当 `wee_alloc` 特性被启用时，使用 `wee_alloc` 作为全局分配器。
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let comment1 = window().unwrap().document().unwrap().create_comment("XX");
    let comment2 = window().unwrap().document().unwrap().create_comment("XX");
    let comment33 = window().unwrap().document().unwrap().create_comment("XX");
    let mut hasher = DefaultHasher::new();
    let node: Node = comment1.into();


    // let mut map = HashMap::new();
    // map.insert(node, 1);
    // map.insert(comment2.into(), 2);
    // map.insert(comment33.into(), 3);
    alert(&format!("Hello, wasm-game-of-life! {:?} {:?}", node. as usize,comment2.as_f64()));
}
