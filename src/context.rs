use std::{
    fmt::{Debug, Display},
    fs, mem,
    path::PathBuf,
};

use crate::lang::{
    Expr, ExprError, ExprSet, ExprType, ParsableValue, Result, ops::ExprOps, parse_str,
};

pub struct LangContext<'expr, T>
where
    T: Clone + PartialEq + Display + ExprOps + ParsableValue + Debug,
{
    builtins: ExprSet<'expr, T>,
}

impl<'expr, T> LangContext<'expr, T>
where
    T: Clone + PartialEq + Display + ExprOps + ParsableValue + Debug,
{
    pub fn new() -> Self {
        LangContext {
            builtins: ExprSet::default(),
        }
    }

    pub fn add_builtin<F>(&mut self, name: impl ToString, func: F) -> Result<()>
    where
        F: 'expr + Fn(&Expr<'expr, T>) -> std::result::Result<Expr<'expr, T>, ExprError>,
    {
        let builtin_name = name.to_string();
        let builtin_expr = Expr::new_builtin(name, func);
        let previous = mem::replace(&mut self.builtins, ExprSet::new());
        self.builtins = previous.set(builtin_name, builtin_expr)?;
        Ok(())
    }

    pub fn read_file(&self, filename: PathBuf) -> Result<Expr<'expr, T>> {
        let code = fs::read_to_string(filename).unwrap();
        let builtins = self.builtins.clone();
        /*.set("include", |file_expr| {
            self.read_file(file_expr.eval_string()?)
        });*/
        let expr = ExprType::BoundExpr(builtins, parse_str(&code)?).into();
        Ok(expr)
    }
}
