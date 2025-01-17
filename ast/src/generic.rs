#![allow(clippy::derive_partial_eq_without_eq)]
use crate::text_size::TextRange;
pub use crate::{builtin::*, text_size::TextSize, ConversionFlag, Node};
use std::fmt::Debug;

// This file was originally generated from asdl by a python script, but we now edit it manually

#[derive(Clone, Debug, PartialEq, is_macro::Is)]
pub enum Ast {
    #[is(name = "module")]
    Mod(Mod),
    Stmt(Stmt),
    Expr(Expr),
    ExprContext(ExprContext),
    BoolOp(BoolOp),
    Operator(Operator),
    UnaryOp(UnaryOp),
    CmpOp(CmpOp),
    Comprehension(Comprehension),
    ExceptHandler(ExceptHandler),
    Arguments(Arguments),
    Arg(Arg),
    Keyword(Keyword),
    Alias(Alias),
    WithItem(WithItem),
    MatchCase(MatchCase),
    Pattern(Pattern),
    TypeIgnore(TypeIgnore),
    Decorator(Decorator),
}
impl Node for Ast {
    const NAME: &'static str = "AST";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

impl From<Mod> for Ast {
    fn from(node: Mod) -> Self {
        Ast::Mod(node)
    }
}

impl From<Stmt> for Ast {
    fn from(node: Stmt) -> Self {
        Ast::Stmt(node)
    }
}

impl From<Expr> for Ast {
    fn from(node: Expr) -> Self {
        Ast::Expr(node)
    }
}

impl From<ExprContext> for Ast {
    fn from(node: ExprContext) -> Self {
        Ast::ExprContext(node)
    }
}

impl From<BoolOp> for Ast {
    fn from(node: BoolOp) -> Self {
        Ast::BoolOp(node)
    }
}

impl From<Operator> for Ast {
    fn from(node: Operator) -> Self {
        Ast::Operator(node)
    }
}

impl From<UnaryOp> for Ast {
    fn from(node: UnaryOp) -> Self {
        Ast::UnaryOp(node)
    }
}

impl From<CmpOp> for Ast {
    fn from(node: CmpOp) -> Self {
        Ast::CmpOp(node)
    }
}

impl From<Comprehension> for Ast {
    fn from(node: Comprehension) -> Self {
        Ast::Comprehension(node)
    }
}

impl From<ExceptHandler> for Ast {
    fn from(node: ExceptHandler) -> Self {
        Ast::ExceptHandler(node)
    }
}

impl From<Arguments> for Ast {
    fn from(node: Arguments) -> Self {
        Ast::Arguments(node)
    }
}

impl From<Arg> for Ast {
    fn from(node: Arg) -> Self {
        Ast::Arg(node)
    }
}

impl From<Keyword> for Ast {
    fn from(node: Keyword) -> Self {
        Ast::Keyword(node)
    }
}

impl From<Alias> for Ast {
    fn from(node: Alias) -> Self {
        Ast::Alias(node)
    }
}

impl From<WithItem> for Ast {
    fn from(node: WithItem) -> Self {
        Ast::WithItem(node)
    }
}

impl From<MatchCase> for Ast {
    fn from(node: MatchCase) -> Self {
        Ast::MatchCase(node)
    }
}

impl From<Pattern> for Ast {
    fn from(node: Pattern) -> Self {
        Ast::Pattern(node)
    }
}

impl From<TypeIgnore> for Ast {
    fn from(node: TypeIgnore) -> Self {
        Ast::TypeIgnore(node)
    }
}

impl From<Decorator> for Ast {
    fn from(node: Decorator) -> Self {
        Ast::Decorator(node)
    }
}

/// See also [mod](https://docs.python.org/3/library/ast.html#ast.mod)
#[derive(Clone, Debug, PartialEq, is_macro::Is)]
pub enum Mod {
    Module(ModModule),
    Interactive(ModInteractive),
    Expression(ModExpression),
    FunctionType(ModFunctionType),
}

/// See also [Module](https://docs.python.org/3/library/ast.html#ast.Module)
#[derive(Clone, Debug, PartialEq)]
pub struct ModModule {
    pub range: TextRange,
    pub body: Vec<Stmt>,
    pub type_ignores: Vec<TypeIgnore>,
}

impl Node for ModModule {
    const NAME: &'static str = "Module";
    const FIELD_NAMES: &'static [&'static str] = &["body", "type_ignores"];
}
impl From<ModModule> for Mod {
    fn from(payload: ModModule) -> Self {
        Mod::Module(payload)
    }
}
impl From<ModModule> for Ast {
    fn from(payload: ModModule) -> Self {
        Mod::from(payload).into()
    }
}

/// See also [Interactive](https://docs.python.org/3/library/ast.html#ast.Interactive)
#[derive(Clone, Debug, PartialEq)]
pub struct ModInteractive {
    pub range: TextRange,
    pub body: Vec<Stmt>,
}

impl Node for ModInteractive {
    const NAME: &'static str = "Interactive";
    const FIELD_NAMES: &'static [&'static str] = &["body"];
}
impl From<ModInteractive> for Mod {
    fn from(payload: ModInteractive) -> Self {
        Mod::Interactive(payload)
    }
}
impl From<ModInteractive> for Ast {
    fn from(payload: ModInteractive) -> Self {
        Mod::from(payload).into()
    }
}

/// See also [Expression](https://docs.python.org/3/library/ast.html#ast.Expression)
#[derive(Clone, Debug, PartialEq)]
pub struct ModExpression {
    pub range: TextRange,
    pub body: Box<Expr>,
}

impl Node for ModExpression {
    const NAME: &'static str = "Expression";
    const FIELD_NAMES: &'static [&'static str] = &["body"];
}
impl From<ModExpression> for Mod {
    fn from(payload: ModExpression) -> Self {
        Mod::Expression(payload)
    }
}
impl From<ModExpression> for Ast {
    fn from(payload: ModExpression) -> Self {
        Mod::from(payload).into()
    }
}

/// See also [FunctionType](https://docs.python.org/3/library/ast.html#ast.FunctionType)
#[derive(Clone, Debug, PartialEq)]
pub struct ModFunctionType {
    pub range: TextRange,
    pub argtypes: Vec<Expr>,
    pub returns: Box<Expr>,
}

impl Node for ModFunctionType {
    const NAME: &'static str = "FunctionType";
    const FIELD_NAMES: &'static [&'static str] = &["argtypes", "returns"];
}
impl From<ModFunctionType> for Mod {
    fn from(payload: ModFunctionType) -> Self {
        Mod::FunctionType(payload)
    }
}
impl From<ModFunctionType> for Ast {
    fn from(payload: ModFunctionType) -> Self {
        Mod::from(payload).into()
    }
}

impl Node for Mod {
    const NAME: &'static str = "mod";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

/// See also [stmt](https://docs.python.org/3/library/ast.html#ast.stmt)
#[derive(Clone, Debug, PartialEq, is_macro::Is)]
pub enum Stmt {
    #[is(name = "function_def_stmt")]
    FunctionDef(StmtFunctionDef),
    #[is(name = "async_function_def_stmt")]
    AsyncFunctionDef(StmtAsyncFunctionDef),
    #[is(name = "class_def_stmt")]
    ClassDef(StmtClassDef),
    #[is(name = "return_stmt")]
    Return(StmtReturn),
    #[is(name = "delete_stmt")]
    Delete(StmtDelete),
    #[is(name = "assign_stmt")]
    Assign(StmtAssign),
    #[is(name = "aug_assign_stmt")]
    AugAssign(StmtAugAssign),
    #[is(name = "ann_assign_stmt")]
    AnnAssign(StmtAnnAssign),
    #[is(name = "for_stmt")]
    For(StmtFor),
    #[is(name = "async_for_stmt")]
    AsyncFor(StmtAsyncFor),
    #[is(name = "while_stmt")]
    While(StmtWhile),
    #[is(name = "if_stmt")]
    If(StmtIf),
    #[is(name = "with_stmt")]
    With(StmtWith),
    #[is(name = "async_with_stmt")]
    AsyncWith(StmtAsyncWith),
    #[is(name = "match_stmt")]
    Match(StmtMatch),
    #[is(name = "raise_stmt")]
    Raise(StmtRaise),
    #[is(name = "try_stmt")]
    Try(StmtTry),
    #[is(name = "try_star_stmt")]
    TryStar(StmtTryStar),
    #[is(name = "assert_stmt")]
    Assert(StmtAssert),
    #[is(name = "import_stmt")]
    Import(StmtImport),
    #[is(name = "import_from_stmt")]
    ImportFrom(StmtImportFrom),
    #[is(name = "global_stmt")]
    Global(StmtGlobal),
    #[is(name = "nonlocal_stmt")]
    Nonlocal(StmtNonlocal),
    #[is(name = "expr_stmt")]
    Expr(StmtExpr),
    #[is(name = "pass_stmt")]
    Pass(StmtPass),
    #[is(name = "break_stmt")]
    Break(StmtBreak),
    #[is(name = "continue_stmt")]
    Continue(StmtContinue),
}

/// See also [FunctionDef](https://docs.python.org/3/library/ast.html#ast.FunctionDef)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtFunctionDef {
    pub range: TextRange,
    pub name: Identifier,
    pub args: Box<Arguments>,
    pub body: Vec<Stmt>,
    pub decorator_list: Vec<Decorator>,
    pub returns: Option<Box<Expr>>,
    pub type_comment: Option<String>,
}

impl Node for StmtFunctionDef {
    const NAME: &'static str = "FunctionDef";
    const FIELD_NAMES: &'static [&'static str] = &[
        "name",
        "args",
        "body",
        "decorator_list",
        "returns",
        "type_comment",
    ];
}
impl From<StmtFunctionDef> for Stmt {
    fn from(payload: StmtFunctionDef) -> Self {
        Stmt::FunctionDef(payload)
    }
}
impl From<StmtFunctionDef> for Ast {
    fn from(payload: StmtFunctionDef) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [AsyncFunctionDef](https://docs.python.org/3/library/ast.html#ast.AsyncFunctionDef)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtAsyncFunctionDef {
    pub range: TextRange,
    pub name: Identifier,
    pub args: Box<Arguments>,
    pub body: Vec<Stmt>,
    pub decorator_list: Vec<Decorator>,
    pub returns: Option<Box<Expr>>,
    pub type_comment: Option<String>,
}

impl Node for StmtAsyncFunctionDef {
    const NAME: &'static str = "AsyncFunctionDef";
    const FIELD_NAMES: &'static [&'static str] = &[
        "name",
        "args",
        "body",
        "decorator_list",
        "returns",
        "type_comment",
    ];
}
impl From<StmtAsyncFunctionDef> for Stmt {
    fn from(payload: StmtAsyncFunctionDef) -> Self {
        Stmt::AsyncFunctionDef(payload)
    }
}
impl From<StmtAsyncFunctionDef> for Ast {
    fn from(payload: StmtAsyncFunctionDef) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [ClassDef](https://docs.python.org/3/library/ast.html#ast.ClassDef)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtClassDef {
    pub range: TextRange,
    pub name: Identifier,
    pub bases: Vec<Expr>,
    pub keywords: Vec<Keyword>,
    pub body: Vec<Stmt>,
    pub decorator_list: Vec<Decorator>,
}

impl Node for StmtClassDef {
    const NAME: &'static str = "ClassDef";
    const FIELD_NAMES: &'static [&'static str] =
        &["name", "bases", "keywords", "body", "decorator_list"];
}
impl From<StmtClassDef> for Stmt {
    fn from(payload: StmtClassDef) -> Self {
        Stmt::ClassDef(payload)
    }
}
impl From<StmtClassDef> for Ast {
    fn from(payload: StmtClassDef) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Return](https://docs.python.org/3/library/ast.html#ast.Return)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtReturn {
    pub range: TextRange,
    pub value: Option<Box<Expr>>,
}

