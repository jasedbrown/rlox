use crate::token::{Literal, Token};

enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Get(Box<Expr>, Token),
    Grouping(Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Set(Box<Expr>, Token, Box<Expr>),
    Super(Token, Token),
    This(Token),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

impl Expr {
    fn sorta_pretty_print(expr: &Expr) -> String {
        use Expr::*;
        match expr {
            Assign(_t, _e) => String::new(),
            Binary(l, t, r) => Self::parenthesize(Some(&t.lexeme), vec![l, r]),
            Call(_e, _t, _v) => String::new(),
            Get(_e, _t) => String::new(),
            Grouping(e) => Self::parenthesize(None, vec![e]),
            Literal(_l) => {
                // start here next time!
            },,
            Logical(_l, _t, _r) => String::new(),
            Set(_l, _t, _r) => String::new(),
            Super(_t1, _t2) => String::new(),
            This(_t) => String::new(),
            Unary(_t, _e) => String::new(),
            Variable(_t) => String::new(),
        }
    }

    fn parenthesize(name: Option<&str>, exprs: Vec<&Expr>) -> String {
        let mut s = String::new();
        s.push('(');

        if let Some(n) = name {
            s.push_str(n);
        }

        for e in exprs {
            s.push(' ');
            s.push_str(Self::sorta_pretty_print(e).as_str());
        }

        s.push(')');

        s
    }

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
}
