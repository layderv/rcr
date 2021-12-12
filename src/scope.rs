use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope {
    pub vars: HashMap<String, u8>, // name of var -> register holding that var
    return_regs: Vec<u8>,
    pub used_regs: Vec<u8>,
    bp_offset: i32,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            vars: HashMap::new(),
            return_regs: vec![],
            used_regs: vec![],
            bp_offset: 0,
        }
    }

    pub fn has_var(&self, v: &str) -> bool {
        self.vars.contains_key(v)
    }

    pub fn new_var(&mut self, id: &str, reg: u8) {
        self.vars.insert(id.to_owned(), reg);
    }

    pub fn get_var(&self, v: &str) -> Option<u8> {
        self.vars.get(v).map(|&v| v)
    }

    pub fn get_regs(&self) -> Vec<u8> {
        self.vars.values().cloned().collect()
    }

    pub fn add_return_reg(&mut self, reg: u8) {
        self.return_regs.push(reg);
    }
}