impl Node for StmtReturn {
    const NAME: &'static str = "Return";
    const FIELD_NAMES: &'static [&'static str] = &["value"];
}
impl From<StmtReturn> for Stmt {
    fn from(payload: StmtReturn) -> Self {
        Stmt::Return(payload)
    }
}
impl From<StmtReturn> for Ast {
    fn from(payload: StmtReturn) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Delete](https://docs.python.org/3/library/ast.html#ast.Delete)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtDelete {
    pub range: TextRange,
    pub targets: Vec<Expr>,
}

impl Node for StmtDelete {
    const NAME: &'static str = "Delete";
    const FIELD_NAMES: &'static [&'static str] = &["targets"];
}
impl From<StmtDelete> for Stmt {
    fn from(payload: StmtDelete) -> Self {
        Stmt::Delete(payload)
    }
}
impl From<StmtDelete> for Ast {
    fn from(payload: StmtDelete) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Assign](https://docs.python.org/3/library/ast.html#ast.Assign)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtAssign {
    pub range: TextRange,
    pub targets: Vec<Expr>,
    pub value: Box<Expr>,
    pub type_comment: Option<String>,
}

impl Node for StmtAssign {
    const NAME: &'static str = "Assign";
    const FIELD_NAMES: &'static [&'static str] = &["targets", "value", "type_comment"];
}
impl From<StmtAssign> for Stmt {
    fn from(payload: StmtAssign) -> Self {
        Stmt::Assign(payload)
    }
}
impl From<StmtAssign> for Ast {
    fn from(payload: StmtAssign) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [AugAssign](https://docs.python.org/3/library/ast.html#ast.AugAssign)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtAugAssign {
    pub range: TextRange,
    pub target: Box<Expr>,
    pub op: Operator,
    pub value: Box<Expr>,
}

impl Node for StmtAugAssign {
    const NAME: &'static str = "AugAssign";
    const FIELD_NAMES: &'static [&'static str] = &["target", "op", "value"];
}
impl From<StmtAugAssign> for Stmt {
    fn from(payload: StmtAugAssign) -> Self {
        Stmt::AugAssign(payload)
    }
}
impl From<StmtAugAssign> for Ast {
    fn from(payload: StmtAugAssign) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [AnnAssign](https://docs.python.org/3/library/ast.html#ast.AnnAssign)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtAnnAssign {
    pub range: TextRange,
    pub target: Box<Expr>,
    pub annotation: Box<Expr>,
    pub value: Option<Box<Expr>>,
    pub simple: bool,
}

impl Node for StmtAnnAssign {
    const NAME: &'static str = "AnnAssign";
    const FIELD_NAMES: &'static [&'static str] = &["target", "annotation", "value", "simple"];
}
impl From<StmtAnnAssign> for Stmt {
    fn from(payload: StmtAnnAssign) -> Self {
        Stmt::AnnAssign(payload)
    }
}
impl From<StmtAnnAssign> for Ast {
    fn from(payload: StmtAnnAssign) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [For](https://docs.python.org/3/library/ast.html#ast.For)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtFor {
    pub range: TextRange,
    pub target: Box<Expr>,
    pub iter: Box<Expr>,
    pub body: Vec<Stmt>,
    pub orelse: Vec<Stmt>,
    pub type_comment: Option<String>,
}

impl Node for StmtFor {
    const NAME: &'static str = "For";
    const FIELD_NAMES: &'static [&'static str] =
        &["target", "iter", "body", "orelse", "type_comment"];
}
impl From<StmtFor> for Stmt {
    fn from(payload: StmtFor) -> Self {
        Stmt::For(payload)
    }
}
impl From<StmtFor> for Ast {
    fn from(payload: StmtFor) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [AsyncFor](https://docs.python.org/3/library/ast.html#ast.AsyncFor)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtAsyncFor {
    pub range: TextRange,
    pub target: Box<Expr>,
    pub iter: Box<Expr>,
    pub body: Vec<Stmt>,
    pub orelse: Vec<Stmt>,
    pub type_comment: Option<String>,
}

impl Node for StmtAsyncFor {
    const NAME: &'static str = "AsyncFor";
    const FIELD_NAMES: &'static [&'static str] =
        &["target", "iter", "body", "orelse", "type_comment"];
}
impl From<StmtAsyncFor> for Stmt {
    fn from(payload: StmtAsyncFor) -> Self {
        Stmt::AsyncFor(payload)
    }
}
impl From<StmtAsyncFor> for Ast {
    fn from(payload: StmtAsyncFor) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [While](https://docs.python.org/3/library/ast.html#ast.While)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtWhile {
    pub range: TextRange,
    pub test: Box<Expr>,
    pub body: Vec<Stmt>,
    pub orelse: Vec<Stmt>,
}

impl Node for StmtWhile {
    const NAME: &'static str = "While";
    const FIELD_NAMES: &'static [&'static str] = &["test", "body", "orelse"];
}
impl From<StmtWhile> for Stmt {
    fn from(payload: StmtWhile) -> Self {
        Stmt::While(payload)
    }
}
impl From<StmtWhile> for Ast {
    fn from(payload: StmtWhile) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [If](https://docs.python.org/3/library/ast.html#ast.If)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtIf {
    pub range: TextRange,
    pub test: Box<Expr>,
    pub body: Vec<Stmt>,
    pub orelse: Vec<Stmt>,
}

impl Node for StmtIf {
    const NAME: &'static str = "If";
    const FIELD_NAMES: &'static [&'static str] = &["test", "body", "orelse"];
}
impl From<StmtIf> for Stmt {
    fn from(payload: StmtIf) -> Self {
        Stmt::If(payload)
    }
}
impl From<StmtIf> for Ast {
    fn from(payload: StmtIf) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [With](https://docs.python.org/3/library/ast.html#ast.With)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtWith {
    pub range: TextRange,
    pub items: Vec<WithItem>,
    pub body: Vec<Stmt>,
    pub type_comment: Option<String>,
}

impl Node for StmtWith {
    const NAME: &'static str = "With";
    const FIELD_NAMES: &'static [&'static str] = &["items", "body", "type_comment"];
}
impl From<StmtWith> for Stmt {
    fn from(payload: StmtWith) -> Self {
        Stmt::With(payload)
    }
}
impl From<StmtWith> for Ast {
    fn from(payload: StmtWith) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [AsyncWith](https://docs.python.org/3/library/ast.html#ast.AsyncWith)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtAsyncWith {
    pub range: TextRange,
    pub items: Vec<WithItem>,
    pub body: Vec<Stmt>,
    pub type_comment: Option<String>,
}

impl Node for StmtAsyncWith {
    const NAME: &'static str = "AsyncWith";
    const FIELD_NAMES: &'static [&'static str] = &["items", "body", "type_comment"];
}
impl From<StmtAsyncWith> for Stmt {
    fn from(payload: StmtAsyncWith) -> Self {
        Stmt::AsyncWith(payload)
    }
}
impl From<StmtAsyncWith> for Ast {
    fn from(payload: StmtAsyncWith) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Match](https://docs.python.org/3/library/ast.html#ast.Match)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtMatch {
    pub range: TextRange,
    pub subject: Box<Expr>,
    pub cases: Vec<MatchCase>,
}

impl Node for StmtMatch {
    const NAME: &'static str = "Match";
    const FIELD_NAMES: &'static [&'static str] = &["subject", "cases"];
}
impl From<StmtMatch> for Stmt {
    fn from(payload: StmtMatch) -> Self {
        Stmt::Match(payload)
    }
}
impl From<StmtMatch> for Ast {
    fn from(payload: StmtMatch) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Raise](https://docs.python.org/3/library/ast.html#ast.Raise)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtRaise {
    pub range: TextRange,
    pub exc: Option<Box<Expr>>,
    pub cause: Option<Box<Expr>>,
}

impl Node for StmtRaise {
    const NAME: &'static str = "Raise";
    const FIELD_NAMES: &'static [&'static str] = &["exc", "cause"];
}
impl From<StmtRaise> for Stmt {
    fn from(payload: StmtRaise) -> Self {
        Stmt::Raise(payload)
    }
}
impl From<StmtRaise> for Ast {
    fn from(payload: StmtRaise) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Try](https://docs.python.org/3/library/ast.html#ast.Try)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtTry {
    pub range: TextRange,
    pub body: Vec<Stmt>,
    pub handlers: Vec<ExceptHandler>,
    pub orelse: Vec<Stmt>,
    pub finalbody: Vec<Stmt>,
}

impl Node for StmtTry {
    const NAME: &'static str = "Try";
    const FIELD_NAMES: &'static [&'static str] = &["body", "handlers", "orelse", "finalbody"];
}
impl From<StmtTry> for Stmt {
    fn from(payload: StmtTry) -> Self {
        Stmt::Try(payload)
    }
}
impl From<StmtTry> for Ast {
    fn from(payload: StmtTry) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [TryStar](https://docs.python.org/3/library/ast.html#ast.TryStar)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtTryStar {
    pub range: TextRange,
    pub body: Vec<Stmt>,
    pub handlers: Vec<ExceptHandler>,
    pub orelse: Vec<Stmt>,
    pub finalbody: Vec<Stmt>,
}

impl Node for StmtTryStar {
    const NAME: &'static str = "TryStar";
    const FIELD_NAMES: &'static [&'static str] = &["body", "handlers", "orelse", "finalbody"];
}
impl From<StmtTryStar> for Stmt {
    fn from(payload: StmtTryStar) -> Self {
        Stmt::TryStar(payload)
    }
}
impl From<StmtTryStar> for Ast {
    fn from(payload: StmtTryStar) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Assert](https://docs.python.org/3/library/ast.html#ast.Assert)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtAssert {
    pub range: TextRange,
    pub test: Box<Expr>,
    pub msg: Option<Box<Expr>>,
}

