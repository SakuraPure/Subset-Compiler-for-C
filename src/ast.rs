// ast.rs

#[derive(Debug)]
pub struct Program {
    pub main: BlockStatement,
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Assignment(AssignmentStatement),
    Conditional(ConditionalStatement),
    Loop(LoopStatement),
}

#[derive(Debug)]
pub struct AssignmentStatement {
    pub id: String,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct ConditionalStatement {
    pub condition: Condition,
    pub block: BlockStatement,
}

#[derive(Debug)]
pub struct LoopStatement {
    pub condition: Condition,
    pub block: BlockStatement,
}

#[derive(Debug)]
pub enum Expression {
    BinaryExpr(AddandSubOp),
    Term(Term),
    ASOP(AddandSubOp),
}

// (10) <项>→<因子>|<项>*<因子>|<项>/<因子>
#[derive(Debug)]
pub enum Term {
    Factor(Factor),
    Multiply(Box<Term>, Box<Factor>),
    Divide(Box<Term>, Box<Factor>),
}

// (11) <因子>→ID|NUM|(<表达式>)
#[derive(Debug)]
pub enum Factor {
    Identifier(String),
    Number(i32),
    Expr(Box<Expression>),
}

// (8) <条件>→<表达式><关系运算符><表达式>
// pub struct BinaryExpression {
//     pub left: Box<Expression>,
//     pub operator: RelationalOperator,
//     pub right: Box<Expression>,
// }

#[derive(Debug)]
pub struct Condition {
    pub left: Expression,
    pub operator: RelationalOperator,
    pub right: Expression,
}

// <表达式>+<项>|<表达式>-<项>
#[derive(Debug)]
pub struct AddandSubOp {
    pub left: Box<Expression>,
    pub opcode:Opcode,
    pub right:Box<Term>,
}

#[derive(Debug)]
pub enum RelationalOperator {
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Equal,
    NotEqual,
}

#[derive(Debug)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
}
