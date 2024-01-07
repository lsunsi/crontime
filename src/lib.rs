mod expr;
mod iter;

pub struct Crontime {
    o: time::PrimitiveDateTime,
    e: expr::Expr,
}

pub fn build(o: time::PrimitiveDateTime, s: &str) -> Result<Crontime, nom::Err<()>> {
    s.parse().map(|e| Crontime { o, e })
}
