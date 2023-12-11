use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::*;

pub trait Visitor {
    fn visit_program(&mut self, program: &Program);
    fn visit_block_statement(&mut self, block: &BlockStatement);
    fn visit_statement(&mut self, statement: &Statement);
    fn visit_expression(&mut self, expression: &Expression) -> String;
    fn visit_factor(&mut self, factor: &Factor) -> String;
    fn visit_add_and_sub_op(&mut self, op: &AddandSubOp) -> String;
    fn visit_term(&mut self, term: &Term) -> String;
    fn visit_assignment_statement(&mut self, assignment: &AssignmentStatement);
    fn visit_conditional_statement(&mut self, conditional: &ConditionalStatement);
    fn visit_loop_statement(&mut self, loop_stmt: &LoopStatement);
    fn visit_condition(&mut self, condition: &Condition);
}

#[derive(Debug)]
pub struct Quadruple {
    pub op: String,
    pub arg1: Option<String>,
    pub arg2: Option<String>,
    pub result: Rst,
}

#[derive(Debug)]
pub enum Rst {
    Value(String),
    Label(Rc<RefCell<String>>),
}

impl Quadruple {
    pub fn new(op: String, arg1: Option<String>, arg2: Option<String>, result: Rst) -> Self {
        Quadruple {
            op,
            arg1,
            arg2,
            result,
        }
    }
}
