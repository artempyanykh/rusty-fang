use std::fmt::Display;
use std::fmt::{self, Debug};
use std::sync::Arc;
use tree_sitter::Point;
use tree_sitter::{Language, Node, Parser, Range, Tree};

extern "C" {
    fn tree_sitter_fang() -> Language;
}

pub fn parse_tree(code: &str) -> Tree {
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_fang() };
    parser.set_language(language).unwrap();

    let tree = parser.parse(code, None).unwrap();
    tree
}

#[derive(Debug)]
pub struct ParsingError {
    msg: String,
    loc: Loc,
    offset: usize,
}

pub struct WithCode<'a, 'b: 'a, T> {
    code: &'a str,
    t: &'b T,
}

impl<'a, 'b, T> WithCode<'a, 'b, T> {
    pub fn new(code: &'a str, t: &'b T) -> WithCode<'a, 'b, T> {
        WithCode { code, t }
    }
}

impl ParsingError {
    fn new(msg: String, range: Range) -> ParsingError {
        ParsingError {
            msg,
            loc: range.start_point.into(),
            offset: range.start_byte,
        }
    }
}

impl<'a, 'b> Display for WithCode<'a, 'b, ParsingError> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        let prev_newline = self.code[..self.t.offset].rfind("\n");
        let next_newline = self.code[self.t.offset..]
            .find("\n")
            .map(|pos| pos + self.t.offset);

        let line = match (prev_newline, next_newline) {
            (Some(p), Some(n)) => self.code[p..n].trim(),
            (Some(p), None) => self.code[p..].trim(),
            (None, Some(n)) => self.code[..n].trim(),
            (None, None) => self.code.trim(),
        };

        let row_num_width = (self.t.loc.row + 1).to_string().len();
        let fill = row_num_width + 3;

        writeln!(
            f,
            "{:>fill$} {}:{}",
            "-->",
            self.t.loc.row + 1,
            self.t.loc.col + 1,
            fill = fill
        )?;
        writeln!(
            f,
            " {:>fill$} | {}",
            self.t.loc.row + 1,
            line,
            fill = row_num_width
        )?;
        writeln!(f, "{:>fill$} ^", " ", fill = fill + self.t.loc.col)?;
        writeln!(
            f,
            "{:>fill$} {}",
            " ",
            self.t.msg,
            fill = fill + self.t.loc.col
        )
    }
}

type Result<A> = std::result::Result<A, Vec<ParsingError>>;

pub fn parse(code: &str) -> Result<CompilationUnit> {
    let tree = parse_tree(code);
    let root = tree.root_node();
    let mut errors = vec![];
    let mut expressions = vec![];

    let grammar_errors = collect_error_nodes(code, root);

    if grammar_errors.is_empty() {
        for child in root.named_children(&mut root.walk()) {
            match parse_ex(code, child) {
                Ok(ex) => expressions.push(ex),
                Err(e) => errors.extend(e),
            }
        }
    } else {
        errors.extend(grammar_errors);
    }

    if errors.is_empty() {
        Ok(CompilationUnit { nodes: expressions })
    } else {
        Err(errors)
    }
}

fn collect_error_nodes(code: &str, node: Node<'_>) -> Vec<ParsingError> {
    // Sometimes ERROR node can have complex structure inside with additional
    // ERROR nodes. Therefore we first try to get the more specific issue found
    // inside child nodes, and if nothing found return a more generic error from
    // current node.
    let this_nodes_error = if node.is_error() {
        let error = ParsingError {
            msg: format!("Unexpected token"),
            loc: node.range().start_point.into(),
            offset: node.range().start_byte,
        };
        vec![error]
    } else if node.is_missing() {
        let error = ParsingError {
            msg: format!("Missing token"),
            loc: node.range().start_point.into(),
            offset: node.range().start_byte,
        };
        vec![error]
    } else {
        vec![]
    };

    let mut all_child_errors = vec![];
    node.children(&mut node.walk())
        .map(|child| collect_error_nodes(code, child))
        .for_each(|child_errors| all_child_errors.extend(child_errors));

    if all_child_errors.is_empty() {
        this_nodes_error
    } else {
        all_child_errors
    }
}

