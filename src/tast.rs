use crate::builtin::{BuiltinName, B};
use crate::ty::Ty;
use std::ops::Deref;

use indented::indented;
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Clone)]
pub struct N<T> {
    pub t: Arc<T>,
    pub ty: Ty,
}

impl<T> N<T> {
    pub fn new(t: T, ty: Ty) -> Self {
        Self { t: Arc::new(t), ty }
    }
}

impl<T> Deref for N<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NameDef(pub String);

impl Display for N<NameDef> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name {}: {}", self.t.0, self.ty)
    }
}

#[allow(dead_code)]
pub struct Prog {
    bindings: Vec<N<Binding>>,
}

#[derive(Debug, Clone)]
pub struct Lambda {
    pub bound: Vec<N<NameDef>>,
    pub free: Vec<N<NameDef>>,
    pub body: Ex,
}

impl Display for N<Lambda> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: handle free vars
        let bounds = self
            .bound
            .iter()
            .map(|b| format!("{}", b))
            .collect::<Vec<_>>()
            .join(",");

        writeln!(f, "λ ({}): {},", bounds, self.ty)?;
        write!(f, "{}", indented(&self.t.body))
    }
}

#[derive(Debug, Clone)]
pub struct Binding {
    pub name: N<NameDef>,
    pub ex: Ex,
}

impl Display for N<Binding> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Binding {} = ", self.t.name)?;
        write!(f, "{}", indented(&self.t.ex))
    }
}

#[derive(Debug, Clone)]
pub struct Application {
    pub ex: Ex,
    pub args: Vec<Ex>,
}

impl Display for N<Application> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "App {}: {}", self.t.ex, self.ty)?;
        let arg_fmt = self
            .t
            .args
            .iter()
            .map(|a| format!("{}", indented(a)))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", arg_fmt)
    }
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub pred: Ex,
    pub then: Ex,
    pub els: Ex,
}

impl Display for N<Condition> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "if {}", self.t.pred)?;
        writeln!(f, "then {}", self.t.then)?;
        write!(f, "else {}", self.t.els)
    }
}

#[derive(Debug, Clone)]
pub enum Ex {
    Bind(N<Binding>),
    Lam(N<Lambda>),
    Ap(N<Application>),
    Cond(N<Condition>),
    URef(N<NameDef>),
    BRef(&'static N<BuiltinName>),
    ConstInt(i64),
    ConstBool(bool),
}

impl Ex {
    #[allow(dead_code)]
    pub fn ty(&self) -> &Ty {
        match self {
            Ex::Bind(n) => &n.ty,
            Ex::Lam(n) => &n.ty,
            Ex::Ap(n) => &n.ty,
            Ex::Cond(n) => &n.ty,
            Ex::URef(r) => &r.ty,
            Ex::BRef(r) => &r.ty,
            Ex::ConstInt(_) => &Ty::Int,
            Ex::ConstBool(_) => &Ty::Bool,
        }
    }
}

impl Display for Ex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ex::Bind(b) => write!(f, "{}", b)?,
            Ex::Lam(l) => write!(f, "{}", l)?,
            Ex::Ap(a) => write!(f, "{}", a)?,
            Ex::Cond(c) => write!(f, "{}", c)?,
            Ex::URef(r) => write!(f, "{}", r)?,
            Ex::BRef(r) => write!(f, "{}", r)?,
            Ex::ConstInt(i) => write!(f, "{}", i)?,
            Ex::ConstBool(b) => write!(f, "{}", b)?,
        }

        Ok(())
    }
}

impl From<N<NameDef>> for Ex {
    fn from(v: N<NameDef>) -> Self {
        Ex::URef(v)
    }
}

impl From<&'static N<BuiltinName>> for Ex {
    fn from(v: &'static N<BuiltinName>) -> Self {
        Ex::BRef(v)
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

#[allow(dead_code)]
pub mod example {
    use super::Ex::*;
    use super::*;

    pub fn fibonacci() -> Binding {
        let less = &B.less;
        let plus = &B.plus;
        let minus = &B.minus;

        let fib = N::new(NameDef("fib".to_string()), Ty::mk_func_1(Ty::Int, Ty::Int));
        let n = N::new(NameDef("n".to_string()), Ty::Int);

        let pred = Application {
            ex: less.into(),
            args: vec![n.clone().into(), ConstInt(2)],
        };

        let then = ConstInt(1);

        let els = Application {
            ex: plus.into(),
            args: vec![
                N::new(
                    Application {
                        ex: fib.clone().into(),
                        args: vec![N::new(
                            Application {
                                ex: minus.into(),
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
                                ex: minus.into(),
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
        let lam_ty = Ty::mk_func_1(Ty::Int, Ty::Int);

        Binding {
            name: fib.clone().into(),
            ex: N::new(lam, lam_ty).into(),
        }
    }

    pub fn fibonacci_ex() -> Ex {
        let binding = fibonacci();
        let ty = Ty::mk_func_1(Ty::Int, Ty::Int);
        N::new(binding, ty).into()
    }

    pub fn fibonacci_ap_n(n: i64) -> Ex {
        let fib = N::new(NameDef("fib".to_string()), Ty::mk_func_1(Ty::Int, Ty::Int));
        let app = Application {
            ex: fib.into(),
            args: vec![ConstInt(n)],
        };
        N::new(app, Ty::Int).into()
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

    pub fn inc_binding() -> Binding {
        let plus = &B.plus;
        let n = N::new(NameDef("n".to_string()), Ty::Int);

        let lam = Lambda {
            bound: vec![n.clone()],
            free: vec![],
            body: N::new(
                Application {
                    ex: plus.into(),
                    args: vec![n.clone().into(), ConstInt(1)],
                },
                Ty::Int,
            )
            .into(),
        };

        let ty = Ty::mk_func_1(Ty::Int, Ty::Int);
        Binding {
            name: N::new(NameDef("inc".to_string()), ty.clone()),
            ex: N::new(lam, ty).into(),
        }
    }

    pub fn inc_ex() -> Ex {
        let ty = Ty::mk_func_1(Ty::Int, Ty::Int);
        N::new(inc_binding(), ty).into()
    }

    pub fn inc_ap_n(n: i64) -> Ex {
        let ty = Ty::mk_func_1(Ty::Int, Ty::Int);
        let ex = N::new(NameDef("inc".to_string()), ty).into();
        let app = Application {
            ex,
            args: vec![ConstInt(n)],
        };
        N::new(app, Ty::Int).into()
    }

    pub fn incinc_ap_n(n: i64) -> Ex {
        let ty = Ty::mk_func_1(Ty::Int, Ty::Int);
        let inc_ref = N::new(NameDef("inc".to_string()), ty);

        let app = Application {
            ex: inc_ref.clone().into(),
            args: vec![ConstInt(n)],
        };
        let app = N::new(app, Ty::Int);

        let app2 = Application {
            ex: inc_ref.clone().into(),
            args: vec![app.into()],
        };
        N::new(app2, Ty::Int).into()
    }
}
