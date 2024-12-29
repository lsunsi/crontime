mod expr;
mod iter;

pub struct Crontime {
    o: time::OffsetDateTime,
    e: expr::Expr,
}

pub fn build(o: time::OffsetDateTime, s: &str) -> Result<Crontime, nom::Err<()>> {
    s.parse().map(|e| Crontime {
        o: o - time::Duration::SECOND,
        e,
    })
}