pub fn parse_ex(code: &str, node: Node<'_>) -> Result<Ex> {
    if node.is_error() {
        panic!("Not implemented for error node:\n{:#?}", node);
    }

    if node.is_missing() {
        panic!("Not implemented for missing node:\n{:#?}", node);
    }

    match node.kind() {
        "const_int" => parse_int(code, node.range()).map(|x| x.into()),
        "const_bool" => Ok(parse_bool(code, node.range()).into()),
        "identifier" => Ok(parse_identifier(code, node.range()).into()),
        "infix_ex" => parse_infix_ex(code, node).map(|x| x.into()),
        "prefix_ex" => parse_prefix_ex(code, node).map(|x| x.into()),
        "binding" => parse_binding(code, node).map(|x| x.into()),
        "let" => parse_let(code, node).map(|x| x.into()),
        "lambda" => parse_lambda(code, node).map(|x| x.into()),
        "ap" => parse_ap(code, node).map(|x| x.into()),
        "cond" => parse_cond(code, node).map(|x| x.into()),
        other => {
            let err_msg = format!("Not implemented for node: {}", other);
            panic!(err_msg)
        }
    }
}

pub fn parse_int(code: &str, range: Range) -> Result<N<i64>> {
    let int_str: String = code[range.start_byte..range.end_byte].to_string();
    let int_val = int_str.replace("_", "").parse::<i64>();
    match int_val {
        Ok(val) => Ok(N::new(val, range)),
        Err(e) => Err(vec![ParsingError::new(format!("{}", e), range)]),
    }
}

pub fn parse_bool(code: &str, range: Range) -> N<bool> {
    let bool_str = &code[range.start_byte..range.end_byte];
    match bool_str {
        "True" => N::new(true, range).into(),
        "False" => N::new(false, range).into(),
        other => panic!("Unexpected bool const: {}", other),
    }
}

pub fn parse_identifier(code: &str, range: Range) -> N<Ident> {
    let ident_name = &code[range.start_byte..range.end_byte];
    N::new(Ident(ident_name.to_string()), range)
}

pub fn parse_infix_ex(code: &str, node: Node<'_>) -> Result<N<InfixEx>> {
    let op_node = require_child_by_field_name(node, "op");
    let lhs_node = require_child_by_field_name(node, "lhs");
    let rhs_node = require_child_by_field_name(node, "rhs");

    let op = parse_op(code, op_node.range());
    let lhs = parse_ex(code, lhs_node);
    let rhs = parse_ex(code, rhs_node);

    combine_results_2(lhs, rhs).map(|(lhs, rhs)| {
        N::new(
            InfixEx {
                op: op,
                lhs: lhs,
                rhs: rhs,
            },
            node.range(),
        )
    })
}

pub fn parse_prefix_ex(code: &str, node: Node<'_>) -> Result<N<PrefixEx>> {
    let op_node = require_child_by_field_name(node, "op");
    let body_node = require_child_by_field_name(node, "body");
    let op = parse_op(code, op_node.range());
    let body = parse_ex(code, body_node)?;
    Ok(N::new(PrefixEx { op, body }, node.range()))
}

pub fn parse_let(code: &str, node: Node<'_>) -> Result<N<LetEx>> {
    let bindings: Vec<_> = node
        .children_by_field_name("bindings", &mut node.walk())
        .filter(|n| n.is_named())
        .map(|n| parse_binding(code, n))
        .collect();
    let bindings = combine_results_n(bindings);
    let body_node = require_child_by_field_name(node, "body");
    let body = parse_ex(code, body_node);

    combine_results_2(bindings, body)
        .map(|(bindings, body)| N::new(LetEx { bindings, body }, node.range()))
}

