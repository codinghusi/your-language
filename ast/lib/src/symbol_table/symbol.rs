pub enum SymbolUsage {
    Declaration,
    Reference,
}

pub struct Symbol {
    pub name: String,
    pub usage: SymbolUsage,
    pub loc: std::ops::Range<usize>,
}
