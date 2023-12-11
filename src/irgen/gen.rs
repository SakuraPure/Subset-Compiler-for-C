use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::*;
use crate::irgen::visitor::Rst::{Label, Value};

use super::visitor::{Quadruple, Visitor};

#[derive(Debug)]
pub struct IRGenerator {
    pub quadruples: Vec<Quadruple>,
    label_count: usize,
    ptr_vec: Vec<Rc<RefCell<String>>>,
}

impl IRGenerator {
    /// Creates a new [`IRGenerator`].
    pub fn new() -> IRGenerator {
        IRGenerator {
            quadruples: Vec::new(),
            label_count: 0,
            ptr_vec: vec![],
        }
    }
}

impl Visitor for IRGenerator {
    fn visit_program(&mut self, program: &Program) {
        self.visit_block_statement(&program.main);
    }

    fn visit_block_statement(&mut self, block: &BlockStatement) {
        block
            .statements
            .iter()
            .for_each(|statement| self.visit_statement(statement));
    }

    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Assignment(assignment) => self.visit_assignment_statement(assignment),
            Statement::Conditional(conditional) => self.visit_conditional_statement(conditional),
            Statement::Loop(loop_stmt) => self.visit_loop_statement(loop_stmt),
        }
    }

    fn visit_expression(&mut self, expression: &Expression) -> String {
        match expression {
            Expression::BinaryExpr(binary_expr) => self.visit_add_and_sub_op(binary_expr),
            Expression::Term(term) => self.visit_term(term),
            Expression::ASOP(asop) => self.visit_add_and_sub_op(asop),
        }
    }

    fn visit_factor(&mut self, factor: &Factor) -> String {
        match factor {
            Factor::Identifier(id) => {
                // 直接返回标识符名称
                id.clone()
            }
            Factor::Number(num) => {
                // 返回数字的字符串表示
                num.to_string()
            }
            Factor::Expr(expr) => {
                // 递归处理表达式，并返回结果
                self.visit_expression(expr)
            }
        }
    }

    fn visit_add_and_sub_op(&mut self, op: &AddandSubOp) -> String {
        let left = self.visit_expression(&op.left);
        let right = self.visit_term(&op.right);
        let result = format!("${}", self.quadruples.len());

        let opcode = match op.opcode {
            Opcode::Add => "add",
            Opcode::Sub => "sub",
            Opcode::Mul => "mul",
            Opcode::Div => "div",
        };

        self.quadruples.push(Quadruple::new(
            opcode.to_string(),
            Some(left),
            Some(right),
            Value(result.clone()),
        ));
        result
    }

    fn visit_term(&mut self, term: &Term) -> String {
        match term {
            Term::Factor(factor) => {
                // 直接处理因子
                self.visit_factor(factor)
            }
            Term::Multiply(left_term, right_factor) => {
                let left = self.visit_term(left_term);
                let right = self.visit_factor(right_factor);
                let result = format!("${}", self.quadruples.len());
                self.quadruples.push(Quadruple::new(
                    "mul".to_string(),
                    Some(left),
                    Some(right),
                    Value(result.clone()),
                ));
                result
            }
            Term::Divide(left_term, right_factor) => {
                let left = self.visit_term(left_term);
                let right = self.visit_factor(right_factor);
                let result = format!("${}", self.quadruples.len());
                self.quadruples.push(Quadruple::new(
                    "div".to_string(),
                    Some(left),
                    Some(right),
                    Value(result.clone()),
                ));
                result
            }
        }
    }

    fn visit_assignment_statement(&mut self, assignment: &AssignmentStatement) {
        // 首先处理赋值表达式的右侧
        let expr_result = self.visit_expression(&assignment.expression);

        // 然后生成赋值的四元式
        let op = "asn".to_string();
        let result = assignment.id.clone();

        self.quadruples
            .push(Quadruple::new(op, Some(expr_result), None, Value(result)));
    }

    fn visit_conditional_statement(&mut self, conditional: &ConditionalStatement) {
        self.visit_condition(&conditional.condition);
        self.visit_block_statement(&conditional.block);

        *self.ptr_vec[self.label_count - 1].borrow_mut() = format!("@{}", self.quadruples.len());

        self.label_count -= 2;
        (0..2).for_each(|_| { self.ptr_vec.pop(); });
    }

    fn visit_loop_statement(&mut self, loop_stmt: &LoopStatement) {
        todo!()
    }

    fn visit_condition(&mut self, condition: &Condition) {
        let op = match condition.operator {
            RelationalOperator::Equal => "jeq",
            RelationalOperator::NotEqual => "jne",
            RelationalOperator::LessThan => "jlt",
            RelationalOperator::LessEqual => "jle",
            RelationalOperator::GreaterThan => "jgt",
            RelationalOperator::GreaterEqual => "jge",
        };
        let left = self.visit_expression(&condition.left);
        let right = self.visit_expression(&condition.right);
        let result = Rc::new(RefCell::new(String::new()));

        self.ptr_vec.push(result.clone());

        self.quadruples.push(Quadruple::new(
            op.to_string(),
            Some(left),
            Some(right),
            Label(result.to_owned()),
        ));
        self.label_count += 1;
        let result = Rc::new(RefCell::new(String::new()));

        self.ptr_vec.push(result.clone());

        self.quadruples.push(Quadruple::new(
            "jmp".to_string(),
            None,
            None,
            Label(result.to_owned()),
        ));

        *self.ptr_vec[self.label_count - 1].borrow_mut() = format!("@{}", self.quadruples.len());

        self.label_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 导入需要测试的模块，如 IRGenerator 和 AST 结构

    #[test]
    fn test_visit_assignment_statement() {
        let mut ir_generator = IRGenerator::new();

        // 创建一个测试用的 AssignmentStatement 实例
        let assignment = AssignmentStatement {
            id: "x".to_string(),
            expression: Expression::Term(Term::Factor(Factor::Number(42))), // 假设赋值表达式是 x = 42
        };

        // 调用 visit_assignment_statement 方法
        ir_generator.visit_assignment_statement(&assignment);

        // 检查生成的四元式
        assert_eq!(ir_generator.quadruples.len(), 1);
        let quadruple = &ir_generator.quadruples[0];
        assert_eq!(quadruple.op, "assign");
        assert_eq!(quadruple.arg1, Some("42".to_string()));
        assert_eq!(quadruple.arg2, None);
        assert_eq!(quadruple.result, "x");
    }
}
