use crate::tast::N;
use crate::ty::*;
use std::fmt::{self, Display};

use once_cell::sync::Lazy;

pub type BuiltinName = &'static str;

impl Display for N<BuiltinName> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "BuiltinName {}: {}", self.t, self.ty)
    }
}

pub struct BuiltinDefs {
    pub plus: N<BuiltinName>,
    pub minus: N<BuiltinName>,
    pub less: N<BuiltinName>,
}

pub static B: Lazy<BuiltinDefs> = Lazy::new(|| BuiltinDefs::new());

impl BuiltinDefs {
    fn new() -> Self {
        let cmp_type = Ty::mk_func_2(Ty::Int, Ty::Int, Ty::Bool);
        let less = N::new("less", cmp_type.clone());

        let arith_type = Ty::mk_func_2(Ty::Int, Ty::Int, Ty::Int);

        let plus = N::new("plus", arith_type.clone());
        let minus = N::new("minus", arith_type.clone());

        BuiltinDefs { plus, minus, less }
    }
}
