use crate::tast::{Condition, Ex, NameDef};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Env {
    mapping: HashMap<NameDef, Rc<Ex>>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            mapping: HashMap::new(),
        }
    }

    pub fn bind(&self, name: &NameDef, ex: Rc<Ex>) -> Env {
        let mut new = self.clone();
        new.mapping.insert(name.clone(), ex);
        new
    }

    pub fn bind_many(&self, names: &[&NameDef], exs: &[Rc<Ex>]) -> Env {
        assert_eq!(names.len(), exs.len());
        let mut new = self.clone();
        for (num, &n) in names.iter().enumerate() {
            new.mapping.insert(n.clone(), exs[num].clone());
        }
        new
    }

    pub fn find(&self, name: &NameDef) -> Option<Rc<Ex>> {
        self.mapping.get(name).cloned()
    }
}

pub fn eval_ex(ex: Rc<Ex>, env: &Env) -> Rc<Ex> {
    use Ex::*;

    match ex.as_ref() {
        Bind(b) => {
            let new_env = env.bind(&b.t.name.t, Rc::new(b.t.ex.clone()));
            eval_ex(Rc::new(b.t.ex.clone()), &new_env)
        }
        Lam(_) => ex,
        Ap(a) => {
            let ex = eval_ex(Rc::new(a.t.ex.clone()), env);
            match ex.as_ref() {
                Ref(n) => eval_builtin(&n.t, &a.t.args, env),
                Lam(l) => {
                    let bound: Vec<_> = l.t.bound.iter().map(|n| n.t.as_ref()).collect();
                    let args: Vec<_> = a.t.args.iter().map(|n| Rc::new(n.clone())).collect();
                    let new_env = env.bind_many(&bound, &args);
                    eval_ex(Rc::new(l.t.body.clone()), &new_env)
                }
                other => panic!("Unexpected application expression: {}", other),
            }
        }
        Cond(c) => {
            let Condition { pred, then, els } = c.t.as_ref();
            let pred = eval_ex(Rc::new(pred.clone()), env);
            if let ConstBool(b) = pred.as_ref() {
                if *b {
                    eval_ex(Rc::new(then.clone()), env)
                } else {
                    eval_ex(Rc::new(els.clone()), env)
                }
            } else {
                panic!("Unexpected boolean expression: {}", pred);
            }
        }
        Ref(n) => env.find(&n.t).unwrap_or(ex), // TODO: properly handle missing and builtin
        ConstInt(_) => ex,
        ConstBool(_) => ex,
    }
}

pub fn eval_builtin(name: &NameDef, args: &[Ex], env: &Env) -> Rc<Ex> {
    use Ex::*;

    if name.0 == "plus" {
        let lhs = eval_ex(Rc::new(args[0].clone()), env);
        let rhs = eval_ex(Rc::new(args[0].clone()), env);

        match (lhs.as_ref(), rhs.as_ref()) {
            (ConstInt(n1), ConstInt(n2)) => return Rc::new(ConstInt(n1 + n2)),
            _ => panic!("Cannot add \n1: {}\n2: {}", lhs, rhs),
        }
    }

    if name.0 == "minus" {
        let lhs = eval_ex(Rc::new(args[0].clone()), env);
        let rhs = eval_ex(Rc::new(args[0].clone()), env);

        match (lhs.as_ref(), rhs.as_ref()) {
            (ConstInt(n1), ConstInt(n2)) => return Rc::new(ConstInt(n1 - n2)),
            _ => panic!("Cannot subtract \n1: {}\n2: {}", lhs, rhs),
        }
    }

    if name.0 == "less" {
        let lhs = eval_ex(Rc::new(args[0].clone()), env);
        let rhs = eval_ex(Rc::new(args[0].clone()), env);

        match (lhs.as_ref(), rhs.as_ref()) {
            (ConstInt(n1), ConstInt(n2)) => return Rc::new(ConstBool(n1 < n2)),
            _ => panic!("Cannot compare \n1: {}\n2: {}", lhs, rhs),
        }
    }

    panic!("Unknown builtin: {}", name.0)
}