impl Node for StmtAssert {
    const NAME: &'static str = "Assert";
    const FIELD_NAMES: &'static [&'static str] = &["test", "msg"];
}
impl From<StmtAssert> for Stmt {
    fn from(payload: StmtAssert) -> Self {
        Stmt::Assert(payload)
    }
}
impl From<StmtAssert> for Ast {
    fn from(payload: StmtAssert) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Import](https://docs.python.org/3/library/ast.html#ast.Import)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtImport {
    pub range: TextRange,
    pub names: Vec<Alias>,
}

impl Node for StmtImport {
    const NAME: &'static str = "Import";
    const FIELD_NAMES: &'static [&'static str] = &["names"];
}
impl From<StmtImport> for Stmt {
    fn from(payload: StmtImport) -> Self {
        Stmt::Import(payload)
    }
}
impl From<StmtImport> for Ast {
    fn from(payload: StmtImport) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [ImportFrom](https://docs.python.org/3/library/ast.html#ast.ImportFrom)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtImportFrom {
    pub range: TextRange,
    pub module: Option<Identifier>,
    pub names: Vec<Alias>,
    pub level: Option<Int>,
}

impl Node for StmtImportFrom {
    const NAME: &'static str = "ImportFrom";
    const FIELD_NAMES: &'static [&'static str] = &["module", "names", "level"];
}
impl From<StmtImportFrom> for Stmt {
    fn from(payload: StmtImportFrom) -> Self {
        Stmt::ImportFrom(payload)
    }
}
impl From<StmtImportFrom> for Ast {
    fn from(payload: StmtImportFrom) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Global](https://docs.python.org/3/library/ast.html#ast.Global)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtGlobal {
    pub range: TextRange,
    pub names: Vec<Identifier>,
}

impl Node for StmtGlobal {
    const NAME: &'static str = "Global";
    const FIELD_NAMES: &'static [&'static str] = &["names"];
}
impl From<StmtGlobal> for Stmt {
    fn from(payload: StmtGlobal) -> Self {
        Stmt::Global(payload)
    }
}
impl From<StmtGlobal> for Ast {
    fn from(payload: StmtGlobal) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Nonlocal](https://docs.python.org/3/library/ast.html#ast.Nonlocal)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtNonlocal {
    pub range: TextRange,
    pub names: Vec<Identifier>,
}

impl Node for StmtNonlocal {
    const NAME: &'static str = "Nonlocal";
    const FIELD_NAMES: &'static [&'static str] = &["names"];
}
impl From<StmtNonlocal> for Stmt {
    fn from(payload: StmtNonlocal) -> Self {
        Stmt::Nonlocal(payload)
    }
}
impl From<StmtNonlocal> for Ast {
    fn from(payload: StmtNonlocal) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Expr](https://docs.python.org/3/library/ast.html#ast.Expr)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtExpr {
    pub range: TextRange,
    pub value: Box<Expr>,
}

impl Node for StmtExpr {
    const NAME: &'static str = "Expr";
    const FIELD_NAMES: &'static [&'static str] = &["value"];
}
impl From<StmtExpr> for Stmt {
    fn from(payload: StmtExpr) -> Self {
        Stmt::Expr(payload)
    }
}
impl From<StmtExpr> for Ast {
    fn from(payload: StmtExpr) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Pass](https://docs.python.org/3/library/ast.html#ast.Pass)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtPass {
    pub range: TextRange,
}

impl Node for StmtPass {
    const NAME: &'static str = "Pass";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl From<StmtPass> for Stmt {
    fn from(payload: StmtPass) -> Self {
        Stmt::Pass(payload)
    }
}
impl From<StmtPass> for Ast {
    fn from(payload: StmtPass) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Break](https://docs.python.org/3/library/ast.html#ast.Break)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtBreak {
    pub range: TextRange,
}

impl Node for StmtBreak {
    const NAME: &'static str = "Break";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl From<StmtBreak> for Stmt {
    fn from(payload: StmtBreak) -> Self {
        Stmt::Break(payload)
    }
}
impl From<StmtBreak> for Ast {
    fn from(payload: StmtBreak) -> Self {
        Stmt::from(payload).into()
    }
}

/// See also [Continue](https://docs.python.org/3/library/ast.html#ast.Continue)
#[derive(Clone, Debug, PartialEq)]
pub struct StmtContinue {
    pub range: TextRange,
}

impl Node for StmtContinue {
    const NAME: &'static str = "Continue";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl From<StmtContinue> for Stmt {
    fn from(payload: StmtContinue) -> Self {
        Stmt::Continue(payload)
    }
}
impl From<StmtContinue> for Ast {
    fn from(payload: StmtContinue) -> Self {
        Stmt::from(payload).into()
    }
}

impl Node for Stmt {
    const NAME: &'static str = "stmt";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

/// See also [expr](https://docs.python.org/3/library/ast.html#ast.expr)
#[derive(Clone, Debug, PartialEq, is_macro::Is)]
pub enum Expr {
    #[is(name = "bool_op_expr")]
    BoolOp(ExprBoolOp),
    #[is(name = "named_expr_expr")]
    NamedExpr(ExprNamedExpr),
    #[is(name = "bin_op_expr")]
    BinOp(ExprBinOp),
    #[is(name = "unary_op_expr")]
    UnaryOp(ExprUnaryOp),
    #[is(name = "lambda_expr")]
    Lambda(ExprLambda),
    #[is(name = "if_exp_expr")]
    IfExp(ExprIfExp),
    #[is(name = "dict_expr")]
    Dict(ExprDict),
    #[is(name = "set_expr")]
    Set(ExprSet),
    #[is(name = "list_comp_expr")]
    ListComp(ExprListComp),
    #[is(name = "set_comp_expr")]
    SetComp(ExprSetComp),
    #[is(name = "dict_comp_expr")]
    DictComp(ExprDictComp),
    #[is(name = "generator_exp_expr")]
    GeneratorExp(ExprGeneratorExp),
    #[is(name = "await_expr")]
    Await(ExprAwait),
    #[is(name = "yield_expr")]
    Yield(ExprYield),
    #[is(name = "yield_from_expr")]
    YieldFrom(ExprYieldFrom),
    #[is(name = "compare_expr")]
    Compare(ExprCompare),
    #[is(name = "call_expr")]
    Call(ExprCall),
    #[is(name = "formatted_value_expr")]
    FormattedValue(ExprFormattedValue),
    #[is(name = "joined_str_expr")]
    JoinedStr(ExprJoinedStr),
    #[is(name = "constant_expr")]
    Constant(ExprConstant),
    #[is(name = "attribute_expr")]
    Attribute(ExprAttribute),
    #[is(name = "subscript_expr")]
    Subscript(ExprSubscript),
    #[is(name = "starred_expr")]
    Starred(ExprStarred),
    #[is(name = "name_expr")]
    Name(ExprName),
    #[is(name = "list_expr")]
    List(ExprList),
    #[is(name = "tuple_expr")]
    Tuple(ExprTuple),
    #[is(name = "slice_expr")]
    Slice(ExprSlice),
}

/// See also [BoolOp](https://docs.python.org/3/library/ast.html#ast.BoolOp)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprBoolOp {
    pub range: TextRange,
    pub op: BoolOp,
    pub values: Vec<Expr>,
}

impl Node for ExprBoolOp {
    const NAME: &'static str = "BoolOp";
    const FIELD_NAMES: &'static [&'static str] = &["op", "values"];
}
impl From<ExprBoolOp> for Expr {
    fn from(payload: ExprBoolOp) -> Self {
        Expr::BoolOp(payload)
    }
}
impl From<ExprBoolOp> for Ast {
    fn from(payload: ExprBoolOp) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [NamedExpr](https://docs.python.org/3/library/ast.html#ast.NamedExpr)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprNamedExpr {
    pub range: TextRange,
    pub target: Box<Expr>,
    pub value: Box<Expr>,
}

impl Node for ExprNamedExpr {
    const NAME: &'static str = "NamedExpr";
    const FIELD_NAMES: &'static [&'static str] = &["target", "value"];
}
impl From<ExprNamedExpr> for Expr {
    fn from(payload: ExprNamedExpr) -> Self {
        Expr::NamedExpr(payload)
    }
}
impl From<ExprNamedExpr> for Ast {
    fn from(payload: ExprNamedExpr) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [BinOp](https://docs.python.org/3/library/ast.html#ast.BinOp)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprBinOp {
    pub range: TextRange,
    pub left: Box<Expr>,
    pub op: Operator,
    pub right: Box<Expr>,
}

impl Node for ExprBinOp {
    const NAME: &'static str = "BinOp";
    const FIELD_NAMES: &'static [&'static str] = &["left", "op", "right"];
}
impl From<ExprBinOp> for Expr {
    fn from(payload: ExprBinOp) -> Self {
        Expr::BinOp(payload)
    }
}
impl From<ExprBinOp> for Ast {
    fn from(payload: ExprBinOp) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [UnaryOp](https://docs.python.org/3/library/ast.html#ast.UnaryOp)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprUnaryOp {
    pub range: TextRange,
    pub op: UnaryOp,
    pub operand: Box<Expr>,
}

impl Node for ExprUnaryOp {
    const NAME: &'static str = "UnaryOp";
    const FIELD_NAMES: &'static [&'static str] = &["op", "operand"];
}
impl From<ExprUnaryOp> for Expr {
    fn from(payload: ExprUnaryOp) -> Self {
        Expr::UnaryOp(payload)
    }
}
impl From<ExprUnaryOp> for Ast {
    fn from(payload: ExprUnaryOp) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Lambda](https://docs.python.org/3/library/ast.html#ast.Lambda)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprLambda {
    pub range: TextRange,
    pub args: Box<Arguments>,
    pub body: Box<Expr>,
}

impl Node for ExprLambda {
    const NAME: &'static str = "Lambda";
    const FIELD_NAMES: &'static [&'static str] = &["args", "body"];
}
impl From<ExprLambda> for Expr {
    fn from(payload: ExprLambda) -> Self {
        Expr::Lambda(payload)
    }
}
impl From<ExprLambda> for Ast {
    fn from(payload: ExprLambda) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [IfExp](https://docs.python.org/3/library/ast.html#ast.IfExp)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprIfExp {
    pub range: TextRange,
    pub test: Box<Expr>,
    pub body: Box<Expr>,
    pub orelse: Box<Expr>,
}

impl Node for ExprIfExp {
    const NAME: &'static str = "IfExp";
    const FIELD_NAMES: &'static [&'static str] = &["test", "body", "orelse"];
}
impl From<ExprIfExp> for Expr {
    fn from(payload: ExprIfExp) -> Self {
        Expr::IfExp(payload)
    }
}
impl From<ExprIfExp> for Ast {
    fn from(payload: ExprIfExp) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Dict](https://docs.python.org/3/library/ast.html#ast.Dict)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprDict {
    pub range: TextRange,
    pub keys: Vec<Option<Expr>>,
    pub values: Vec<Expr>,
}

