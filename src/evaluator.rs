use crate::ast::{BinaryOperator, ExprKind, Expression, Literal, Program, StmtKind, UnaryOperator};

#[allow(dead_code)]
pub fn eval_program(program: &Program) -> isize {
    program
        .statements
        .iter()
        .filter_map(|stmt| match &stmt.kind {
            StmtKind::VarDecl(decl) => Some(eval_expr(&decl.initializer)),
            StmtKind::ExprStmt(expr) => Some(eval_expr(expr)),
            StmtKind::Return(Some(expr)) => Some(eval_expr(expr)),
            _ => None,
        })
        .next_back()
        .unwrap_or(0)
}

#[allow(dead_code)]
pub fn eval_expr(expr: &Expression) -> isize {
    match &expr.kind {
        ExprKind::Literal(Literal::Int(n)) => *n,
        ExprKind::Literal(Literal::Str(_)) => 0,
        ExprKind::Ident(_) => 0,
        ExprKind::UnaryOp(UnaryOperator::Neg, sub_expr) => -eval_expr(sub_expr),
        ExprKind::BinaryOp(lhs, op, rhs) => {
            let lhs_val = eval_expr(lhs);
            let rhs_val = eval_expr(rhs);
            match op {
                BinaryOperator::Add => lhs_val + rhs_val,
                BinaryOperator::Sub => lhs_val - rhs_val,
                BinaryOperator::Mul => lhs_val * rhs_val,
                BinaryOperator::Div => lhs_val / rhs_val,
            }
        }
        ExprKind::Call(_name, _args) => 0,
    }
}
