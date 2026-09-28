use std::fmt::Write;

use crate::parser::{BinaryOp, Expr, Program, Stmt};

struct DotGenerator {
    next_id: usize,
    output: String,
}

impl DotGenerator {
    fn new() -> Self {
        let mut genr = DotGenerator {
            next_id: 0,
            output: String::new(),
        };
        genr.output.push_str("digraph AST {\n");
        genr.output.push_str("    node [shape=box, style=rounded, fontname=\"Courier\"];\n");
        genr
    }

    fn alloc_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn add_node(&mut self, id: usize, label: &str) {
        let escaped = label.replace('"', "\\\"");
        let _ = writeln!(self.output, "    node_{} [label=\"{}\"];", id, escaped);
    }

    fn add_edge(&mut self, from: usize, to: usize, label: Option<&str>) {
        match label {
            Some(lbl) => {
                let _ = writeln!(self.output, "    node_{} -> node_{} [label=\"{}\"];", from, to, lbl);
            }
            None => {
                let _ = writeln!(self.output, "    node_{} -> node_{};", from, to);
            }
        }
    }

    fn gen_expr(&mut self, expr: &Expr) -> usize {
        let id = self.alloc_id();
        match expr {
            Expr::Int(n) => {
                self.add_node(id, &format!("Int({})", n));
            }
            Expr::Variable(name) => {
                self.add_node(id, &format!("Var({})", name));
            }
            Expr::Binary { op, left, right } => {
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                    BinaryOp::Modulo => "%",
                    BinaryOp::Equal => "==",
                    BinaryOp::NotEqaul => "!=",
                    BinaryOp::LessThan => "<",
                    BinaryOp::LessThanEqual => "<=",
                    BinaryOp::GreaterThan => ">",
                    BinaryOp::GreaterThanEqual => ">=",
                };
                self.add_node(id, &format!("Op({})", op_str));

                let left_id = self.gen_expr(left);
                let right_id = self.gen_expr(right);

                self.add_edge(id, left_id, Some("L"));
                self.add_edge(id, right_id, Some("R"));
            }
        }
        id
    }

    fn gen_stmt(&mut self, stmt: &Stmt) -> usize {
        let id = self.alloc_id();
        match stmt {
            Stmt::Decl { var_type, identifier, init } => {
                self.add_node(id, &format!("Decl({:?} {})", var_type, identifier));
                if let Some(init_expr) = init {
                    let expr_id = self.gen_expr(init_expr);
                    self.add_edge(id, expr_id, Some("init"));
                }
            }
            Stmt::Assign { identifier, value } => {
                self.add_node(id, &format!("Assign({})", identifier));
                let val_id = self.gen_expr(value);
                self.add_edge(id, val_id, Some("val"));
            }
            Stmt::Print(expr) => {
                self.add_node(id, "Print");
                let expr_id = self.gen_expr(expr);
                self.add_edge(id, expr_id, None);
            }
            Stmt::If { condition, then_branch, else_branch } => {
                self.add_node(id, "If");
                let cond_id = self.gen_expr(condition);
                self.add_edge(id, cond_id, Some("cond"));

                let then_id = self.alloc_id();
                self.add_node(then_id, "Then");
                self.add_edge(id, then_id, None);
                for s in then_branch {
                    let s_id = self.gen_stmt(s);
                    self.add_edge(then_id, s_id, None);
                }

                if let Some(else_stmts) = else_branch {
                    let else_id = self.alloc_id();
                    self.add_node(else_id, "Else");
                    self.add_edge(id, else_id, None);
                    for s in else_stmts {
                        let s_id = self.gen_stmt(s);
                        self.add_edge(else_id, s_id, None);
                    }
                }
            }
            Stmt::While { condition, body } => {
                self.add_node(id, "While");
                let cond_id = self.gen_expr(condition);
                self.add_edge(id, cond_id, Some("cond"));

                let body_id = self.alloc_id();
                self.add_node(body_id, "Body");
                self.add_edge(id, body_id, None);
                for s in body {
                    let s_id = self.gen_stmt(s);
                    self.add_edge(body_id, s_id, None);
                }
            }
        }
        id
    }

    fn finish(mut self) -> String {
        self.output.push_str("}\n");
        self.output
    }
}

pub fn ast_to_dot(program: &Program) -> String {
    let mut genr = DotGenerator::new();
    let root_id = genr.alloc_id();
    genr.add_node(root_id, "Program");

    for stmt in program {
        let stmt_id = genr.gen_stmt(stmt);
        genr.add_edge(root_id, stmt_id, None);
    }

    genr.finish()
}