impl Node for ExprDict {
    const NAME: &'static str = "Dict";
    const FIELD_NAMES: &'static [&'static str] = &["keys", "values"];
}
impl From<ExprDict> for Expr {
    fn from(payload: ExprDict) -> Self {
        Expr::Dict(payload)
    }
}
impl From<ExprDict> for Ast {
    fn from(payload: ExprDict) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Set](https://docs.python.org/3/library/ast.html#ast.Set)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprSet {
    pub range: TextRange,
    pub elts: Vec<Expr>,
}

impl Node for ExprSet {
    const NAME: &'static str = "Set";
    const FIELD_NAMES: &'static [&'static str] = &["elts"];
}
impl From<ExprSet> for Expr {
    fn from(payload: ExprSet) -> Self {
        Expr::Set(payload)
    }
}
impl From<ExprSet> for Ast {
    fn from(payload: ExprSet) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [ListComp](https://docs.python.org/3/library/ast.html#ast.ListComp)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprListComp {
    pub range: TextRange,
    pub elt: Box<Expr>,
    pub generators: Vec<Comprehension>,
}

impl Node for ExprListComp {
    const NAME: &'static str = "ListComp";
    const FIELD_NAMES: &'static [&'static str] = &["elt", "generators"];
}
impl From<ExprListComp> for Expr {
    fn from(payload: ExprListComp) -> Self {
        Expr::ListComp(payload)
    }
}
impl From<ExprListComp> for Ast {
    fn from(payload: ExprListComp) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [SetComp](https://docs.python.org/3/library/ast.html#ast.SetComp)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprSetComp {
    pub range: TextRange,
    pub elt: Box<Expr>,
    pub generators: Vec<Comprehension>,
}

impl Node for ExprSetComp {
    const NAME: &'static str = "SetComp";
    const FIELD_NAMES: &'static [&'static str] = &["elt", "generators"];
}
impl From<ExprSetComp> for Expr {
    fn from(payload: ExprSetComp) -> Self {
        Expr::SetComp(payload)
    }
}
impl From<ExprSetComp> for Ast {
    fn from(payload: ExprSetComp) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [DictComp](https://docs.python.org/3/library/ast.html#ast.DictComp)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprDictComp {
    pub range: TextRange,
    pub key: Box<Expr>,
    pub value: Box<Expr>,
    pub generators: Vec<Comprehension>,
}

impl Node for ExprDictComp {
    const NAME: &'static str = "DictComp";
    const FIELD_NAMES: &'static [&'static str] = &["key", "value", "generators"];
}
impl From<ExprDictComp> for Expr {
    fn from(payload: ExprDictComp) -> Self {
        Expr::DictComp(payload)
    }
}
impl From<ExprDictComp> for Ast {
    fn from(payload: ExprDictComp) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [GeneratorExp](https://docs.python.org/3/library/ast.html#ast.GeneratorExp)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprGeneratorExp {
    pub range: TextRange,
    pub elt: Box<Expr>,
    pub generators: Vec<Comprehension>,
}

impl Node for ExprGeneratorExp {
    const NAME: &'static str = "GeneratorExp";
    const FIELD_NAMES: &'static [&'static str] = &["elt", "generators"];
}
impl From<ExprGeneratorExp> for Expr {
    fn from(payload: ExprGeneratorExp) -> Self {
        Expr::GeneratorExp(payload)
    }
}
impl From<ExprGeneratorExp> for Ast {
    fn from(payload: ExprGeneratorExp) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Await](https://docs.python.org/3/library/ast.html#ast.Await)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprAwait {
    pub range: TextRange,
    pub value: Box<Expr>,
}

impl Node for ExprAwait {
    const NAME: &'static str = "Await";
    const FIELD_NAMES: &'static [&'static str] = &["value"];
}
impl From<ExprAwait> for Expr {
    fn from(payload: ExprAwait) -> Self {
        Expr::Await(payload)
    }
}
impl From<ExprAwait> for Ast {
    fn from(payload: ExprAwait) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Yield](https://docs.python.org/3/library/ast.html#ast.Yield)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprYield {
    pub range: TextRange,
    pub value: Option<Box<Expr>>,
}

impl Node for ExprYield {
    const NAME: &'static str = "Yield";
    const FIELD_NAMES: &'static [&'static str] = &["value"];
}
impl From<ExprYield> for Expr {
    fn from(payload: ExprYield) -> Self {
        Expr::Yield(payload)
    }
}
impl From<ExprYield> for Ast {
    fn from(payload: ExprYield) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [YieldFrom](https://docs.python.org/3/library/ast.html#ast.YieldFrom)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprYieldFrom {
    pub range: TextRange,
    pub value: Box<Expr>,
}

impl Node for ExprYieldFrom {
    const NAME: &'static str = "YieldFrom";
    const FIELD_NAMES: &'static [&'static str] = &["value"];
}
impl From<ExprYieldFrom> for Expr {
    fn from(payload: ExprYieldFrom) -> Self {
        Expr::YieldFrom(payload)
    }
}
impl From<ExprYieldFrom> for Ast {
    fn from(payload: ExprYieldFrom) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Compare](https://docs.python.org/3/library/ast.html#ast.Compare)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprCompare {
    pub range: TextRange,
    pub left: Box<Expr>,
    pub ops: Vec<CmpOp>,
    pub comparators: Vec<Expr>,
}

impl Node for ExprCompare {
    const NAME: &'static str = "Compare";
    const FIELD_NAMES: &'static [&'static str] = &["left", "ops", "comparators"];
}
impl From<ExprCompare> for Expr {
    fn from(payload: ExprCompare) -> Self {
        Expr::Compare(payload)
    }
}
impl From<ExprCompare> for Ast {
    fn from(payload: ExprCompare) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Call](https://docs.python.org/3/library/ast.html#ast.Call)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprCall {
    pub range: TextRange,
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
    pub keywords: Vec<Keyword>,
}

impl Node for ExprCall {
    const NAME: &'static str = "Call";
    const FIELD_NAMES: &'static [&'static str] = &["func", "args", "keywords"];
}
impl From<ExprCall> for Expr {
    fn from(payload: ExprCall) -> Self {
        Expr::Call(payload)
    }
}
impl From<ExprCall> for Ast {
    fn from(payload: ExprCall) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [FormattedValue](https://docs.python.org/3/library/ast.html#ast.FormattedValue)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprFormattedValue {
    pub range: TextRange,
    pub value: Box<Expr>,
    pub conversion: ConversionFlag,
    pub format_spec: Option<Box<Expr>>,
}

impl Node for ExprFormattedValue {
    const NAME: &'static str = "FormattedValue";
    const FIELD_NAMES: &'static [&'static str] = &["value", "conversion", "format_spec"];
}
impl From<ExprFormattedValue> for Expr {
    fn from(payload: ExprFormattedValue) -> Self {
        Expr::FormattedValue(payload)
    }
}
impl From<ExprFormattedValue> for Ast {
    fn from(payload: ExprFormattedValue) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [JoinedStr](https://docs.python.org/3/library/ast.html#ast.JoinedStr)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprJoinedStr {
    pub range: TextRange,
    pub values: Vec<Expr>,
}

impl Node for ExprJoinedStr {
    const NAME: &'static str = "JoinedStr";
    const FIELD_NAMES: &'static [&'static str] = &["values"];
}
impl From<ExprJoinedStr> for Expr {
    fn from(payload: ExprJoinedStr) -> Self {
        Expr::JoinedStr(payload)
    }
}
impl From<ExprJoinedStr> for Ast {
    fn from(payload: ExprJoinedStr) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Constant](https://docs.python.org/3/library/ast.html#ast.Constant)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprConstant {
    pub range: TextRange,
    pub value: Constant,
    pub kind: Option<String>,
}

impl Node for ExprConstant {
    const NAME: &'static str = "Constant";
    const FIELD_NAMES: &'static [&'static str] = &["value", "kind"];
}
impl From<ExprConstant> for Expr {
    fn from(payload: ExprConstant) -> Self {
        Expr::Constant(payload)
    }
}
impl From<ExprConstant> for Ast {
    fn from(payload: ExprConstant) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Attribute](https://docs.python.org/3/library/ast.html#ast.Attribute)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprAttribute {
    pub range: TextRange,
    pub value: Box<Expr>,
    pub attr: Identifier,
    pub ctx: ExprContext,
}

impl Node for ExprAttribute {
    const NAME: &'static str = "Attribute";
    const FIELD_NAMES: &'static [&'static str] = &["value", "attr", "ctx"];
}
impl From<ExprAttribute> for Expr {
    fn from(payload: ExprAttribute) -> Self {
        Expr::Attribute(payload)
    }
}
impl From<ExprAttribute> for Ast {
    fn from(payload: ExprAttribute) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Subscript](https://docs.python.org/3/library/ast.html#ast.Subscript)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprSubscript {
    pub range: TextRange,
    pub value: Box<Expr>,
    pub slice: Box<Expr>,
    pub ctx: ExprContext,
}

impl Node for ExprSubscript {
    const NAME: &'static str = "Subscript";
    const FIELD_NAMES: &'static [&'static str] = &["value", "slice", "ctx"];
}
impl From<ExprSubscript> for Expr {
    fn from(payload: ExprSubscript) -> Self {
        Expr::Subscript(payload)
    }
}
impl From<ExprSubscript> for Ast {
    fn from(payload: ExprSubscript) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Starred](https://docs.python.org/3/library/ast.html#ast.Starred)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprStarred {
    pub range: TextRange,
    pub value: Box<Expr>,
    pub ctx: ExprContext,
}

impl Node for ExprStarred {
    const NAME: &'static str = "Starred";
    const FIELD_NAMES: &'static [&'static str] = &["value", "ctx"];
}
impl From<ExprStarred> for Expr {
    fn from(payload: ExprStarred) -> Self {
        Expr::Starred(payload)
    }
}
impl From<ExprStarred> for Ast {
    fn from(payload: ExprStarred) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Name](https://docs.python.org/3/library/ast.html#ast.Name)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprName {
    pub range: TextRange,
    pub id: String,
    pub ctx: ExprContext,
}

impl Node for ExprName {
    const NAME: &'static str = "Name";
    const FIELD_NAMES: &'static [&'static str] = &["id", "ctx"];
}
impl From<ExprName> for Expr {
    fn from(payload: ExprName) -> Self {
        Expr::Name(payload)
    }
}
impl From<ExprName> for Ast {
    fn from(payload: ExprName) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [List](https://docs.python.org/3/library/ast.html#ast.List)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprList {
    pub range: TextRange,
    pub elts: Vec<Expr>,
    pub ctx: ExprContext,
}

impl Node for ExprList {
    const NAME: &'static str = "List";
    const FIELD_NAMES: &'static [&'static str] = &["elts", "ctx"];
}
impl From<ExprList> for Expr {
    fn from(payload: ExprList) -> Self {
        Expr::List(payload)
    }
}
impl From<ExprList> for Ast {
    fn from(payload: ExprList) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Tuple](https://docs.python.org/3/library/ast.html#ast.Tuple)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprTuple {
    pub range: TextRange,
    pub elts: Vec<Expr>,
    pub ctx: ExprContext,
}

