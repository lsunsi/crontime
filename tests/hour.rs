mod t;

#[test]
fn hour_invalid() {
    assert!(crontime::build(t::ORIGIN, "* * -1 * * *").is_err());
    assert!(crontime::build(t::ORIGIN, "* * 24 * * *").is_err());
}

#[test]
fn hour_single() {
    t::assert(
        "0 0 11 * * *",
        &[
            (0, t::datetime!(1917-11-07 11:00:00 UTC)),
            (1, t::datetime!(1917-11-08 11:00:00 UTC)),
            (2, t::datetime!(1917-11-09 11:00:00 UTC)),
            (24, t::datetime!(1917-12-01 11:00:00 UTC)),
        ],
    );

    t::assert(
        "0 0 23 * * *",
        &[
            (0, t::datetime!(1917-11-07 23:00:00 UTC)),
            (1, t::datetime!(1917-11-08 23:00:00 UTC)),
            (2, t::datetime!(1917-11-09 23:00:00 UTC)),
            (24, t::datetime!(1917-12-01 23:00:00 UTC)),
        ],
    );
}

#[test]
fn hour_many() {
    t::assert(
        "0 0 6,8,23 * * *",
        &[
            (0, t::datetime!(1917-11-07 06:00:00 UTC)),
            (1, t::datetime!(1917-11-07 08:00:00 UTC)),
            (2, t::datetime!(1917-11-07 23:00:00 UTC)),
            (3, t::datetime!(1917-11-08 06:00:00 UTC)),
            (4, t::datetime!(1917-11-08 08:00:00 UTC)),
            (5, t::datetime!(1917-11-08 23:00:00 UTC)),
            (72, t::datetime!(1917-12-01 06:00:00 UTC)),
        ],
    );
}

#[test]
fn hour_range() {
    t::assert(
        "0 0 11-13 * * *",
        &[
            (0, t::datetime!(1917-11-07 11:00:00 UTC)),
            (1, t::datetime!(1917-11-07 12:00:00 UTC)),
            (2, t::datetime!(1917-11-07 13:00:00 UTC)),
            (3, t::datetime!(1917-11-08 11:00:00 UTC)),
            (4, t::datetime!(1917-11-08 12:00:00 UTC)),
            (5, t::datetime!(1917-11-08 13:00:00 UTC)),
            (72, t::datetime!(1917-12-01 11:00:00 UTC)),
        ],
    );
}
