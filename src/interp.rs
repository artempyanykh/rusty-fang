use crate::tast::Ex;

#[derive(Clone)]
pub struct Env {
    generation: usize,
    mapping: Vec<(NameDef, usize), Rc<Ex>>,
}

impl Env {
    pub fn bind(&self, name: &NameDef, ex: Rc<Ex>) -> Env {
        let new = self.clone();
        new.generation = self.generation + 1;
        new.mapping.add((name.clone(), new.generation), ex);
        new
    }
}

pub fn eval_ex(ex: Ex, env: Env) -> Ex {
    ex
}