impl Node for ExprTuple {
    const NAME: &'static str = "Tuple";
    const FIELD_NAMES: &'static [&'static str] = &["elts", "ctx"];
}
impl From<ExprTuple> for Expr {
    fn from(payload: ExprTuple) -> Self {
        Expr::Tuple(payload)
    }
}
impl From<ExprTuple> for Ast {
    fn from(payload: ExprTuple) -> Self {
        Expr::from(payload).into()
    }
}

/// See also [Slice](https://docs.python.org/3/library/ast.html#ast.Slice)
#[derive(Clone, Debug, PartialEq)]
pub struct ExprSlice {
    pub range: TextRange,
    pub lower: Option<Box<Expr>>,
    pub upper: Option<Box<Expr>>,
    pub step: Option<Box<Expr>>,
}

impl Node for ExprSlice {
    const NAME: &'static str = "Slice";
    const FIELD_NAMES: &'static [&'static str] = &["lower", "upper", "step"];
}
impl From<ExprSlice> for Expr {
    fn from(payload: ExprSlice) -> Self {
        Expr::Slice(payload)
    }
}
impl From<ExprSlice> for Ast {
    fn from(payload: ExprSlice) -> Self {
        Expr::from(payload).into()
    }
}

impl Node for Expr {
    const NAME: &'static str = "expr";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

/// See also [expr_context](https://docs.python.org/3/library/ast.html#ast.expr_context)
#[derive(Clone, Debug, PartialEq, is_macro::Is, Copy, Hash, Eq)]
pub enum ExprContext {
    Load,
    Store,
    Del,
}
impl ExprContext {
    #[inline]
    pub const fn load(&self) -> Option<ExprContextLoad> {
        match self {
            ExprContext::Load => Some(ExprContextLoad),
            _ => None,
        }
    }

    #[inline]
    pub const fn store(&self) -> Option<ExprContextStore> {
        match self {
            ExprContext::Store => Some(ExprContextStore),
            _ => None,
        }
    }

    #[inline]
    pub const fn del(&self) -> Option<ExprContextDel> {
        match self {
            ExprContext::Del => Some(ExprContextDel),
            _ => None,
        }
    }
}

pub struct ExprContextLoad;
impl From<ExprContextLoad> for ExprContext {
    fn from(_: ExprContextLoad) -> Self {
        ExprContext::Load
    }
}
impl From<ExprContextLoad> for Ast {
    fn from(_: ExprContextLoad) -> Self {
        ExprContext::Load.into()
    }
}
impl Node for ExprContextLoad {
    const NAME: &'static str = "Load";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<ExprContext> for ExprContextLoad {
    #[inline]
    fn eq(&self, other: &ExprContext) -> bool {
        matches!(other, ExprContext::Load)
    }
}

pub struct ExprContextStore;
impl From<ExprContextStore> for ExprContext {
    fn from(_: ExprContextStore) -> Self {
        ExprContext::Store
    }
}
impl From<ExprContextStore> for Ast {
    fn from(_: ExprContextStore) -> Self {
        ExprContext::Store.into()
    }
}
impl Node for ExprContextStore {
    const NAME: &'static str = "Store";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<ExprContext> for ExprContextStore {
    #[inline]
    fn eq(&self, other: &ExprContext) -> bool {
        matches!(other, ExprContext::Store)
    }
}

pub struct ExprContextDel;
impl From<ExprContextDel> for ExprContext {
    fn from(_: ExprContextDel) -> Self {
        ExprContext::Del
    }
}
impl From<ExprContextDel> for Ast {
    fn from(_: ExprContextDel) -> Self {
        ExprContext::Del.into()
    }
}
impl Node for ExprContextDel {
    const NAME: &'static str = "Del";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<ExprContext> for ExprContextDel {
    #[inline]
    fn eq(&self, other: &ExprContext) -> bool {
        matches!(other, ExprContext::Del)
    }
}

impl Node for ExprContext {
    const NAME: &'static str = "expr_context";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

/// See also [boolop](https://docs.python.org/3/library/ast.html#ast.boolop)
#[derive(Clone, Debug, PartialEq, is_macro::Is, Copy, Hash, Eq)]
pub enum BoolOp {
    And,
    Or,
}
impl BoolOp {
    #[inline]
    pub const fn and(&self) -> Option<BoolOpAnd> {
        match self {
            BoolOp::And => Some(BoolOpAnd),
            _ => None,
        }
    }

