use std::fmt::Display;

#[derive(Clone)]
pub enum Ty {
    Int,
    Bool,
    F { par: Vec<Ty>, ret: Box<Ty> },
}

impl Ty {
    pub fn mk_func_n<'a, 'b>(par: impl Iterator<Item = &'a Ty>, ret: &'b Ty) -> Ty {
        let par = par.cloned().collect();
        let ret = Box::new(ret.clone());

        Ty::F { par, ret }
    }

    pub fn mk_func_1<'a, 'b>(par: &Ty, ret: &Ty) -> Ty {
        Ty::F {
            par: vec![par.clone()],
            ret: Box::new(ret.clone()),
        }
    }

    pub fn mk_func_2(par_1: &Ty, par_2: &Ty, ret: &Ty) -> Ty {
        Ty::F {
            par: vec![par_1.clone(), par_2.clone()],
            ret: Box::new(ret.clone()),
        }
    }
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ty::Int => write!(f, "Int"),
            Ty::Bool => write!(f, "Bool"),
            Ty::F { par, ret } => {
                let par_display = par
                    .iter()
                    .map(|p| format!("{}", p))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "({}) -> {}", par_display, ret)
            }
        }
    }
}
