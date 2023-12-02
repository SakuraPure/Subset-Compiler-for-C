// ast.rs

pub struct Program {
    pub main: BlockStatement,
}

pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Assignment(AssignmentStatement),
    Conditional(ConditionalStatement),
    Loop(LoopStatement),
}

pub struct AssignmentStatement {
    pub id: String,
    pub expression: Expression,
}

pub struct ConditionalStatement {
    pub condition: Expression,
    pub block: BlockStatement,
}

pub struct LoopStatement {
    pub condition: Expression,
    pub block: BlockStatement,
}

pub enum Expression {
    BinaryExpr(BinaryExpression),
    Term(Term),
    ASOP(AddandSubOp),
}

// (10) <项>→<因子>|<项>*<因子>|<项>/<因子>
pub enum Term {
    Factor(Factor),
    Multiply(Box<Term>, Box<Factor>),
    Divide(Box<Term>, Box<Factor>),
}

// (11) <因子>→ID|NUM|(<表达式>)
pub enum Factor {
    Identifier(String),
    Number(i64),
    Expr(Box<Expression>),
}

// (8) <条件>→<表达式><关系运算符><表达式>
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: RelationalOperator,
    pub right: Box<Expression>,
}

// <表达式>+<项>|<表达式>-<项>
pub struct AddandSubOp {
    pub left: Box<Expression>,
    pub opcode:Opcode,
    pub right:Box<Expression>,
}

pub enum RelationalOperator {
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Equal,
    NotEqual,
}

pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
}
