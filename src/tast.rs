use crate::ty::Ty;

use indented::indented;
use std::{fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub struct NameDef(String);

pub struct Prog {
    bindings: Vec<N<Binding>>,
}

#[derive(Clone)]
struct N<T> {
    t: Rc<T>,
    ty: Ty,
}

impl<T> N<T> {
    fn new(t: T, ty: Ty) -> Self {
        Self { t: Rc::new(t), ty }
    }
}

struct Lambda {
    bound: Vec<N<NameDef>>,
    free: Vec<N<NameDef>>,
    body: Ex,
}

impl Lambda {
    pub fn ty(&self) -> Ty {
        let par = self.bound.iter().map(|b| &b.ty);
        let ret = &self.body.ty();
        Ty::mk_func_n(par, ret)
    }
}

pub struct Binding {
    name: N<NameDef>,
    ex: Ex,
}

struct Application {
    ex: Ex,
    args: Vec<Ex>,
}

struct Condition {
    pred: Ex,
    then: Ex,
    els: Ex,
}

pub enum Ex {
    Bind(N<Binding>),
    Lam(N<Lambda>),
    Ap(N<Application>),
    Cond(N<Condition>),
    Ref(N<NameDef>),
    ConstInt(i64),
    ConstBool(bool),
}

impl Ex {
    pub fn ty(&self) -> &Ty {
        match self {
            Ex::Bind(n) => &n.ty,
            Ex::Lam(n) => &n.ty,
            Ex::Ap(n) => &n.ty,
            Ex::Cond(n) => &n.ty,
            Ex::Ref(n) => &n.ty,
            Ex::ConstInt(_) => &Ty::Int,
            Ex::ConstBool(_) => &Ty::Bool,
        }
    }
}

impl Display for Ex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ex::Bind(b) => {
                writeln!(f, "{}: {}", b.t.name.t.0, b.ty)?;
                write!(f, "{}", indented(&b.t.ex))?;
            }
            Ex::Lam(_) => (),
            Ex::Ap(_) => (),
            Ex::Cond(_) => (),
            Ex::Ref(r) => {
                write!(f, "{}: {}", r.t.0, r.ty)?;
            }
            Ex::ConstInt(i) => {
                write!(f, "{}", i)?;
            }
            Ex::ConstBool(b) => {
                write!(f, "{}", b)?;
            }
        }

        Ok(())
    }
}

impl From<N<NameDef>> for Ex {
    fn from(v: N<NameDef>) -> Self {
        Ex::Ref(v)
    }
}

impl From<N<Condition>> for Ex {
    fn from(v: N<Condition>) -> Self {
        Ex::Cond(v)
    }
}

impl From<N<Application>> for Ex {
    fn from(v: N<Application>) -> Self {
        Ex::Ap(v)
    }
}

impl From<N<Lambda>> for Ex {
    fn from(v: N<Lambda>) -> Self {
        Ex::Lam(v)
    }
}

impl From<N<Binding>> for Ex {
    fn from(v: N<Binding>) -> Self {
        Ex::Bind(v)
    }
}

pub mod example {
    use super::Ex::*;
    use super::*;

    pub fn fibonacci() -> Binding {
        let cmp_type = Ty::mk_func_2(&Ty::Int, &Ty::Int, &Ty::Bool);
        let less = N::new(NameDef("less".to_string()), cmp_type.clone());

        let arith_type = Ty::mk_func_2(&Ty::Int, &Ty::Int, &Ty::Int);

        let plus = N::new(NameDef("plus".to_string()), arith_type.clone());
        let minus = N::new(NameDef("minus".to_string()), arith_type.clone());

        let fib = N::new(
            NameDef("fib".to_string()),
            Ty::mk_func_1(&Ty::Int, &Ty::Int),
        );
        let n = N::new(NameDef("n".to_string()), Ty::Int);

        let pred = Application {
            ex: less.clone().into(),
            args: vec![n.clone().into(), ConstInt(2)],
        };

        let then = ConstInt(1);

        let els = Application {
            ex: plus.clone().into(),
            args: vec![
                N::new(
                    Application {
                        ex: fib.clone().into(),
                        args: vec![N::new(
                            Application {
                                ex: minus.clone().into(),
                                args: vec![n.clone().into(), ConstInt(1)],
                            },
                            Ty::Int,
                        )
                        .into()],
                    },
                    Ty::Int,
                )
                .into(),
                N::new(
                    Application {
                        ex: fib.clone().into(),
                        args: vec![N::new(
                            Application {
                                ex: minus.clone().into(),
                                args: vec![n.clone().into(), ConstInt(2)],
                            },
                            Ty::Int,
                        )
                        .into()],
                    },
                    Ty::Int,
                )
                .into(),
            ],
        };

        let body = Condition {
            pred: N::new(pred, Ty::Bool).into(),
            then: then,
            els: N::new(els, Ty::Int).into(),
        };

        let lam = Lambda {
            bound: vec![n.clone().into()],
            free: vec![],
            body: N::new(body, Ty::Int).into(),
        };
        let lam_ty = Ty::mk_func_1(&Ty::Int, &Ty::Int);

        Binding {
            name: fib.clone().into(),
            ex: N::new(lam, lam_ty).into(),
        }
    }

    pub fn fibonacci_ex() -> Ex {
        let binding = fibonacci();
        let ty = Ty::mk_func_1(&Ty::Int, &Ty::Int);
        N::new(binding, ty).into()
    }

    pub fn const_binding() -> Binding {
        let name = N::new(NameDef("n".to_string()), Ty::Int);
        Binding {
            name,
            ex: ConstInt(42),
        }
    }

    pub fn const_binding_ex() -> Ex {
        let binding = const_binding();
        N::new(binding, Ty::Int).into()
    }
}
