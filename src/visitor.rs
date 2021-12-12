use crate::{instruction::Opcode, scope::Scope, token::Token};

pub trait Visitor {
    fn visit_token(&mut self, node: &Token);
}

#[derive(Debug, Default)]
pub struct Compiler {
    free_registers: Vec<u8>,
    used_registers: Vec<u8>,
    assembly: Vec<String>,
    scopes: Vec<Scope>,
    scope_idx: usize,
    identifier_buffer: Vec<String>,
    in_return: bool,
}

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) {
        match node {
            &Token::AdditionOperator => self.handle_binary_operator(Opcode::ADD),
            &Token::SubtractionOperator => self.handle_binary_operator(Opcode::SUB),
            &Token::MultiplicationOperator => self.handle_binary_operator(Opcode::MUL),
            &Token::DivisionOperator => self.handle_binary_operator(Opcode::DIV),
            &Token::NegationOperator => self.handle_binary_operator(Opcode::NEG),
            &Token::GreaterThanOperator => self.handle_binary_operator(Opcode::GT),
            &Token::GreaterThanOrEqualToOperator => self.handle_binary_operator(Opcode::GEQ),
            &Token::LessThanOperator => self.handle_binary_operator(Opcode::LT),
            &Token::LessThanOrEqualToOperator => self.handle_binary_operator(Opcode::LEQ),
            &Token::LogicalOrOperator => self.handle_binary_operator(Opcode::OR),
            &Token::LogicalAndOperator => self.handle_binary_operator(Opcode::AND),
            &Token::LogicalNotOperator => self.handle_binary_operator(Opcode::NOT),
            &Token::LogicalEqualOperator => self.handle_binary_operator(Opcode::EQ),
            &Token::Integer { i } => {
                let reg = self.use_free_register().unwrap();
                let asm = format!("{} ${} #{}", Opcode::LOAD, reg, i as u16);
                self.assembly.push(asm);
                self.current_scope().used_regs.push(reg);
                self.used_registers.push(reg);
            }
            &Token::Identifier { ref value } => {
                // TODO check var
                let reg = match self.get_variable(value.as_str()) {
                    Some(reg) => {
                        self.used_registers.retain(|&x| x != reg);
                        self.used_registers.push(reg);
                        reg
                    }
                    None => self.use_free_register().unwrap(),
                };
                self.new_variable(value.as_str(), reg);
            }
            &Token::Assignment {
                ref id,
                ref expression,
            } => {
                self.visit_token(id);
                self.visit_token(expression);
                let mut scope = self.current_scope();
                let id = match id.as_ref() {
                    Token::Identifier { value } => value,
                    _ => panic!("?"),
                };
                let dst = self.get_variable(id).unwrap();
                let src = self.consume_last_used_register().unwrap();
                let asm = format!("{} ${} ${}", Opcode::MOV, dst, src);
                self.assembly.push(asm);
                // keep `dst` on top (better: don't consume src)
                self.used_registers.retain(|&x| x != dst);
                self.used_registers.push(dst);
            }
            &Token::Term { ref elements } => {
                for e in elements {
                    self.visit_token(e);
                }
            }
            &Token::Expression { ref elements } => {
                for e in elements {
                    self.visit_token(e);
                }
            }
            &Token::Program { ref expressions } => {
                self.assembly.push(".data".to_string()); // TODO
                self.assembly.push(".code".to_string());
                for e in expressions {
                    self.visit_token(e)
                }
                self.assembly.push(Opcode::HLT.to_string());
            }
            x => todo!("todo: {:?}", x),
        }
    }
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            free_registers: (0..=31).rev().collect(),
            used_registers: (0..=31).rev().collect(),
            assembly: vec![],
            scopes: vec![Scope::new()],
            scope_idx: 0,
            identifier_buffer: vec![],
            in_return: false,
        }
    }

    fn handle_binary_operator(&mut self, opcode: Opcode) {
        let left = self.consume_last_used_register().unwrap();
        let right = self.consume_last_used_register().unwrap();
        let result = left;
        let asm = format!("{} ${} ${} ${}", opcode.to_string(), left, right, result);
        self.assembly.push(asm);
        self.used_registers.push(result);
        // self.free_registers.push(left);
        self.free_registers.push(right);
    }

    pub fn get_variable(&self, var: &str) -> Option<u8> {
        for scope in self.scopes.iter().rev() {
            if let Some(reg) = scope.get_var(var) {
                return Some(reg);
            }
        }
        None
    }

    pub fn new_variable(&mut self, id: &str, reg: u8) {
        self.scopes.last_mut().map(|scope| scope.new_var(id, reg));
    }

    pub fn new_scope(&mut self) {
        self.scopes.push(Scope::new());
        self.scope_idx += 1;
    }

    pub fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            let regs = scope.get_regs();
            self.free_registers.extend(regs);
            // TODO regs.iter().for_each(|&reg| self.used_registers.)
            self.scope_idx -= 1;
        }
    }

    pub fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    pub fn use_free_register(&mut self) -> Option<u8> {
        self.free_registers.pop()
    }

    pub fn consume_last_used_register(&mut self) -> Option<u8> {
        self.used_registers.pop()
    }
}

mod tests {
    use crate::parser_program::program;

    use super::*;
    #[test]
    fn test() {
        let p = program(
            "
        x=(3+2)*(1+1);
        y=x;
        y = y+ 2;
        ",
        )
        .unwrap();
        // println!("{:#?}", p);
        let mut c = Compiler::new();
        c.visit_token(&p.1);
        println!("{:#?}", c.assembly);
    }
}
