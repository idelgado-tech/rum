use crate::ast::AstCdms;
use crate::ast::AstExp;
use crate::ast::Oprim;

use crate::rum_type::Type;

use std::collections::HashMap;

impl AstExp {
    pub fn type_check(&self, type_cache: &HashMap<String, Type>) -> Type {
        println!("\nInto Expr eval");
        println!("expr : {:?}", self);
        println!("type_cache {:?}", type_cache);

        use AstExp::*;

        match self {
            ASTInt(n) => Type::Int,
            ASTBool(b) => Type::Bool,
            ASTUnPrim(_, exp) => {
                let exp_type = exp.type_check(type_cache);
                if exp_type == Type::Bool {
                    Type::Bool
                } else {
                    Type::TypeError(format!("Not primitive function require a boolean argument"))
                }
            }
            ASTBinPrim(op, e1, e2) => {
                let expr1 = e1.type_check(type_cache);
                let expr2 = e2.type_check(type_cache);

                if expr1.is_err() || expr2.is_err() {
                    return Type::TypeError(format!("TODO"));
                }

                match op {
                    Oprim::Add => {
                        if expr1 != Type::Int || expr2 != Type::Int {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Mul => {
                        if expr1 != Type::Int || expr2 != Type::Int {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Div => {
                        if expr1 != Type::Int || expr2 != Type::Int {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Sub => {
                        if expr1 != Type::Int || expr2 != Type::Int {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Eq => {
                        if expr1 != Type::Bool || expr2 != Type::Bool {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Lt => {
                        if expr1 != Type::Bool || expr2 != Type::Bool {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::And => {
                        if expr1 != Type::Bool || expr2 != Type::Bool {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Or => {
                        if expr1 != Type::Bool || expr2 != Type::Bool {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                }
            }
            ASTIf(condition, then, _else) => {
                let type_condition = condition.type_check(type_cache);
                let type_then = then.type_check(type_cache);
                let type_else = _else.type_check(type_cache);

                if type_condition != Type::Bool {
                    return Type::TypeError(format!("TODO"));
                }

                if type_then != type_else {
                    return Type::TypeError(format!("TODO"));
                }

                type_then
            }

            ASTIdent(x) => type_cache.get(x).expect("Not in environment").clone(),

            ASTApp(e, args) => {
                let func_type = type_cache.get(e).expect("Not in environment").clone();
                let type_args = Vec::with_capacity(args.len());

                for arg in args {
                    type_args.push(arg.type_check(type_cache));
                }

                Type::check_fun(func_type, type_args)
            }

            ASTAbs(args, e) => {
                let mut abs_args = Vec::new();

                for arg in args {
                    abs_args.push(arg.ident.clone());
                }
                Value::Fermeture(e.clone(), abs_args, env.clone())
            }

            // rum2
            ASTAlloc(e) => match e.eval(&env, mem) {
                Value::Int(n) => mem.allocn(n as usize),
                _ => panic!(format!(" {:?} is not a Number", e)),
            },

            ASTNth(e1, e2) => match e1.eval(&env, mem) {
                Value::Block(adr, n) => match e2.eval(&env, mem) {
                    Value::Int(i) => {
                        if i > n as i64 {
                            panic!("ASTNTH out of the block");
                        }
                        mem.mem[adr + i as usize].clone()
                    }
                    _ => panic!(format!(" {:?} is not a Number", e2)),
                },
                _ => panic!(format!(" {:?} is not a Block", e1)),
            },
            
            ASTLen(e) => match e.eval(&env, mem) {
                Value::Block(_, n) => Value::Int(n as i64),
                _ => panic!(format!(" {:?} is not a Block", e)),
            },
        }
    }
}