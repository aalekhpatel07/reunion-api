use pyo3::prelude::*;
use reunion::{UnionFind as UnionFindRust, UnionFindTrait};
use std::collections::{HashMap, HashSet};
use std::fmt;

#[pyclass]
#[pyo3(text_signature = "(/)")]
#[derive(Debug, Clone)]
struct UnionFind {
    _inner: UnionFindRust<u64>,
}

#[pymethods]
impl UnionFind {
    /// Create an instance of UnionFind data structure.
    /// # Example
    /// .. jupyter-execute::
    ///         import pyreunion
    ///         uf = pyreunion.UnionFind()
    ///
    /// ```
    #[new]
    fn new() -> Self {
        UnionFind {
            _inner: UnionFindRust::<u64>::new(),
        }
    }

    /// Find a representative of the group that `entry` belongs to.
    #[pyo3(text_signature = "(self, entry)")]
    fn find(&mut self, node: u64) -> u64 {
        self._inner.find(node)
    }

    /// Union the groups that the two entries: `entryA`, and `entryB` belong to.
    #[pyo3(text_signature = "(self, entryA, entryB)")]
    fn union(&mut self, x: u64, y: u64) {
        self._inner.union(x, y);
    }

    /// Get the collection of non-trivial subsets.
    #[pyo3(text_signature = "(self)")]
    fn subsets(&mut self) -> Vec<HashSet<u64>> {
        self._inner.subsets()
    }

    /// Get the collection of entries in this data structure.
    #[pyo3(text_signature = "(self)")]
    fn entries(&mut self) -> Vec<u64> {
        self._inner.entries()
    }

    /// Get the size of the data structure.
    #[pyo3(text_signature = "(self)")]
    fn size(&self) -> usize {
        self._inner.size()
    }

    /// Get the mapping between entry and its parents.
    #[getter]
    fn parents(&self) -> HashMap<u64, u64> {
        self._inner.parents.clone()
    }

    fn str(&self) -> String {
        format!("{}", self._inner)
    }
}

impl fmt::Display for UnionFind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self._inner, f)
    }
}

#[pymodule]
fn pyreunion(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<UnionFind>()?;
    Ok(())
}
