use std::{fmt::Display, rc::Rc};

use crate::{
    Expr,
    lang::{ExprSet, ExprType, Result, ops::ExprBuiltin},
    path::VirtPath,
    value::Value,
};

#[derive(PartialEq, Debug)]
pub struct PbBuild {
    dest: VirtPath,
}

impl Display for PbBuild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Build({})", self.dest)
    }
}

#[derive(Debug)]
pub struct BuiltinPbBuild;

impl ExprBuiltin<Value> for BuiltinPbBuild {
    fn get_name(&self) -> String {
        "build".into()
    }

    fn call(
        &self,
        arg: crate::lang::Expr<Value>,
    ) -> crate::lang::ops::Result<crate::lang::Expr<Value>> {
        let val = arg.value()?;
        let path = val
            .try_as_path()
            .ok_or(crate::lang::ops::Error::Type(format!(
                "expected path, got {}",
                arg
            )))?;
        Ok(ExprType::Value(Value::Path(path.lock())).into())
    }
}

pub fn get_pb_builtins() -> Result<Expr<Value>> {
    let pbset = ExprSet::new().set("build", Expr::new_builtin(Rc::new(BuiltinPbBuild)))?;
    Ok(pbset.into())
}
