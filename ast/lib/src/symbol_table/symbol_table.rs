use std::collections::HashMap;
use super::symbol::Symbol;
use super::symbol_tracker::SymbolTracker;

pub struct SymbolTable {
    pub symbols: HashMap<String, SymbolTracker>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { symbols: HashMap::new() }
    }
    pub fn add(&mut self, symbol: Symbol) -> &Self {
        let name = String::from(&symbol.name);
        self.symbols.entry(name).or_insert(SymbolTracker::new()).add(symbol);
        self
    }
}