    #[inline]
    pub const fn or(&self) -> Option<BoolOpOr> {
        match self {
            BoolOp::Or => Some(BoolOpOr),
            _ => None,
        }
    }
}

pub struct BoolOpAnd;
impl From<BoolOpAnd> for BoolOp {
    fn from(_: BoolOpAnd) -> Self {
        BoolOp::And
    }
}
impl From<BoolOpAnd> for Ast {
    fn from(_: BoolOpAnd) -> Self {
        BoolOp::And.into()
    }
}
impl Node for BoolOpAnd {
    const NAME: &'static str = "And";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<BoolOp> for BoolOpAnd {
    #[inline]
    fn eq(&self, other: &BoolOp) -> bool {
        matches!(other, BoolOp::And)
    }
}

pub struct BoolOpOr;
impl From<BoolOpOr> for BoolOp {
    fn from(_: BoolOpOr) -> Self {
        BoolOp::Or
    }
}
impl From<BoolOpOr> for Ast {
    fn from(_: BoolOpOr) -> Self {
        BoolOp::Or.into()
    }
}
impl Node for BoolOpOr {
    const NAME: &'static str = "Or";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<BoolOp> for BoolOpOr {
    #[inline]
    fn eq(&self, other: &BoolOp) -> bool {
        matches!(other, BoolOp::Or)
    }
}

impl Node for BoolOp {
    const NAME: &'static str = "boolop";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

/// See also [operator](https://docs.python.org/3/library/ast.html#ast.operator)
#[derive(Clone, Debug, PartialEq, is_macro::Is, Copy, Hash, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}
impl Operator {
    #[inline]
    pub const fn operator_add(&self) -> Option<OperatorAdd> {
        match self {
            Operator::Add => Some(OperatorAdd),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_sub(&self) -> Option<OperatorSub> {
        match self {
            Operator::Sub => Some(OperatorSub),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_mult(&self) -> Option<OperatorMult> {
        match self {
            Operator::Mult => Some(OperatorMult),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_mat_mult(&self) -> Option<OperatorMatMult> {
        match self {
            Operator::MatMult => Some(OperatorMatMult),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_div(&self) -> Option<OperatorDiv> {
        match self {
            Operator::Div => Some(OperatorDiv),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_mod(&self) -> Option<OperatorMod> {
        match self {
            Operator::Mod => Some(OperatorMod),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_pow(&self) -> Option<OperatorPow> {
        match self {
            Operator::Pow => Some(OperatorPow),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_l_shift(&self) -> Option<OperatorLShift> {
        match self {
            Operator::LShift => Some(OperatorLShift),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_r_shift(&self) -> Option<OperatorRShift> {
        match self {
            Operator::RShift => Some(OperatorRShift),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_bit_or(&self) -> Option<OperatorBitOr> {
        match self {
            Operator::BitOr => Some(OperatorBitOr),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_bit_xor(&self) -> Option<OperatorBitXor> {
        match self {
            Operator::BitXor => Some(OperatorBitXor),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_bit_and(&self) -> Option<OperatorBitAnd> {
        match self {
            Operator::BitAnd => Some(OperatorBitAnd),
            _ => None,
        }
    }

    #[inline]
    pub const fn operator_floor_div(&self) -> Option<OperatorFloorDiv> {
        match self {
            Operator::FloorDiv => Some(OperatorFloorDiv),
            _ => None,
        }
    }
}

pub struct OperatorAdd;
impl From<OperatorAdd> for Operator {
    fn from(_: OperatorAdd) -> Self {
        Operator::Add
    }
}
impl From<OperatorAdd> for Ast {
    fn from(_: OperatorAdd) -> Self {
        Operator::Add.into()
    }
}
impl Node for OperatorAdd {
    const NAME: &'static str = "Add";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorAdd {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::Add)
    }
}

pub struct OperatorSub;
impl From<OperatorSub> for Operator {
    fn from(_: OperatorSub) -> Self {
        Operator::Sub
    }
}
impl From<OperatorSub> for Ast {
    fn from(_: OperatorSub) -> Self {
        Operator::Sub.into()
    }
}
impl Node for OperatorSub {
    const NAME: &'static str = "Sub";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorSub {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::Sub)
    }
}

pub struct OperatorMult;
impl From<OperatorMult> for Operator {
    fn from(_: OperatorMult) -> Self {
        Operator::Mult
    }
}
impl From<OperatorMult> for Ast {
    fn from(_: OperatorMult) -> Self {
        Operator::Mult.into()
    }
}
impl Node for OperatorMult {
    const NAME: &'static str = "Mult";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorMult {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::Mult)
    }
}

pub struct OperatorMatMult;
impl From<OperatorMatMult> for Operator {
    fn from(_: OperatorMatMult) -> Self {
        Operator::MatMult
    }
}
impl From<OperatorMatMult> for Ast {
    fn from(_: OperatorMatMult) -> Self {
        Operator::MatMult.into()
    }
}
impl Node for OperatorMatMult {
    const NAME: &'static str = "MatMult";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorMatMult {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::MatMult)
    }
}

pub struct OperatorDiv;
impl From<OperatorDiv> for Operator {
    fn from(_: OperatorDiv) -> Self {
        Operator::Div
    }
}
impl From<OperatorDiv> for Ast {
    fn from(_: OperatorDiv) -> Self {
        Operator::Div.into()
    }
}
impl Node for OperatorDiv {
    const NAME: &'static str = "Div";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorDiv {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::Div)
    }
}

pub struct OperatorMod;
impl From<OperatorMod> for Operator {
    fn from(_: OperatorMod) -> Self {
        Operator::Mod
    }
}
impl From<OperatorMod> for Ast {
    fn from(_: OperatorMod) -> Self {
        Operator::Mod.into()
    }
}
impl Node for OperatorMod {
    const NAME: &'static str = "Mod";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorMod {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::Mod)
    }
}

pub struct OperatorPow;
impl From<OperatorPow> for Operator {
    fn from(_: OperatorPow) -> Self {
        Operator::Pow
    }
}
impl From<OperatorPow> for Ast {
    fn from(_: OperatorPow) -> Self {
        Operator::Pow.into()
    }
}
impl Node for OperatorPow {
    const NAME: &'static str = "Pow";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorPow {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::Pow)
    }
}

pub struct OperatorLShift;
impl From<OperatorLShift> for Operator {
    fn from(_: OperatorLShift) -> Self {
        Operator::LShift
    }
}
impl From<OperatorLShift> for Ast {
    fn from(_: OperatorLShift) -> Self {
        Operator::LShift.into()
    }
}
impl Node for OperatorLShift {
    const NAME: &'static str = "LShift";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorLShift {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::LShift)
    }
}

pub struct OperatorRShift;
impl From<OperatorRShift> for Operator {
    fn from(_: OperatorRShift) -> Self {
        Operator::RShift
    }
}
impl From<OperatorRShift> for Ast {
    fn from(_: OperatorRShift) -> Self {
        Operator::RShift.into()
    }
}
impl Node for OperatorRShift {
    const NAME: &'static str = "RShift";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorRShift {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::RShift)
    }
}

pub struct OperatorBitOr;
impl From<OperatorBitOr> for Operator {
    fn from(_: OperatorBitOr) -> Self {
        Operator::BitOr
    }
}
impl From<OperatorBitOr> for Ast {
    fn from(_: OperatorBitOr) -> Self {
        Operator::BitOr.into()
    }
}
impl Node for OperatorBitOr {
    const NAME: &'static str = "BitOr";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorBitOr {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::BitOr)
    }
}

pub struct OperatorBitXor;
impl From<OperatorBitXor> for Operator {
    fn from(_: OperatorBitXor) -> Self {
        Operator::BitXor
    }
}
impl From<OperatorBitXor> for Ast {
    fn from(_: OperatorBitXor) -> Self {
        Operator::BitXor.into()
    }
}
impl Node for OperatorBitXor {
    const NAME: &'static str = "BitXor";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorBitXor {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::BitXor)
    }
}

pub struct OperatorBitAnd;
impl From<OperatorBitAnd> for Operator {
    fn from(_: OperatorBitAnd) -> Self {
        Operator::BitAnd
    }
}
impl From<OperatorBitAnd> for Ast {
    fn from(_: OperatorBitAnd) -> Self {
        Operator::BitAnd.into()
    }
}
impl Node for OperatorBitAnd {
    const NAME: &'static str = "BitAnd";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorBitAnd {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::BitAnd)
    }
}

pub struct OperatorFloorDiv;
impl From<OperatorFloorDiv> for Operator {
    fn from(_: OperatorFloorDiv) -> Self {
        Operator::FloorDiv
    }
}
impl From<OperatorFloorDiv> for Ast {
    fn from(_: OperatorFloorDiv) -> Self {
        Operator::FloorDiv.into()
    }
}
impl Node for OperatorFloorDiv {
    const NAME: &'static str = "FloorDiv";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<Operator> for OperatorFloorDiv {
    #[inline]
    fn eq(&self, other: &Operator) -> bool {
        matches!(other, Operator::FloorDiv)
    }
}

impl Node for Operator {
    const NAME: &'static str = "operator";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

/// See also [unaryop](https://docs.python.org/3/library/ast.html#ast.unaryop)
#[derive(Clone, Debug, PartialEq, is_macro::Is, Copy, Hash, Eq)]
pub enum UnaryOp {
    Invert,
    Not,
    UAdd,
    USub,
}
impl UnaryOp {
    #[inline]
    pub const fn invert(&self) -> Option<UnaryOpInvert> {
        match self {
            UnaryOp::Invert => Some(UnaryOpInvert),
            _ => None,
        }
    }

    #[inline]
    pub const fn not(&self) -> Option<UnaryOpNot> {
        match self {
            UnaryOp::Not => Some(UnaryOpNot),
            _ => None,
        }
    }

    #[inline]
    pub const fn u_add(&self) -> Option<UnaryOpUAdd> {
        match self {
            UnaryOp::UAdd => Some(UnaryOpUAdd),
            _ => None,
        }
    }

    #[inline]
    pub const fn u_sub(&self) -> Option<UnaryOpUSub> {
        match self {
            UnaryOp::USub => Some(UnaryOpUSub),
            _ => None,
        }
    }
}

pub struct UnaryOpInvert;
impl From<UnaryOpInvert> for UnaryOp {
    fn from(_: UnaryOpInvert) -> Self {
        UnaryOp::Invert
    }
}
impl From<UnaryOpInvert> for Ast {
    fn from(_: UnaryOpInvert) -> Self {
        UnaryOp::Invert.into()
    }
}
impl Node for UnaryOpInvert {
    const NAME: &'static str = "Invert";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<UnaryOp> for UnaryOpInvert {
    #[inline]
    fn eq(&self, other: &UnaryOp) -> bool {
        matches!(other, UnaryOp::Invert)
    }
}

pub struct UnaryOpNot;
impl From<UnaryOpNot> for UnaryOp {
    fn from(_: UnaryOpNot) -> Self {
        UnaryOp::Not
    }
}
impl From<UnaryOpNot> for Ast {
    fn from(_: UnaryOpNot) -> Self {
        UnaryOp::Not.into()
    }
}
impl Node for UnaryOpNot {
    const NAME: &'static str = "Not";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<UnaryOp> for UnaryOpNot {
    #[inline]
    fn eq(&self, other: &UnaryOp) -> bool {
        matches!(other, UnaryOp::Not)
    }
}

pub struct UnaryOpUAdd;
impl From<UnaryOpUAdd> for UnaryOp {
    fn from(_: UnaryOpUAdd) -> Self {
        UnaryOp::UAdd
    }
}
impl From<UnaryOpUAdd> for Ast {
    fn from(_: UnaryOpUAdd) -> Self {
        UnaryOp::UAdd.into()
    }
}
impl Node for UnaryOpUAdd {
    const NAME: &'static str = "UAdd";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<UnaryOp> for UnaryOpUAdd {
    #[inline]
    fn eq(&self, other: &UnaryOp) -> bool {
        matches!(other, UnaryOp::UAdd)
    }
}

pub struct UnaryOpUSub;
impl From<UnaryOpUSub> for UnaryOp {
    fn from(_: UnaryOpUSub) -> Self {
        UnaryOp::USub
    }
}
impl From<UnaryOpUSub> for Ast {
    fn from(_: UnaryOpUSub) -> Self {
        UnaryOp::USub.into()
    }
}
impl Node for UnaryOpUSub {
    const NAME: &'static str = "USub";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<UnaryOp> for UnaryOpUSub {
    #[inline]
    fn eq(&self, other: &UnaryOp) -> bool {
        matches!(other, UnaryOp::USub)
    }
}

impl Node for UnaryOp {
    const NAME: &'static str = "unaryop";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

/// See also [cmpop](https://docs.python.org/3/library/ast.html#ast.cmpop)
#[derive(Clone, Debug, PartialEq, is_macro::Is, Copy, Hash, Eq)]
pub enum CmpOp {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}
impl CmpOp {
    #[inline]
    pub const fn cmp_op_eq(&self) -> Option<CmpOpEq> {
        match self {
            CmpOp::Eq => Some(CmpOpEq),
            _ => None,
        }
    }

    #[inline]
    pub const fn cmp_op_not_eq(&self) -> Option<CmpOpNotEq> {
        match self {
            CmpOp::NotEq => Some(CmpOpNotEq),
            _ => None,
        }
    }

    #[inline]
    pub const fn cmp_op_lt(&self) -> Option<CmpOpLt> {
        match self {
            CmpOp::Lt => Some(CmpOpLt),
            _ => None,
        }
    }

    #[inline]
    pub const fn cmp_op_lt_e(&self) -> Option<CmpOpLtE> {
        match self {
            CmpOp::LtE => Some(CmpOpLtE),
            _ => None,
        }
    }

    #[inline]
    pub const fn cmp_op_gt(&self) -> Option<CmpOpGt> {
        match self {
            CmpOp::Gt => Some(CmpOpGt),
            _ => None,
        }
    }

    #[inline]
    pub const fn cmp_op_gt_e(&self) -> Option<CmpOpGtE> {
        match self {
            CmpOp::GtE => Some(CmpOpGtE),
            _ => None,
        }
    }

    #[inline]
    pub const fn cmp_op_is(&self) -> Option<CmpOpIs> {
        match self {
            CmpOp::Is => Some(CmpOpIs),
            _ => None,
        }
    }

    #[inline]
    pub const fn cmp_op_is_not(&self) -> Option<CmpOpIsNot> {
        match self {
            CmpOp::IsNot => Some(CmpOpIsNot),
            _ => None,
        }
    }

    #[inline]
    pub const fn cmp_op_in(&self) -> Option<CmpOpIn> {
        match self {
            CmpOp::In => Some(CmpOpIn),
            _ => None,
        }
    }

    #[inline]
    pub const fn cmp_op_not_in(&self) -> Option<CmpOpNotIn> {
        match self {
            CmpOp::NotIn => Some(CmpOpNotIn),
            _ => None,
        }
    }
}

pub struct CmpOpEq;
impl From<CmpOpEq> for CmpOp {
    fn from(_: CmpOpEq) -> Self {
        CmpOp::Eq
    }
}
impl From<CmpOpEq> for Ast {
    fn from(_: CmpOpEq) -> Self {
        CmpOp::Eq.into()
    }
}
impl Node for CmpOpEq {
    const NAME: &'static str = "Eq";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<CmpOp> for CmpOpEq {
    #[inline]
    fn eq(&self, other: &CmpOp) -> bool {
        matches!(other, CmpOp::Eq)
    }
}

pub struct CmpOpNotEq;
impl From<CmpOpNotEq> for CmpOp {
    fn from(_: CmpOpNotEq) -> Self {
        CmpOp::NotEq
    }
}
impl From<CmpOpNotEq> for Ast {
    fn from(_: CmpOpNotEq) -> Self {
        CmpOp::NotEq.into()
    }
}
impl Node for CmpOpNotEq {
    const NAME: &'static str = "NotEq";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<CmpOp> for CmpOpNotEq {
    #[inline]
    fn eq(&self, other: &CmpOp) -> bool {
        matches!(other, CmpOp::NotEq)
    }
}

pub struct CmpOpLt;
impl From<CmpOpLt> for CmpOp {
    fn from(_: CmpOpLt) -> Self {
        CmpOp::Lt
    }
}
impl From<CmpOpLt> for Ast {
    fn from(_: CmpOpLt) -> Self {
        CmpOp::Lt.into()
    }
}
impl Node for CmpOpLt {
    const NAME: &'static str = "Lt";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<CmpOp> for CmpOpLt {
    #[inline]
    fn eq(&self, other: &CmpOp) -> bool {
        matches!(other, CmpOp::Lt)
    }
}

pub struct CmpOpLtE;
impl From<CmpOpLtE> for CmpOp {
    fn from(_: CmpOpLtE) -> Self {
        CmpOp::LtE
    }
}
impl From<CmpOpLtE> for Ast {
    fn from(_: CmpOpLtE) -> Self {
        CmpOp::LtE.into()
    }
}
impl Node for CmpOpLtE {
    const NAME: &'static str = "LtE";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<CmpOp> for CmpOpLtE {
    #[inline]
    fn eq(&self, other: &CmpOp) -> bool {
        matches!(other, CmpOp::LtE)
    }
}

pub struct CmpOpGt;
impl From<CmpOpGt> for CmpOp {
    fn from(_: CmpOpGt) -> Self {
        CmpOp::Gt
    }
}
impl From<CmpOpGt> for Ast {
    fn from(_: CmpOpGt) -> Self {
        CmpOp::Gt.into()
    }
}
impl Node for CmpOpGt {
    const NAME: &'static str = "Gt";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<CmpOp> for CmpOpGt {
    #[inline]
    fn eq(&self, other: &CmpOp) -> bool {
        matches!(other, CmpOp::Gt)
    }
}

pub struct CmpOpGtE;
impl From<CmpOpGtE> for CmpOp {
    fn from(_: CmpOpGtE) -> Self {
        CmpOp::GtE
    }
}
impl From<CmpOpGtE> for Ast {
    fn from(_: CmpOpGtE) -> Self {
        CmpOp::GtE.into()
    }
}
impl Node for CmpOpGtE {
    const NAME: &'static str = "GtE";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<CmpOp> for CmpOpGtE {
    #[inline]
    fn eq(&self, other: &CmpOp) -> bool {
        matches!(other, CmpOp::GtE)
    }
}

pub struct CmpOpIs;
impl From<CmpOpIs> for CmpOp {
    fn from(_: CmpOpIs) -> Self {
        CmpOp::Is
    }
}
impl From<CmpOpIs> for Ast {
    fn from(_: CmpOpIs) -> Self {
        CmpOp::Is.into()
    }
}
impl Node for CmpOpIs {
    const NAME: &'static str = "Is";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<CmpOp> for CmpOpIs {
    #[inline]
    fn eq(&self, other: &CmpOp) -> bool {
        matches!(other, CmpOp::Is)
    }
}

pub struct CmpOpIsNot;
impl From<CmpOpIsNot> for CmpOp {
    fn from(_: CmpOpIsNot) -> Self {
        CmpOp::IsNot
    }
}
impl From<CmpOpIsNot> for Ast {
    fn from(_: CmpOpIsNot) -> Self {
        CmpOp::IsNot.into()
    }
}
impl Node for CmpOpIsNot {
    const NAME: &'static str = "IsNot";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<CmpOp> for CmpOpIsNot {
    #[inline]
    fn eq(&self, other: &CmpOp) -> bool {
        matches!(other, CmpOp::IsNot)
    }
}

pub struct CmpOpIn;
impl From<CmpOpIn> for CmpOp {
    fn from(_: CmpOpIn) -> Self {
        CmpOp::In
    }
}
impl From<CmpOpIn> for Ast {
    fn from(_: CmpOpIn) -> Self {
        CmpOp::In.into()
    }
}
impl Node for CmpOpIn {
    const NAME: &'static str = "In";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<CmpOp> for CmpOpIn {
    #[inline]
    fn eq(&self, other: &CmpOp) -> bool {
        matches!(other, CmpOp::In)
    }
}

pub struct CmpOpNotIn;
impl From<CmpOpNotIn> for CmpOp {
    fn from(_: CmpOpNotIn) -> Self {
        CmpOp::NotIn
    }
}
impl From<CmpOpNotIn> for Ast {
    fn from(_: CmpOpNotIn) -> Self {
        CmpOp::NotIn.into()
    }
}
impl Node for CmpOpNotIn {
    const NAME: &'static str = "NotIn";
    const FIELD_NAMES: &'static [&'static str] = &[];
}
impl std::cmp::PartialEq<CmpOp> for CmpOpNotIn {
    #[inline]
    fn eq(&self, other: &CmpOp) -> bool {
        matches!(other, CmpOp::NotIn)
    }
}

impl Node for CmpOp {
    const NAME: &'static str = "cmpop";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

/// See also [comprehension](https://docs.python.org/3/library/ast.html#ast.comprehension)
#[derive(Clone, Debug, PartialEq)]
pub struct Comprehension {
    pub range: TextRange,
    pub target: Expr,
    pub iter: Expr,
    pub ifs: Vec<Expr>,
    pub is_async: bool,
}

impl Node for Comprehension {
    const NAME: &'static str = "comprehension";
    const FIELD_NAMES: &'static [&'static str] = &["target", "iter", "ifs", "is_async"];
}

/// See also [excepthandler](https://docs.python.org/3/library/ast.html#ast.excepthandler)
#[derive(Clone, Debug, PartialEq, is_macro::Is)]
pub enum ExceptHandler {
    ExceptHandler(ExceptHandlerExceptHandler),
}

/// See also [ExceptHandler](https://docs.python.org/3/library/ast.html#ast.ExceptHandler)
#[derive(Clone, Debug, PartialEq)]
pub struct ExceptHandlerExceptHandler {
    pub range: TextRange,
    pub type_: Option<Box<Expr>>,
    pub name: Option<Identifier>,
    pub body: Vec<Stmt>,
}

impl Node for ExceptHandlerExceptHandler {
    const NAME: &'static str = "ExceptHandler";
    const FIELD_NAMES: &'static [&'static str] = &["type", "name", "body"];
}
impl From<ExceptHandlerExceptHandler> for ExceptHandler {
    fn from(payload: ExceptHandlerExceptHandler) -> Self {
        ExceptHandler::ExceptHandler(payload)
    }
}
impl From<ExceptHandlerExceptHandler> for Ast {
    fn from(payload: ExceptHandlerExceptHandler) -> Self {
        ExceptHandler::from(payload).into()
    }
}

impl Node for ExceptHandler {
    const NAME: &'static str = "excepthandler";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

/// See also [arguments](https://docs.python.org/3/library/ast.html#ast.arguments)
#[derive(Clone, Debug, PartialEq)]
pub struct PythonArguments {
    pub range: TextRange,
    pub posonlyargs: Vec<Arg>,
    pub args: Vec<Arg>,
    pub vararg: Option<Box<Arg>>,
    pub kwonlyargs: Vec<Arg>,
    pub kw_defaults: Vec<Expr>,
    pub kwarg: Option<Box<Arg>>,
    pub defaults: Vec<Expr>,
}

impl Node for PythonArguments {
    const NAME: &'static str = "arguments";
    const FIELD_NAMES: &'static [&'static str] = &[
        "posonlyargs",
        "args",
        "vararg",
        "kwonlyargs",
        "kw_defaults",
        "kwarg",
        "defaults",
    ];
}

/// See also [arg](https://docs.python.org/3/library/ast.html#ast.arg)
#[derive(Clone, Debug, PartialEq)]
pub struct Arg {
    pub range: TextRange,
    pub arg: Identifier,
    pub annotation: Option<Box<Expr>>,
    pub type_comment: Option<String>,
}

impl Node for Arg {
    const NAME: &'static str = "arg";
    const FIELD_NAMES: &'static [&'static str] = &["arg", "annotation", "type_comment"];
}

/// See also [keyword](https://docs.python.org/3/library/ast.html#ast.keyword)
#[derive(Clone, Debug, PartialEq)]
pub struct Keyword {
    pub range: TextRange,
    pub arg: Option<Identifier>,
    pub value: Expr,
}

impl Node for Keyword {
    const NAME: &'static str = "keyword";
    const FIELD_NAMES: &'static [&'static str] = &["arg", "value"];
}

/// See also [alias](https://docs.python.org/3/library/ast.html#ast.alias)
#[derive(Clone, Debug, PartialEq)]
pub struct Alias {
    pub range: TextRange,
    pub name: Identifier,
    pub asname: Option<Identifier>,
}

impl Node for Alias {
    const NAME: &'static str = "alias";
    const FIELD_NAMES: &'static [&'static str] = &["name", "asname"];
}

/// See also [withitem](https://docs.python.org/3/library/ast.html#ast.withitem)
#[derive(Clone, Debug, PartialEq)]
pub struct WithItem {
    pub range: TextRange,
    pub context_expr: Expr,
    pub optional_vars: Option<Box<Expr>>,
}

impl Node for WithItem {
    const NAME: &'static str = "withitem";
    const FIELD_NAMES: &'static [&'static str] = &["context_expr", "optional_vars"];
}

/// See also [match_case](https://docs.python.org/3/library/ast.html#ast.match_case)
#[derive(Clone, Debug, PartialEq)]
pub struct MatchCase {
    pub range: TextRange,
    pub pattern: Pattern,
    pub guard: Option<Box<Expr>>,
    pub body: Vec<Stmt>,
}

impl Node for MatchCase {
    const NAME: &'static str = "match_case";
    const FIELD_NAMES: &'static [&'static str] = &["pattern", "guard", "body"];
}

/// See also [pattern](https://docs.python.org/3/library/ast.html#ast.pattern)
#[derive(Clone, Debug, PartialEq, is_macro::Is)]
pub enum Pattern {
    MatchValue(PatternMatchValue),
    MatchSingleton(PatternMatchSingleton),
    MatchSequence(PatternMatchSequence),
    MatchMapping(PatternMatchMapping),
    MatchClass(PatternMatchClass),
    MatchStar(PatternMatchStar),
    MatchAs(PatternMatchAs),
    MatchOr(PatternMatchOr),
}

/// See also [MatchValue](https://docs.python.org/3/library/ast.html#ast.MatchValue)
#[derive(Clone, Debug, PartialEq)]
pub struct PatternMatchValue {
    pub range: TextRange,
    pub value: Box<Expr>,
}

impl Node for PatternMatchValue {
    const NAME: &'static str = "MatchValue";
    const FIELD_NAMES: &'static [&'static str] = &["value"];
}
impl From<PatternMatchValue> for Pattern {
    fn from(payload: PatternMatchValue) -> Self {
        Pattern::MatchValue(payload)
    }
}
impl From<PatternMatchValue> for Ast {
    fn from(payload: PatternMatchValue) -> Self {
        Pattern::from(payload).into()
    }
}

/// See also [MatchSingleton](https://docs.python.org/3/library/ast.html#ast.MatchSingleton)
#[derive(Clone, Debug, PartialEq)]
pub struct PatternMatchSingleton {
    pub range: TextRange,
    pub value: Constant,
}

impl Node for PatternMatchSingleton {
    const NAME: &'static str = "MatchSingleton";
    const FIELD_NAMES: &'static [&'static str] = &["value"];
}
impl From<PatternMatchSingleton> for Pattern {
    fn from(payload: PatternMatchSingleton) -> Self {
        Pattern::MatchSingleton(payload)
    }
}
impl From<PatternMatchSingleton> for Ast {
    fn from(payload: PatternMatchSingleton) -> Self {
        Pattern::from(payload).into()
    }
}

/// See also [MatchSequence](https://docs.python.org/3/library/ast.html#ast.MatchSequence)
#[derive(Clone, Debug, PartialEq)]
pub struct PatternMatchSequence {
    pub range: TextRange,
    pub patterns: Vec<Pattern>,
}

impl Node for PatternMatchSequence {
    const NAME: &'static str = "MatchSequence";
    const FIELD_NAMES: &'static [&'static str] = &["patterns"];
}
impl From<PatternMatchSequence> for Pattern {
    fn from(payload: PatternMatchSequence) -> Self {
        Pattern::MatchSequence(payload)
    }
}
impl From<PatternMatchSequence> for Ast {
    fn from(payload: PatternMatchSequence) -> Self {
        Pattern::from(payload).into()
    }
}

/// See also [MatchMapping](https://docs.python.org/3/library/ast.html#ast.MatchMapping)
#[derive(Clone, Debug, PartialEq)]
pub struct PatternMatchMapping {
    pub range: TextRange,
    pub keys: Vec<Expr>,
    pub patterns: Vec<Pattern>,
    pub rest: Option<Identifier>,
}

impl Node for PatternMatchMapping {
    const NAME: &'static str = "MatchMapping";
    const FIELD_NAMES: &'static [&'static str] = &["keys", "patterns", "rest"];
}
impl From<PatternMatchMapping> for Pattern {
    fn from(payload: PatternMatchMapping) -> Self {
        Pattern::MatchMapping(payload)
    }
}
impl From<PatternMatchMapping> for Ast {
    fn from(payload: PatternMatchMapping) -> Self {
        Pattern::from(payload).into()
    }
}

/// See also [MatchClass](https://docs.python.org/3/library/ast.html#ast.MatchClass)
#[derive(Clone, Debug, PartialEq)]
pub struct PatternMatchClass {
    pub range: TextRange,
    pub cls: Box<Expr>,
    pub patterns: Vec<Pattern>,
    pub kwd_attrs: Vec<Identifier>,
    pub kwd_patterns: Vec<Pattern>,
}

impl Node for PatternMatchClass {
    const NAME: &'static str = "MatchClass";
    const FIELD_NAMES: &'static [&'static str] = &["cls", "patterns", "kwd_attrs", "kwd_patterns"];
}
impl From<PatternMatchClass> for Pattern {
    fn from(payload: PatternMatchClass) -> Self {
        Pattern::MatchClass(payload)
    }
}
impl From<PatternMatchClass> for Ast {
    fn from(payload: PatternMatchClass) -> Self {
        Pattern::from(payload).into()
    }
}

/// See also [MatchStar](https://docs.python.org/3/library/ast.html#ast.MatchStar)
#[derive(Clone, Debug, PartialEq)]
pub struct PatternMatchStar {
    pub range: TextRange,
    pub name: Option<Identifier>,
}

impl Node for PatternMatchStar {
    const NAME: &'static str = "MatchStar";
    const FIELD_NAMES: &'static [&'static str] = &["name"];
}
impl From<PatternMatchStar> for Pattern {
    fn from(payload: PatternMatchStar) -> Self {
        Pattern::MatchStar(payload)
    }
}
impl From<PatternMatchStar> for Ast {
    fn from(payload: PatternMatchStar) -> Self {
        Pattern::from(payload).into()
    }
}

/// See also [MatchAs](https://docs.python.org/3/library/ast.html#ast.MatchAs)
#[derive(Clone, Debug, PartialEq)]
pub struct PatternMatchAs {
    pub range: TextRange,
    pub pattern: Option<Box<Pattern>>,
    pub name: Option<Identifier>,
}

impl Node for PatternMatchAs {
    const NAME: &'static str = "MatchAs";
    const FIELD_NAMES: &'static [&'static str] = &["pattern", "name"];
}
impl From<PatternMatchAs> for Pattern {
    fn from(payload: PatternMatchAs) -> Self {
        Pattern::MatchAs(payload)
    }
}
impl From<PatternMatchAs> for Ast {
    fn from(payload: PatternMatchAs) -> Self {
        Pattern::from(payload).into()
    }
}

/// See also [MatchOr](https://docs.python.org/3/library/ast.html#ast.MatchOr)
#[derive(Clone, Debug, PartialEq)]
pub struct PatternMatchOr {
    pub range: TextRange,
    pub patterns: Vec<Pattern>,
}

impl Node for PatternMatchOr {
    const NAME: &'static str = "MatchOr";
    const FIELD_NAMES: &'static [&'static str] = &["patterns"];
}
impl From<PatternMatchOr> for Pattern {
    fn from(payload: PatternMatchOr) -> Self {
        Pattern::MatchOr(payload)
    }
}
impl From<PatternMatchOr> for Ast {
    fn from(payload: PatternMatchOr) -> Self {
        Pattern::from(payload).into()
    }
}

