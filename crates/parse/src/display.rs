use crate::ast::{expr::ExprNode, stmt::StmtNode};

pub fn print_stmt(node: StmtNode) {
    print_stmt_lvl(node, 0);
}

pub fn indent(lvl: usize) {
    for _ in 0..lvl * 2 {
        // indent 2 spaces
        print!(" ")
    }
}

pub fn print_stmt_lvl(node: StmtNode, lvl: usize) {
    indent(lvl);
    match node {
        StmtNode::Expression { expr } => {
            println!("{:?};", expr);
        }
        StmtNode::Let { target, expr } => {
            println!("let {:?} = {:?};", target, expr);
        }
        StmtNode::Return { expr } => {
            println!("return {:?};", expr);
        }
        StmtNode::Print { expr } => {
            println!("print {:?};", expr);
        }
        StmtNode::For { iter, n_iter, body } => {
            println!("for {} : {:?} {{", iter, n_iter);
            for stmt in body {
                print_stmt_lvl(stmt, lvl + 1);
            }
            indent(lvl);
            println!("}}");
        }
        StmtNode::While { cond, body } => {
            println!("while {:?} {{", cond);
            for stmt in body {
                print_stmt_lvl(stmt, lvl + 1);
            }
            indent(lvl);
            println!("}}");
        }
        StmtNode::If { cond, body } => {
            println!("if {:?} {{", cond);
            for stmt in body {
                print_stmt_lvl(stmt, lvl + 1);
            }
            indent(lvl);
            println!("}}");
        }
        StmtNode::IfElse {
            cond,
            if_body,
            else_body,
        } => {
            println!("if {:?} {{", cond);
            for stmt in if_body {
                print_stmt_lvl(stmt, lvl + 1);
            }
            indent(lvl);

            println!("}} else {{");
            for stmt in else_body {
                print_stmt_lvl(stmt, lvl + 1);
            }
            indent(lvl);
            println!("}}");
        }
        StmtNode::Scope { body } => {
            println!("{{");
            for stmt in body {
                print_stmt_lvl(stmt, lvl + 1);
            }
            indent(lvl);
        }
        StmtNode::FuncDef { name, params, body } => {
            println!("func {} ({}) {{", name, params.join(","));
            for stmt in body {
                print_stmt_lvl(stmt, lvl + 1);
            }
            indent(lvl);
            println!("}}");
        }
        StmtNode::Model { name, fields } => {
            println!("model {} {{", name);
            for (name, typename) in fields {
                indent(lvl + 1);
                println!("{}: {},", name, typename);
            }
            indent(lvl);
            println!("}}");
        }
    }
}
