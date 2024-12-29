mod t;

#[test]
fn daym_invalid() {
    assert!(crontime::build(t::ORIGIN, "* * * 0 * *").is_err());
    assert!(crontime::build(t::ORIGIN, "* * * 32 * *").is_err());
}

#[test]
fn daym_single() {
    t::assert(
        "0 0 0 1 * *",
        &[
            (0, t::datetime!(1917-12-01 00:00:00 UTC)),
            (1, t::datetime!(1918-01-01 00:00:00 UTC)),
            (2, t::datetime!(1918-02-01 00:00:00 UTC)),
            (12, t::datetime!(1918-12-01 00:00:00 UTC)),
            (15, t::datetime!(1919-03-01 00:00:00 UTC)),
        ],
    );

    t::assert(
        "0 0 0 12 * *",
        &[
            (0, t::datetime!(1917-11-12 00:00:00 UTC)),
            (1, t::datetime!(1917-12-12 00:00:00 UTC)),
            (2, t::datetime!(1918-01-12 00:00:00 UTC)),
            (12, t::datetime!(1918-11-12 00:00:00 UTC)),
            (15, t::datetime!(1919-02-12 00:00:00 UTC)),
        ],
    );

    t::assert(
        "0 0 0 29 * *",
        &[
            (0, t::datetime!(1917-11-29 00:00:00 UTC)),
            (1, t::datetime!(1917-12-29 00:00:00 UTC)),
            (2, t::datetime!(1918-01-29 00:00:00 UTC)),
            (12, t::datetime!(1918-12-29 00:00:00 UTC)),
            (13, t::datetime!(1919-01-29 00:00:00 UTC)),
            (14, t::datetime!(1919-03-29 00:00:00 UTC)),
            (25, t::datetime!(1920-02-29 00:00:00 UTC)),
        ],
    );

    t::assert(
        "0 0 0 31 * *",
        &[
            (0, t::datetime!(1917-12-31 00:00:00 UTC)),
            (1, t::datetime!(1918-01-31 00:00:00 UTC)),
            (2, t::datetime!(1918-03-31 00:00:00 UTC)),
            (4, t::datetime!(1918-07-31 00:00:00 UTC)),
            (7, t::datetime!(1918-12-31 00:00:00 UTC)),
            (8, t::datetime!(1919-01-31 00:00:00 UTC)),
        ],
    );
}

#[test]
fn daym_many() {
    t::assert(
        "0 0 0 7,29,31 * *",
        &[
            (0, t::datetime!(1917-11-07 00:00:00 UTC)),
            (1, t::datetime!(1917-11-29 00:00:00 UTC)),
            (2, t::datetime!(1917-12-07 00:00:00 UTC)),
            (3, t::datetime!(1917-12-29 00:00:00 UTC)),
            (4, t::datetime!(1917-12-31 00:00:00 UTC)),
            (5, t::datetime!(1918-01-07 00:00:00 UTC)),
        ],
    );
}

#[test]
fn daym_range() {
    t::assert(
        "0 0 0 27-29 * *",
        &[
            (0, t::datetime!(1917-11-27 00:00:00 UTC)),
            (1, t::datetime!(1917-11-28 00:00:00 UTC)),
            (2, t::datetime!(1917-11-29 00:00:00 UTC)),
            (3, t::datetime!(1917-12-27 00:00:00 UTC)),
            (4, t::datetime!(1917-12-28 00:00:00 UTC)),
            (5, t::datetime!(1917-12-29 00:00:00 UTC)),
            (9, t::datetime!(1918-02-27 00:00:00 UTC)),
            (11, t::datetime!(1918-03-27 00:00:00 UTC)),
        ],
    );
}
