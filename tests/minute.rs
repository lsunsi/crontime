mod t;

#[test]
fn minute_invalid() {
    assert!(crontime::build(t::ORIGIN, "* -1 * * * *").is_err());
    assert!(crontime::build(t::ORIGIN, "* 60 * * * *").is_err());
}

#[test]
fn minute_any() {
    t::assert(
        "0 * * * * *",
        &[
            (0, t::datetime!(1917-11-07 00:00:00 UTC)),
            (1, t::datetime!(1917-11-07 00:01:00 UTC)),
            (60, t::datetime!(1917-11-07 01:00:00 UTC)),
            (62, t::datetime!(1917-11-07 01:02:00 UTC)),
        ],
    );
}

#[test]
fn minute_single() {
    t::assert(
        "0 7 * * * *",
        &[
            (0, t::datetime!(1917-11-07 00:07:00 UTC)),
            (1, t::datetime!(1917-11-07 01:07:00 UTC)),
            (24, t::datetime!(1917-11-08 00:07:00 UTC)),
            (26, t::datetime!(1917-11-08 02:07:00 UTC)),
        ],
    );

    t::assert(
        "0 59 * * * *",
        &[
            (0, t::datetime!(1917-11-07 00:59:00 UTC)),
            (1, t::datetime!(1917-11-07 01:59:00 UTC)),
            (24, t::datetime!(1917-11-08 00:59:00 UTC)),
            (26, t::datetime!(1917-11-08 02:59:00 UTC)),
        ],
    );
}

#[test]
fn minute_many() {
    t::assert(
        "0 7,31,59 * * * *",
        &[
            (0, t::datetime!(1917-11-07 00:07:00 UTC)),
            (1, t::datetime!(1917-11-07 00:31:00 UTC)),
            (2, t::datetime!(1917-11-07 00:59:00 UTC)),
            (3, t::datetime!(1917-11-07 01:07:00 UTC)),
            (6, t::datetime!(1917-11-07 02:07:00 UTC)),
            (8, t::datetime!(1917-11-07 02:59:00 UTC)),
        ],
    );
}

#[test]
fn minute_range() {
    t::assert(
        "0 17-39 * * * *",
        &[
            (0, t::datetime!(1917-11-07 00:17:00 UTC)),
            (1, t::datetime!(1917-11-07 00:18:00 UTC)),
            (2, t::datetime!(1917-11-07 00:19:00 UTC)),
            (20, t::datetime!(1917-11-07 00:37:00 UTC)),
            (21, t::datetime!(1917-11-07 00:38:00 UTC)),
            (22, t::datetime!(1917-11-07 00:39:00 UTC)),
            (23, t::datetime!(1917-11-07 01:17:00 UTC)),
            (46, t::datetime!(1917-11-07 02:17:00 UTC)),
            (48, t::datetime!(1917-11-07 02:19:00 UTC)),
        ],
    );
}
