//! The logic of `rua`

use crate::models::Rua;

/// The runner for `rua`.
pub struct RuaRunner<T> {
    rua: T,
}

struct RuaModule {
    path: String,
}

impl<T: Rua> RuaRunner<T> {
    /// Creates a new runner.
    pub fn new(rua: T) -> Self {
        Self { rua }
    }

    /// Runs the runner.
    pub fn run(&mut self) {}
}
