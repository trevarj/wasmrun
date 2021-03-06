use super::parser::Fun;
use super::value::Value;

pub type ModuleIdx = usize;

#[derive(Default, Debug)]
pub struct Store {
    pub funcs: Vec<Func>,
    pub tables: Vec<Vec<Option<u32>>>, // indexed by table address (table_addrs), returns function address (index into Store.funcs)
    pub mems: Vec<Vec<u8>>,            // indexed by module idx
    pub globals: Vec<Global>,
}

#[derive(Debug)]
pub struct Func {
    pub module_idx: ModuleIdx,
    pub fun: Fun,
}

#[derive(Debug)]
pub struct Global {
    pub value: Value,
    pub mutable: bool, // Only needed for validation
}
