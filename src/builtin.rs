use crate::tast::N;
use crate::ty::*;

use once_cell::sync::Lazy;

pub type BuiltinName = N<&'static str>;

pub struct BuiltinDefs {
    plus: BuiltinName,
    minus: BuiltinName,
    less: BuiltinName,
}

impl BuiltinDefs {
    pub fn plus(&self) -> &BuiltinName {
        &self.plus
    }

    pub fn minus(&self) -> &BuiltinName {
        &self.minus
    }

    pub fn less(&self) -> &BuiltinName {
        &self.less
    }
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
