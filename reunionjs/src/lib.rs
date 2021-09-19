use reunion::{UnionFind as UnionFindRust, UnionFindTrait};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};


#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
}

// To pass arbitrary data between Rust and JS through serde.
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionFind {
    _inner: UnionFindRust<u64>,
}

#[wasm_bindgen]
impl UnionFind {
    /// Create an instance of UnionFind data structure.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        UnionFind {
            _inner: UnionFindRust::<u64>::new(),
        }
    }

    /// Find a representative of the group that `entry` belongs to.
    pub fn find(&mut self, node: u64) -> u64 {
        self._inner.find(node)
    }

    /// Union the groups that the two entries: `entryA`, and `entryB` belong to.
    pub fn union(&mut self, x: u64, y: u64) {
        self._inner.union(x, y);
    }

    /// Get the collection of non-trivial subsets.
    pub fn subsets(&mut self) -> JsValue { JsValue::from_serde(&self._inner.subsets()).unwrap() }

    /// Get the collection of entries in this data structure.
    pub fn entries(&mut self) -> Vec<u64> { self._inner.entries() }

    /// Get the size of the data structure.
    pub fn size(&self) -> usize {
        self._inner.size()
    }

    /// Get the mapping between entry and its parents.
    pub fn parents(&self) -> JsValue { JsValue::from_serde(&self._inner.parents.clone()).unwrap() }

    /// Get a string representation of the underlying data structure.
    pub fn str(&self) -> String { format!("{}", self._inner) }

    /// Read the data structure from JS.
    pub fn read(val: &JsValue) -> Self { val.into_serde().unwrap() }
}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
