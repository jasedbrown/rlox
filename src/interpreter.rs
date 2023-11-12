use anyhow::{anyhow, Result};

use crate::expr::{Expr, LiteralValue};
use crate::ErrorReporter;

pub struct Interpreter {
    error_reporter: ErrorReporter,
}

impl Interpreter {
    pub fn new(error_reporter: ErrorReporter) -> Self {
        Self { error_reporter }
    }

    pub fn do_it(&self, expr: Expr) -> Result<()> {
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
