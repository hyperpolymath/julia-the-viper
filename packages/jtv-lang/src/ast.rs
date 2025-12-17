// Abstract Syntax Tree for Julia the Viper
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<TopLevel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TopLevel {
    Module(ModuleDecl),
    Import(ImportStmt),
    Function(FunctionDecl),
    Control(ControlStmt),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleDecl {
    pub name: String,
    pub body: Vec<TopLevel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportStmt {
    pub path: Vec<String>,
    pub alias: Option<String>,
}

// ===== CONTROL LANGUAGE (Turing-complete) =====

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControlStmt {
    Assignment(Assignment),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Return(Option<DataExpr>),
    Print(Vec<DataExpr>),
    ReverseBlock(ReverseBlock),
    Block(Vec<ControlStmt>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub target: String,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStmt {
    pub condition: ControlExpr,
    pub then_branch: Vec<ControlStmt>,
    pub else_branch: Option<Vec<ControlStmt>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileStmt {
    pub condition: ControlExpr,
    pub body: Vec<ControlStmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStmt {
    pub variable: String,
    pub range: RangeExpr,
    pub body: Vec<ControlStmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReverseBlock {
    pub body: Vec<ReversibleStmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReversibleStmt {
    AddAssign(String, DataExpr),  // x += expr
    SubAssign(String, DataExpr),  // x -= expr (auto-generated in reverse)
    If(IfStmt),
}

// ===== EXPRESSIONS =====

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Data(DataExpr),
    Control(ControlExpr),
}

// DATA LANGUAGE (Total, addition-only)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataExpr {
    Number(Number),
    Identifier(String),
    Add(Box<DataExpr>, Box<DataExpr>),
    Negate(Box<DataExpr>),
    FunctionCall(FunctionCall),
    List(Vec<DataExpr>),
    Tuple(Vec<DataExpr>),
}

// CONTROL EXPRESSIONS (can have side effects)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControlExpr {
    Data(DataExpr),
    Comparison(Box<DataExpr>, Comparator, Box<DataExpr>),
    Logical(Box<ControlExpr>, LogicalOp, Box<ControlExpr>),
    Not(Box<ControlExpr>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Comparator {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogicalOp {
    And,
    Or,
}

// ===== FUNCTIONS =====

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeAnnotation>,
    pub purity: Purity,
    pub body: Vec<ControlStmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Purity {
    Pure,    // @pure - no loops, no IO
    Total,   // @total - guaranteed to terminate
    Impure,  // default - may loop, may have side effects
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<DataExpr>,
}

// ===== TYPE SYSTEM =====

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeAnnotation {
    Basic(BasicType),
    List(Box<TypeAnnotation>),
    Tuple(Vec<TypeAnnotation>),
    Function(Vec<TypeAnnotation>, Box<TypeAnnotation>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BasicType {
    Int,
    Float,
    Rational,
    Complex,
    Hex,
    Binary,
    Symbolic,
    Bool,
    String,
}

// ===== LITERALS =====

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Number {
    Int(i64),
    Float(f64),
    Rational(i64, i64),  // numerator, denominator
    Complex(f64, f64),   // real, imaginary
    Hex(String),
    Binary(String),
    Symbolic(String),    // For symbolic math (e.g., "x", "pi")
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeExpr {
    pub start: Box<DataExpr>,
    pub end: Box<DataExpr>,
    pub step: Option<Box<DataExpr>>,
}

// ===== VISITOR PATTERN FOR TRAVERSAL =====

pub trait Visitor {
    fn visit_program(&mut self, program: &Program);
    fn visit_control_stmt(&mut self, stmt: &ControlStmt);
    fn visit_data_expr(&mut self, expr: &DataExpr);
    fn visit_function_decl(&mut self, func: &FunctionDecl);
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }

    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_program(self);
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

// Helper functions for AST construction
impl DataExpr {
    pub fn add(left: DataExpr, right: DataExpr) -> Self {
        DataExpr::Add(Box::new(left), Box::new(right))
    }

    pub fn negate(expr: DataExpr) -> Self {
        DataExpr::Negate(Box::new(expr))
    }

    pub fn number(n: Number) -> Self {
        DataExpr::Number(n)
    }

    pub fn identifier(name: impl Into<String>) -> Self {
        DataExpr::Identifier(name.into())
    }
}

impl Number {
    pub fn int(n: i64) -> Self {
        Number::Int(n)
    }

    pub fn float(n: f64) -> Self {
        Number::Float(n)
    }

    pub fn rational(num: i64, den: i64) -> Self {
        Number::Rational(num, den)
    }

    pub fn complex(real: f64, imag: f64) -> Self {
        Number::Complex(real, imag)
    }
}
