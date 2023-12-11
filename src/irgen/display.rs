use std::fmt::{Display, Formatter};
use crate::irgen::gen::IRGenerator;
use crate::irgen::visitor::Quadruple;

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
        write!(f, "{} {} {} {}", self.op, arg1, arg2, self.result)
    }
}

impl Display for IRGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}