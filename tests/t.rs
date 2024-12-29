pub use time::macros::datetime;
pub const ORIGIN: time::OffsetDateTime = datetime!(1917-11-07 00:00:00 UTC);

pub fn assert(i: &'static str, os: &[(usize, time::OffsetDateTime)]) {
    let mut ct = crontime::build(ORIGIN, i).expect("build").enumerate();

    for (n, o) in os {
        let oo = ct
            .find_map(|(nn, oo)| (nn == *n).then_some(oo))
            .expect("find");

        assert_eq!(oo, *o);
    }
}
