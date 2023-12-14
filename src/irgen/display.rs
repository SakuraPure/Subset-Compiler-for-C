use std::fmt::{Display, Formatter};
use crate::irgen::gen::IRGenerator;
use crate::irgen::visitor::Quadruple;
use crate::irgen::visitor::Rst::{Label, Value};

impl Display for Quadruple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arg1 = match &self.arg1 {
            Some(arg) => arg,
            None => "",
        };
        let arg2 = match &self.arg2 {
            Some(arg) => arg,
            None => "",
        };
        let result = match &self.result {
            Value(val) => val.clone(),
            Label(ptr) => ptr.borrow().clone(),
        };
        write!(f, "{:4} {:4} {:4} {:4}", self.op, arg1, arg2, result)
    }
}

impl Display for IRGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut quadruples = String::new();
        for (index, quad) in self.quadruples.iter().enumerate() {
            quadruples = format!("{}\n{:04} ({})", quadruples, index, quad);
        }
        write!(f, "{}", quadruples)
    }
}