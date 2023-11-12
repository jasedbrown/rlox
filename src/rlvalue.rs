/// The result value and type of evaluating an expression.
/// This first attempt is a basic enum tag [0], but
/// it'd be nifty to build a nan box for the next generation.
///
/// [0] https://piotrduperas.com/posts/nan-boxing
pub enum RlValue {
    // asdfasdf
    Nil,
    Boolean(bool),
    Double(f64),
    String(String),
}
