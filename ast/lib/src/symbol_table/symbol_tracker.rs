
use super::symbol::{Symbol, SymbolUsage};

pub struct SymbolTracker {
    declared: bool,
    referenced: bool,
    usages: Vec<Symbol>,
}

impl SymbolTracker {
    pub fn new() -> Self {
        Self {
            declared: false,
            referenced: false,
            usages: vec![]
        }
    }

    pub fn add(&mut self, symbol: Symbol) -> &Self {
        match &symbol.usage {
            SymbolUsage::Declaration => {
                if self.declared {
                    // FIXME: bad error message
                    panic!("symbol {} already declared", &symbol.name);
                }
                self.declared = true;
            },
            SymbolUsage::Reference => self.referenced = true,
        }
        self.usages.push(symbol);
        self
    }
}