mod t;

#[test]
fn second_invalid() {
    assert!(crontime::build(t::ORIGIN, "-1 * * * * *").is_err());
    assert!(crontime::build(t::ORIGIN, "60 * * * * *").is_err());
}

#[test]
fn second_any() {
    t::assert(
        "* * * * * *",
        &[
            (0, t::datetime!(1917-11-07 00:00:00 UTC)),
            (1, t::datetime!(1917-11-07 00:00:01 UTC)),
            (60, t::datetime!(1917-11-07 00:01:00 UTC)),
            (62, t::datetime!(1917-11-07 00:01:02 UTC)),
        ],
    );
}

#[test]
fn second_single() {
    t::assert(
        "1 * * * * *",
        &[
            (0, t::datetime!(1917-11-07 00:00:01 UTC)),
            (1, t::datetime!(1917-11-07 00:01:01 UTC)),
            (60, t::datetime!(1917-11-07 01:00:01 UTC)),
            (62, t::datetime!(1917-11-07 01:02:01 UTC)),
        ],
    );

    t::assert(
        "7 * * * * *",
        &[
            (0, t::datetime!(1917-11-07 00:00:07 UTC)),
            (1, t::datetime!(1917-11-07 00:01:07 UTC)),
            (60, t::datetime!(1917-11-07 01:00:07 UTC)),
            (62, t::datetime!(1917-11-07 01:02:07 UTC)),
        ],
    );
}

#[test]
fn second_many() {
    t::assert(
        "1,9,7 * * * * *",
        &[
            (0, t::datetime!(1917-11-07 00:00:01 UTC)),
            (1, t::datetime!(1917-11-07 00:00:07 UTC)),
            (2, t::datetime!(1917-11-07 00:00:09 UTC)),
            (3, t::datetime!(1917-11-07 00:01:01 UTC)),
            (7, t::datetime!(1917-11-07 00:02:07 UTC)),
            (182, t::datetime!(1917-11-07 01:00:09 UTC)),
        ],
    );
}

#[test]
fn second_range() {
    t::assert(
        "3-5 * * * * *",
        &[
            (0, t::datetime!(1917-11-07 00:00:03 UTC)),
            (1, t::datetime!(1917-11-07 00:00:04 UTC)),
            (2, t::datetime!(1917-11-07 00:00:05 UTC)),
            (3, t::datetime!(1917-11-07 00:01:03 UTC)),
            (7, t::datetime!(1917-11-07 00:02:04 UTC)),
            (182, t::datetime!(1917-11-07 01:00:05 UTC)),
        ],
    );
}