pub fn parse_binding(code: &str, node: Node<'_>) -> Result<N<Bind>> {
    let lhs_node = require_child_by_field_name(node, "lhs");
    let lhs = parse_identifier(code, lhs_node.range());

    let params: Vec<_> = node
        .children_by_field_name("params", &mut node.walk())
        .filter(|n| n.is_named())
        .map(|n| parse_identifier(code, n.range()))
        .collect();

    let rhs_node = require_child_by_field_name(node, "rhs");
    let rhs = parse_ex(code, rhs_node)?;

    Ok(N::new(Bind { lhs, params, rhs }, node.range()))
}

pub fn parse_lambda(code: &str, node: Node<'_>) -> Result<N<Lam>> {
    let params: Vec<_> = node
        .children_by_field_name("params", &mut node.walk())
        .filter(|n| n.is_named())
        .map(|n| parse_identifier(code, n.range()))
        .collect();
    let body_node = require_child_by_field_name(node, "body");
    let body = parse_ex(code, body_node)?;
    Ok(N::new(Lam { params, body }, node.range()))
}

pub fn parse_ap(code: &str, node: Node<'_>) -> Result<N<Ap>> {
    let receiver_node = require_child_by_field_name(node, "receiver");
    let receiver = parse_ex(code, receiver_node);
    let arguments: Vec<_> = node
        .children_by_field_name("arguments", &mut node.walk())
        .filter(|n| n.is_named())
        .map(|n| parse_ex(code, n))
        .collect();
    let arguments = combine_results_n(arguments);

    combine_results_2(receiver, arguments)
        .map(|(receiver, args)| N::new(Ap { receiver, args }, node.range()))
}

pub fn parse_cond(code: &str, node: Node<'_>) -> Result<N<Cond>> {
    let pred_node = require_child_by_field_name(node, "pred");
    let then_node = require_child_by_field_name(node, "then");
    let els_node = require_child_by_field_name(node, "else");

    let pred = parse_ex(code, pred_node);
    let then = parse_ex(code, then_node);
    let els = parse_ex(code, els_node);

    combine_results_3(pred, then, els)
        .map(|(pred, then, els)| N::new(Cond { pred, then, els }, node.range()))
}

pub fn require_child_by_field_name<'tree>(node: Node<'tree>, field: &str) -> Node<'tree> {
    node.child_by_field_name(field).unwrap_or_else(|| {
        panic!(
            "Required field {} is missing in node {}:\n{}",
            field,
            node.kind(),
            node.to_sexp()
        )
    })
}

pub fn combine_results_2<A, B, E>(
    a: std::result::Result<A, Vec<E>>,
    b: std::result::Result<B, Vec<E>>,
) -> std::result::Result<(A, B), Vec<E>> {
    match (a, b) {
        (Ok(a), Ok(b)) => Ok((a, b)),
        (Err(mut ea), Err(eb)) => {
            ea.extend(eb);
            Err(ea)
        }
        (Err(ea), _) => Err(ea),
        (_, Err(eb)) => Err(eb),
    }
}

pub fn combine_results_3<A, B, C, E>(
    a: std::result::Result<A, Vec<E>>,
    b: std::result::Result<B, Vec<E>>,
    c: std::result::Result<C, Vec<E>>,
) -> std::result::Result<(A, B, C), Vec<E>> {
    match combine_results_2(combine_results_2(a, b), c) {
        Ok(((a, b), c)) => Ok((a, b, c)),
        Err(e) => Err(e),
    }
}

pub fn combine_results_n<A, E>(
    results: Vec<std::result::Result<A, Vec<E>>>,
) -> std::result::Result<Vec<A>, Vec<E>> {
    let mut errs = vec![];
    let mut oks = vec![];

    for result in results {
        match result {
            Ok(r) => oks.push(r),
            Err(e) => errs.extend(e),
        }
    }

    if errs.is_empty() {
        Ok(oks)
    } else {
        Err(errs)
    }
}

