mod t;

#[test]
fn month_invalid() {
    assert!(crontime::build(t::ORIGIN, "* * * * 0 *").is_err());
    assert!(crontime::build(t::ORIGIN, "* * * * 13 *").is_err());
}

#[test]
fn month_single() {
    t::assert(
        "0 0 0 1 1 *",
        &[
            (0, t::datetime!(1918-01-01 00:00:00 UTC)),
            (1, t::datetime!(1919-01-01 00:00:00 UTC)),
            (2, t::datetime!(1920-01-01 00:00:00 UTC)),
            (9, t::datetime!(1927-01-01 00:00:00 UTC)),
        ],
    );

    t::assert(
        "0 0 0 1 11 *",
        &[
            (0, t::datetime!(1918-11-01 00:00:00 UTC)),
            (1, t::datetime!(1919-11-01 00:00:00 UTC)),
            (2, t::datetime!(1920-11-01 00:00:00 UTC)),
            (9, t::datetime!(1927-11-01 00:00:00 UTC)),
        ],
    );

    t::assert(
        "0 0 0 1 12 *",
        &[
            (0, t::datetime!(1917-12-01 00:00:00 UTC)),
            (1, t::datetime!(1918-12-01 00:00:00 UTC)),
            (2, t::datetime!(1919-12-01 00:00:00 UTC)),
            (9, t::datetime!(1926-12-01 00:00:00 UTC)),
        ],
    );
}

#[test]
fn month_many() {
    t::assert(
        "0 0 0 1 5,11,12 *",
        &[
            (0, t::datetime!(1917-12-01 00:00:00 UTC)),
            (1, t::datetime!(1918-05-01 00:00:00 UTC)),
            (2, t::datetime!(1918-11-01 00:00:00 UTC)),
            (3, t::datetime!(1918-12-01 00:00:00 UTC)),
            (4, t::datetime!(1919-05-01 00:00:00 UTC)),
        ],
    );
}

#[test]
fn month_range() {
    t::assert(
        "0 0 0 1 10-12 *",
        &[
            (0, t::datetime!(1917-12-01 00:00:00 UTC)),
            (1, t::datetime!(1918-10-01 00:00:00 UTC)),
            (2, t::datetime!(1918-11-01 00:00:00 UTC)),
            (3, t::datetime!(1918-12-01 00:00:00 UTC)),
            (4, t::datetime!(1919-10-01 00:00:00 UTC)),
        ],
    );
}
