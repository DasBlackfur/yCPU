use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol {
    Resolved(u8),
    UnResolved(String, i8),
}

impl Symbol {
    pub fn address(&self) -> u8 {
        match self {
            Symbol::Resolved(addr) => *addr,
            Symbol::UnResolved(name, _) => panic!("Unresolvable symbol {name}"),
        }
    }

    pub fn resolve(&mut self, lookup: &SymbolTable<'_>) -> bool {
        match self {
            Symbol::Resolved(_) => true,
            Symbol::UnResolved(name, offset) => {
                if let Some(addr) = lookup.get(name.as_str()) {
                    *self = Symbol::Resolved(((*addr as i16) + (*offset as i16)) as u8);
                    true
                } else {
                    false
                }
            }
        }
    }
}

pub type SymbolTable<'input> = HashMap<&'input str, u8>;
