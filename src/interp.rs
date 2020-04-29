use crate::builtin::B;
use crate::tast::{Condition, Ex, NameDef, NameRef};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    mapping: HashMap<NameDef, Ex>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            mapping: HashMap::new(),
        }
    }

    pub fn bind(&self, name: &NameDef, ex: Ex) -> Env {
        let mut new = self.clone();
        new.mapping.insert(name.clone(), ex);
        new
    }

    pub fn bind_many(&self, names: &[&NameDef], exs: &[Ex]) -> Env {
        assert_eq!(names.len(), exs.len());
        let mut new = self.clone();
        for (num, &n) in names.iter().enumerate() {
            new.mapping.insert(n.clone(), exs[num].clone());
        }
        new
    }

    pub fn find(&self, name: &NameDef) -> Option<Ex> {
        self.mapping.get(name).cloned()
    }
}

pub fn eval_ex(ex: Ex, env: &Env) -> Ex {
    use Ex::*;

    match ex {
        Bind(b) => {
            let new_env = env.bind(&b.t.name.t, b.t.ex.clone());
            eval_ex(b.t.ex.clone(), &new_env)
        }
        Lam(_) => ex,
        Ap(a) => {
            let ex = eval_ex(a.t.ex.clone(), env);
            let args: Vec<_> =
                a.t.args
                    .iter()
                    .map(|arg| eval_ex(arg.clone(), env))
                    .collect();

            match ex {
                Ref(n) => eval_builtin(&n, &args, env),
                Lam(l) => {
                    let bound: Vec<_> = l.t.bound.iter().map(|n| n.t.as_ref()).collect();
                    let new_env = env.bind_many(&bound, &args);
                    eval_ex(l.t.body.clone(), &new_env)
                }
                other => panic!("Unexpected application expression: {}", other),
            }
        }
        Cond(c) => {
            let Condition { pred, then, els } = c.t.as_ref();
            let pred = eval_ex(pred.clone(), env);
            if let ConstBool(b) = pred {
                if b {
                    eval_ex(then.clone(), env)
                } else {
                    eval_ex(els.clone(), env)
                }
            } else {
                panic!("Unexpected boolean expression: {}", pred);
            }
        }
        Ref(n) => {
            // TODO: properly handle missing and builtin
            match n {
                NameRef::Builtin(bn) => bn.into(),
                NameRef::User(un) => env
                    .find(&un.t)
                    .unwrap_or_else(|| panic!("Unkown name: {}", un)),
            }
        }
        ConstInt(_) => ex,
        ConstBool(_) => ex,
    }
}

pub fn eval_builtin(name: &NameRef, args: &[Ex], env: &Env) -> Ex {
    use Ex::*;

    if let NameRef::Builtin(name) = *name {
        if name.t == B.plus().t {
            let lhs = &args[0];
            let rhs = &args[1];

            match (lhs, rhs) {
                (ConstInt(n1), ConstInt(n2)) => return ConstInt(n1 + n2),
                _ => panic!("Cannot add \n1: {}\n2: {}", lhs, rhs),
            }
        }

        if name.t == B.minus().t {
            let lhs = &args[0];
            let rhs = &args[1];

            match (lhs, rhs) {
                (ConstInt(n1), ConstInt(n2)) => return ConstInt(n1 - n2),
                _ => panic!("Cannot subtract \n1: {}\n2: {}", lhs, rhs),
            }
        }

        if name.t == B.less().t {
            let lhs = &args[0];
            let rhs = &args[1];

            match (lhs, rhs) {
                (ConstInt(n1), ConstInt(n2)) => return ConstBool(n1 < n2),
                _ => panic!("Cannot compare \n1: {}\n2: {}", lhs, rhs),
            }
        }

        panic!("Unknown builtin name: {}", name.t)
    } else {
        panic!("Expected a builtin name, got user-defined: {}", name)
    }
}