impl Node for Pattern {
    const NAME: &'static str = "pattern";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

/// See also [type_ignore](https://docs.python.org/3/library/ast.html#ast.type_ignore)
#[derive(Clone, Debug, PartialEq, is_macro::Is)]
pub enum TypeIgnore {
    TypeIgnore(TypeIgnoreTypeIgnore),
}

/// See also [TypeIgnore](https://docs.python.org/3/library/ast.html#ast.TypeIgnore)
#[derive(Clone, Debug, PartialEq)]
pub struct TypeIgnoreTypeIgnore {
    pub range: TextRange,
    pub lineno: Int,
    pub tag: String,
}

impl Node for TypeIgnoreTypeIgnore {
    const NAME: &'static str = "TypeIgnore";
    const FIELD_NAMES: &'static [&'static str] = &["lineno", "tag"];
}
impl From<TypeIgnoreTypeIgnore> for TypeIgnore {
    fn from(payload: TypeIgnoreTypeIgnore) -> Self {
        TypeIgnore::TypeIgnore(payload)
    }
}
impl From<TypeIgnoreTypeIgnore> for Ast {
    fn from(payload: TypeIgnoreTypeIgnore) -> Self {
        TypeIgnore::from(payload).into()
    }
}

impl Node for TypeIgnore {
    const NAME: &'static str = "type_ignore";
    const FIELD_NAMES: &'static [&'static str] = &[];
}

