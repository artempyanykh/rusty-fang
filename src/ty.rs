use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Ty {
    Int,
    Bool,
    F { par: Vec<Ty>, ret: Box<Ty> },
}

impl Ty {
    #[allow(dead_code)]
    pub fn mk_func_n<'a, 'b>(par: Vec<Ty>, ret: Ty) -> Ty {
        let ret = Box::new(ret);
        Ty::F { par, ret }
    }

    pub fn mk_func_1<'a, 'b>(par: Ty, ret: Ty) -> Ty {
        Ty::F {
            par: vec![par],
            ret: Box::new(ret),
        }
    }

    pub fn mk_func_2(par_1: Ty, par_2: Ty, ret: Ty) -> Ty {
        Ty::F {
            par: vec![par_1, par_2],
            ret: Box::new(ret),
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
