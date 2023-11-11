use anyhow::{anyhow, Result};

use crate::expr::{Expr, LiteralValue};
//use crate::ErrortReporter;

pub struct Interpreter {}

impl Interpreter {
    pub fn do_it(&self) -> Result<()> {
        // fn visit(expr: Expr) -> Result<()> {
        //     use Expr::*;
        //     match expr {
        //         Assign(_t, _e) => (),
        //         Binary(_l, _t, _r) => (),
        //         Call(_e, _t, _v) => (),
        //         Get(_e, _t) => (),
        //         Grouping(_e) => (),
        //         Literal(_l) => (),
        //         Logical(_l, _t, _r) => (),
        //         Set(_l, _t, _r) => (),
        //         Super(_t1, _t2) => (),
        //         This(_t) => (),
        //         Unary(_t, _e) => (),
        //         Variable(_t) => (),
        //     };

        //     Ok(())
        // }

        Ok(())
    }
}
