use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use super::VarType;

static SYMBOL_TABLE: OnceLock<Mutex<HashMap<String, VarType>>> = OnceLock::new();

fn get_table() -> &'static Mutex<HashMap<String, VarType>> {
    SYMBOL_TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn insert_symbol(name: String, var_type: VarType) -> Option<VarType> {
    let mut table = get_table().lock().expect("Symbol table lock poisoned");
    table.insert(name, var_type)
}

pub fn get_symbol(name: &str) -> Option<VarType> {
    let table = get_table().lock().expect("Symbol table lock poisoned");
    table.get(name).cloned()
}

pub fn contains_symbol(name: &str) -> bool {
    let table = get_table().lock().expect("Symbol table lock poisoned");
    table.contains_key(name)
}

pub fn remove_symbol(name: &str) -> Option<VarType> {
    let mut table = get_table().lock().expect("Symbol table lock poisoned");
    table.remove(name)
}

pub fn clear_table() {
    let mut table = get_table().lock().expect("Symbol table lock poisoned");
    table.clear();
}