pub fn parse_op(code: &str, range: Range) -> N<Operator> {
    let op_str = &code[range.start_byte..range.end_byte];
    let op = Operator(op_str.to_string());
    N::new(op, range)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Loc {
    row: usize,
    col: usize,
}

impl From<Point> for Loc {
    fn from(point: Point) -> Self {
        Self {
            row: point.row,
            col: point.column,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Span<T> {
    start: T,
    end: T,
}

impl From<Range> for Span<usize> {
    fn from(r: Range) -> Self {
        Self {
            start: r.start_byte,
            end: r.end_byte,
        }
    }
}

impl From<Range> for Span<Loc> {
    fn from(r: Range) -> Self {
        Self {
            start: r.start_point.into(),
            end: r.end_point.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CompilationUnit {
    nodes: Vec<Ex>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct N<T> {
    t: Arc<T>,
    offset_span: Span<usize>,
    loc_span: Span<Loc>,
}

impl<T> N<T> {
    fn new(t: T, range: Range) -> N<T> {
        Self {
            t: Arc::new(t),
            offset_span: range.into(),
            loc_span: range.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq, Eq)]
pub struct Operator(pub String);

#[derive(Debug, PartialEq, Eq)]
pub struct InfixEx {
    op: N<Operator>,
    lhs: Ex,
    rhs: Ex,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrefixEx {
    op: N<Operator>,
    body: Ex,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LetEx {
    bindings: Vec<N<Bind>>,
    body: Ex,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bind {
    lhs: N<Ident>,
    params: Vec<N<Ident>>,
    rhs: Ex,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Lam {
    params: Vec<N<Ident>>,
    body: Ex,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ap {
    receiver: Ex,
    args: Vec<Ex>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Cond {
    pred: Ex,
    then: Ex,
    els: Ex,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ex {
    Application(N<Ap>),
    Binding(N<Bind>),
    Condition(N<Cond>),
    ConstBool(N<bool>),
    ConstInt(N<i64>),
    Identifier(N<Ident>),
    Infix(N<InfixEx>),
    Lambda(N<Lam>),
    Let(N<LetEx>),
    Prefix(N<PrefixEx>),
}

impl From<N<PrefixEx>> for Ex {
    fn from(v: N<PrefixEx>) -> Self {
        Ex::Prefix(v)
    }
}

impl From<N<LetEx>> for Ex {
    fn from(v: N<LetEx>) -> Self {
        Ex::Let(v)
    }
}

impl From<N<Lam>> for Ex {
    fn from(v: N<Lam>) -> Self {
        Ex::Lambda(v)
    }
}

impl From<N<InfixEx>> for Ex {
    fn from(v: N<InfixEx>) -> Self {
        Ex::Infix(v)
    }
}

impl From<N<Ident>> for Ex {
    fn from(v: N<Ident>) -> Self {
        Ex::Identifier(v)
    }
}

impl From<N<i64>> for Ex {
    fn from(v: N<i64>) -> Self {
        Ex::ConstInt(v)
    }
}

impl From<N<bool>> for Ex {
    fn from(v: N<bool>) -> Self {
        Ex::ConstBool(v)
    }
}

impl From<N<Cond>> for Ex {
    fn from(v: N<Cond>) -> Self {
        Ex::Condition(v)
    }
}

impl From<N<Bind>> for Ex {
    fn from(v: N<Bind>) -> Self {
        Ex::Binding(v)
    }
}

impl From<N<Ap>> for Ex {
    fn from(v: N<Ap>) -> Self {
        Ex::Application(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn t_parse_int() {
        let r = parse("a + b * c").unwrap();
        assert_eq!(r, CompilationUnit { nodes: vec![] });

        // let r = parse("1_000_000").unwrap();
        // assert_eq!(format!("{:#?}", r), "1000000");

        // let r = parse("9_999_999_999_999_999_999").unwrap();
        // assert_eq!(format!("{:#?}", r), "1");
        // let r = parse("_-").unwrap();
        // assert_eq!(format!("{:#?}", r), "1");
    }
}