/// See also [decorator](https://docs.python.org/3/library/ast.html#ast.decorator)
#[derive(Clone, Debug, PartialEq)]
pub struct Decorator {
    pub range: TextRange,
    pub expression: Expr,
}

impl Node for Decorator {
    const NAME: &'static str = "decorator";
    const FIELD_NAMES: &'static [&'static str] = &["expression"];
}

/// An alternative type of AST `arguments`. This is parser-friendly and human-friendly definition of function arguments.
/// This form also has advantage to implement pre-order traverse.
/// `defaults` and `kw_defaults` fields are removed and the default values are placed under each `arg_with_default` typed argument.
/// `vararg` and `kwarg` are still typed as `arg` because they never can have a default value.
///
/// The matching Python style AST type is [PythonArguments]. While [PythonArguments] has ordered `kwonlyargs` fields by
/// default existence, [Arguments] has location-ordered kwonlyargs fields.
///
/// NOTE: This type is different from original Python AST.

#[derive(Clone, Debug, PartialEq)]
pub struct Arguments {
    pub range: TextRange,
    pub posonlyargs: Vec<ArgWithDefault>,
    pub args: Vec<ArgWithDefault>,
    pub vararg: Option<Box<Arg>>,
    pub kwonlyargs: Vec<ArgWithDefault>,
    pub kwarg: Option<Box<Arg>>,
}

impl Node for Arguments {
    const NAME: &'static str = "alt:arguments";
    const FIELD_NAMES: &'static [&'static str] =
        &["posonlyargs", "args", "vararg", "kwonlyargs", "kwarg"];
}

/// An alternative type of AST `arg`. This is used for each function argument that might have a default value.
/// Used by `Arguments` original type.
///
/// NOTE: This type is different from original Python AST.

#[derive(Clone, Debug, PartialEq)]
pub struct ArgWithDefault {
    pub range: TextRange,
    pub def: Arg,
    pub default: Option<Box<Expr>>,
}

impl Node for ArgWithDefault {
    const NAME: &'static str = "arg_with_default";
    const FIELD_NAMES: &'static [&'static str] = &["def", "default"];
}

pub type Suite = Vec<Stmt>;

impl CmpOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            CmpOp::Eq => "==",
            CmpOp::NotEq => "!=",
            CmpOp::Lt => "<",
            CmpOp::LtE => "<=",
            CmpOp::Gt => ">",
            CmpOp::GtE => ">=",
            CmpOp::Is => "is",
            CmpOp::IsNot => "is not",
            CmpOp::In => "in",
            CmpOp::NotIn => "not in",
        }
    }
}

impl Arguments {
    pub fn empty(range: TextRange) -> Self {
        Self {
            range,
            posonlyargs: Vec::new(),
            args: Vec::new(),
            vararg: None,
            kwonlyargs: Vec::new(),
            kwarg: None,
        }
    }
}

#[allow(clippy::borrowed_box)] // local utility
fn clone_boxed_expr(expr: &Box<Expr>) -> Box<Expr> {
    let expr: &Expr = expr.as_ref();
    Box::new(expr.clone())
}

impl ArgWithDefault {
    pub fn as_arg(&self) -> &Arg {
        &self.def
    }

    pub fn to_arg(&self) -> (Arg, Option<Box<Expr>>) {
        let ArgWithDefault {
            range: _,
            def,
            default,
        } = self;
        (def.clone(), default.as_ref().map(clone_boxed_expr))
    }
    pub fn into_arg(self) -> (Arg, Option<Box<Expr>>) {
        let ArgWithDefault {
            range: _,
            def,
            default,
        } = self;
        (def, default)
    }
}

impl Arguments {
    pub fn defaults(&self) -> impl std::iter::Iterator<Item = &Expr> {
        self.posonlyargs
            .iter()
            .chain(self.args.iter())
            .filter_map(|arg| arg.default.as_ref().map(|e| e.as_ref()))
    }

    #[allow(clippy::type_complexity)]
    pub fn split_kwonlyargs(&self) -> (Vec<&Arg>, Vec<(&Arg, &Expr)>) {
        let mut args = Vec::new();
        let mut with_defaults = Vec::new();
        for arg in self.kwonlyargs.iter() {
            if let Some(ref default) = arg.default {
                with_defaults.push((arg.as_arg(), &**default));
            } else {
                args.push(arg.as_arg());
            }
        }
        (args, with_defaults)
    }
}
