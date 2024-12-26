use time::macros::datetime;
const ORIGIN: time::OffsetDateTime = datetime!(1917-11-07 00:00:00 UTC);

#[test]
fn second_invalid() {
    assert!(crontime::build(ORIGIN, "-1 * * * * *").is_err());
    assert!(crontime::build(ORIGIN, "60 * * * * *").is_err());
}

#[test]
fn second_any() {
    assert(
        "* * * * * *",
        &[
            (0, datetime!(1917-11-07 00:00:00 UTC)),
            (1, datetime!(1917-11-07 00:00:01 UTC)),
            (60, datetime!(1917-11-07 00:01:00 UTC)),
            (62, datetime!(1917-11-07 00:01:02 UTC)),
        ],
    );
}

#[test]
fn second_single() {
    assert(
        "1 * * * * *",
        &[
            (0, datetime!(1917-11-07 00:00:01 UTC)),
            (1, datetime!(1917-11-07 00:01:01 UTC)),
            (60, datetime!(1917-11-07 01:00:01 UTC)),
            (62, datetime!(1917-11-07 01:02:01 UTC)),
        ],
    );

    assert(
        "7 * * * * *",
        &[
            (0, datetime!(1917-11-07 00:00:07 UTC)),
            (1, datetime!(1917-11-07 00:01:07 UTC)),
            (60, datetime!(1917-11-07 01:00:07 UTC)),
            (62, datetime!(1917-11-07 01:02:07 UTC)),
        ],
    );
}

#[test]
fn second_many() {
    assert(
        "1,9,7 * * * * *",
        &[
            (0, datetime!(1917-11-07 00:00:01 UTC)),
            (1, datetime!(1917-11-07 00:00:07 UTC)),
            (2, datetime!(1917-11-07 00:00:09 UTC)),
            (3, datetime!(1917-11-07 00:01:01 UTC)),
            (7, datetime!(1917-11-07 00:02:07 UTC)),
            (182, datetime!(1917-11-07 01:00:09 UTC)),
        ],
    );
}

#[test]
fn second_range() {
    assert(
        "3-5 * * * * *",
        &[
            (0, datetime!(1917-11-07 00:00:03 UTC)),
            (1, datetime!(1917-11-07 00:00:04 UTC)),
            (2, datetime!(1917-11-07 00:00:05 UTC)),
            (3, datetime!(1917-11-07 00:01:03 UTC)),
            (7, datetime!(1917-11-07 00:02:04 UTC)),
            (182, datetime!(1917-11-07 01:00:05 UTC)),
        ],
    );
}

#[test]
fn minute_invalid() {
    assert!(crontime::build(ORIGIN, "* -1 * * * *").is_err());
    assert!(crontime::build(ORIGIN, "* 60 * * * *").is_err());
}

#[test]
fn minute_any() {
    assert(
        "0 * * * * *",
        &[
            (0, datetime!(1917-11-07 00:00:00 UTC)),
            (1, datetime!(1917-11-07 00:01:00 UTC)),
            (60, datetime!(1917-11-07 01:00:00 UTC)),
            (62, datetime!(1917-11-07 01:02:00 UTC)),
        ],
    );
}

#[test]
fn minute_single() {
    assert(
        "0 7 * * * *",
        &[
            (0, datetime!(1917-11-07 00:07:00 UTC)),
            (1, datetime!(1917-11-07 01:07:00 UTC)),
            (24, datetime!(1917-11-08 00:07:00 UTC)),
            (26, datetime!(1917-11-08 02:07:00 UTC)),
        ],
    );

    assert(
        "0 59 * * * *",
        &[
            (0, datetime!(1917-11-07 00:59:00 UTC)),
            (1, datetime!(1917-11-07 01:59:00 UTC)),
            (24, datetime!(1917-11-08 00:59:00 UTC)),
            (26, datetime!(1917-11-08 02:59:00 UTC)),
        ],
    );
}

#[test]
fn minute_many() {
    assert(
        "0 7,31,59 * * * *",
        &[
            (0, datetime!(1917-11-07 00:07:00 UTC)),
            (1, datetime!(1917-11-07 00:31:00 UTC)),
            (2, datetime!(1917-11-07 00:59:00 UTC)),
            (3, datetime!(1917-11-07 01:07:00 UTC)),
            (6, datetime!(1917-11-07 02:07:00 UTC)),
            (8, datetime!(1917-11-07 02:59:00 UTC)),
        ],
    );
}

#[test]
fn minute_range() {
    assert(
        "0 17-39 * * * *",
        &[
            (0, datetime!(1917-11-07 00:17:00 UTC)),
            (1, datetime!(1917-11-07 00:18:00 UTC)),
            (2, datetime!(1917-11-07 00:19:00 UTC)),
            (20, datetime!(1917-11-07 00:37:00 UTC)),
            (21, datetime!(1917-11-07 00:38:00 UTC)),
            (22, datetime!(1917-11-07 00:39:00 UTC)),
            (23, datetime!(1917-11-07 01:17:00 UTC)),
            (46, datetime!(1917-11-07 02:17:00 UTC)),
            (48, datetime!(1917-11-07 02:19:00 UTC)),
        ],
    );
}

#[test]
fn hour_invalid() {
    assert!(crontime::build(ORIGIN, "* * -1 * * *").is_err());
    assert!(crontime::build(ORIGIN, "* * 24 * * *").is_err());
}

#[test]
fn hour_single() {
    assert(
        "0 0 11 * * *",
        &[
            (0, datetime!(1917-11-07 11:00:00 UTC)),
            (1, datetime!(1917-11-08 11:00:00 UTC)),
            (2, datetime!(1917-11-09 11:00:00 UTC)),
            (24, datetime!(1917-12-01 11:00:00 UTC)),
        ],
    );

    assert(
        "0 0 23 * * *",
        &[
            (0, datetime!(1917-11-07 23:00:00 UTC)),
            (1, datetime!(1917-11-08 23:00:00 UTC)),
            (2, datetime!(1917-11-09 23:00:00 UTC)),
            (24, datetime!(1917-12-01 23:00:00 UTC)),
        ],
    );
}

#[test]
fn hour_many() {
    assert(
        "0 0 6,8,23 * * *",
        &[
            (0, datetime!(1917-11-07 06:00:00 UTC)),
            (1, datetime!(1917-11-07 08:00:00 UTC)),
            (2, datetime!(1917-11-07 23:00:00 UTC)),
            (3, datetime!(1917-11-08 06:00:00 UTC)),
            (4, datetime!(1917-11-08 08:00:00 UTC)),
            (5, datetime!(1917-11-08 23:00:00 UTC)),
            (72, datetime!(1917-12-01 06:00:00 UTC)),
        ],
    );
}

#[test]
fn hour_range() {
    assert(
        "0 0 11-13 * * *",
        &[
            (0, datetime!(1917-11-07 11:00:00 UTC)),
            (1, datetime!(1917-11-07 12:00:00 UTC)),
            (2, datetime!(1917-11-07 13:00:00 UTC)),
            (3, datetime!(1917-11-08 11:00:00 UTC)),
            (4, datetime!(1917-11-08 12:00:00 UTC)),
            (5, datetime!(1917-11-08 13:00:00 UTC)),
            (72, datetime!(1917-12-01 11:00:00 UTC)),
        ],
    );
}

#[test]
fn daym_invalid() {
    assert!(crontime::build(ORIGIN, "* * * 0 * *").is_err());
    assert!(crontime::build(ORIGIN, "* * * 32 * *").is_err());
}

#[test]
fn daym_single() {
    assert(
        "0 0 0 1 * *",
        &[
            (0, datetime!(1917-12-01 00:00:00 UTC)),
            (1, datetime!(1918-01-01 00:00:00 UTC)),
            (2, datetime!(1918-02-01 00:00:00 UTC)),
            (12, datetime!(1918-12-01 00:00:00 UTC)),
            (15, datetime!(1919-03-01 00:00:00 UTC)),
        ],
    );

    assert(
        "0 0 0 12 * *",
        &[
            (0, datetime!(1917-11-12 00:00:00 UTC)),
            (1, datetime!(1917-12-12 00:00:00 UTC)),
            (2, datetime!(1918-01-12 00:00:00 UTC)),
            (12, datetime!(1918-11-12 00:00:00 UTC)),
            (15, datetime!(1919-02-12 00:00:00 UTC)),
        ],
    );

    assert(
        "0 0 0 29 * *",
        &[
            (0, datetime!(1917-11-29 00:00:00 UTC)),
            (1, datetime!(1917-12-29 00:00:00 UTC)),
            (2, datetime!(1918-01-29 00:00:00 UTC)),
            (12, datetime!(1918-12-29 00:00:00 UTC)),
            (13, datetime!(1919-01-29 00:00:00 UTC)),
            (14, datetime!(1919-03-29 00:00:00 UTC)),
            (25, datetime!(1920-02-29 00:00:00 UTC)),
        ],
    );

    assert(
        "0 0 0 31 * *",
        &[
            (0, datetime!(1917-12-31 00:00:00 UTC)),
            (1, datetime!(1918-01-31 00:00:00 UTC)),
            (2, datetime!(1918-03-31 00:00:00 UTC)),
            (4, datetime!(1918-07-31 00:00:00 UTC)),
            (7, datetime!(1918-12-31 00:00:00 UTC)),
            (8, datetime!(1919-01-31 00:00:00 UTC)),
        ],
    );
}

#[test]
fn daym_many() {
    assert(
        "0 0 0 7,29,31 * *",
        &[
            (0, datetime!(1917-11-07 00:00:00 UTC)),
            (1, datetime!(1917-11-29 00:00:00 UTC)),
            (2, datetime!(1917-12-07 00:00:00 UTC)),
            (3, datetime!(1917-12-29 00:00:00 UTC)),
            (4, datetime!(1917-12-31 00:00:00 UTC)),
            (5, datetime!(1918-01-07 00:00:00 UTC)),
        ],
    );
}

#[test]
fn daym_range() {
    assert(
        "0 0 0 27-29 * *",
        &[
            (0, datetime!(1917-11-27 00:00:00 UTC)),
            (1, datetime!(1917-11-28 00:00:00 UTC)),
            (2, datetime!(1917-11-29 00:00:00 UTC)),
            (3, datetime!(1917-12-27 00:00:00 UTC)),
            (4, datetime!(1917-12-28 00:00:00 UTC)),
            (5, datetime!(1917-12-29 00:00:00 UTC)),
            (9, datetime!(1918-02-27 00:00:00 UTC)),
            (11, datetime!(1918-03-27 00:00:00 UTC)),
        ],
    );
}

#[test]
fn test_legacy() {
    let now = datetime!(1917-11-07 00:00:00 UTC);

    let ios: &[(&str, &[(usize, time::OffsetDateTime)])] = &[
        (
            "* * * * * *",
            &[
                (0, datetime!(1917-11-07 00:00:00 UTC)),
                (1, datetime!(1917-11-07 00:00:01 UTC)),
                (60, datetime!(1917-11-07 00:01:00 UTC)),
                (62, datetime!(1917-11-07 00:01:02 UTC)),
            ],
        ),
        (
            "1 * * * * *",
            &[
                (0, datetime!(1917-11-07 00:00:01 UTC)),
                (1, datetime!(1917-11-07 00:01:01 UTC)),
                (60, datetime!(1917-11-07 01:00:01 UTC)),
                (62, datetime!(1917-11-07 01:02:01 UTC)),
            ],
        ),
        (
            "* 9 * * * *",
            &[
                (0, datetime!(1917-11-07 00:09:00 UTC)),
                (1, datetime!(1917-11-07 00:09:01 UTC)),
                (60, datetime!(1917-11-07 01:09:00 UTC)),
                (62, datetime!(1917-11-07 01:09:02 UTC)),
            ],
        ),
        (
            "1 9 * * * *",
            &[
                (0, datetime!(1917-11-07 00:09:01 UTC)),
                (1, datetime!(1917-11-07 01:09:01 UTC)),
                (24, datetime!(1917-11-08 00:09:01 UTC)),
                (26, datetime!(1917-11-08 02:09:01 UTC)),
            ],
        ),
        (
            "0 2-4 * * * *",
            &[
                (0, datetime!(1917-11-07 00:02:00 UTC)),
                (1, datetime!(1917-11-07 00:03:00 UTC)),
                (2, datetime!(1917-11-07 00:04:00 UTC)),
                (3, datetime!(1917-11-07 01:02:00 UTC)),
            ],
        ),
        (
            "0 2,4 * * * *",
            &[
                (0, datetime!(1917-11-07 00:02:00 UTC)),
                (1, datetime!(1917-11-07 00:04:00 UTC)),
                (2, datetime!(1917-11-07 01:02:00 UTC)),
            ],
        ),
        (
            "0 * 0 * * *",
            &[
                (0, datetime!(1917-11-07 00:00:00 UTC)),
                (1, datetime!(1917-11-07 00:01:00 UTC)),
                (60, datetime!(1917-11-08 00:00:00 UTC)),
                (121, datetime!(1917-11-09 00:01:00 UTC)),
            ],
        ),
        (
            "0 3 6-8 * * *",
            &[
                (0, datetime!(1917-11-07 06:03:00 UTC)),
                (1, datetime!(1917-11-07 07:03:00 UTC)),
                (2, datetime!(1917-11-07 08:03:00 UTC)),
                (3, datetime!(1917-11-08 06:03:00 UTC)),
                (6, datetime!(1917-11-09 06:03:00 UTC)),
                (10, datetime!(1917-11-10 07:03:00 UTC)),
                (14, datetime!(1917-11-11 08:03:00 UTC)),
            ],
        ),
        (
            "0 3 2,8 * * *",
            &[
                (0, datetime!(1917-11-07 02:03:00 UTC)),
                (1, datetime!(1917-11-07 08:03:00 UTC)),
                (2, datetime!(1917-11-08 02:03:00 UTC)),
                (4, datetime!(1917-11-09 02:03:00 UTC)),
                (7, datetime!(1917-11-10 08:03:00 UTC)),
            ],
        ),
    ];

    for (i, os) in ios {
        let mut ct = crontime::build(now, i).expect("build").enumerate();

        for (n, o) in *os {
            let oo = ct
                .find_map(|(nn, oo)| (nn == *n).then_some(oo))
                .expect("find");

            println!("{i} | {n} | {o}");
            assert_eq!(oo, *o);
        }

        println!();
    }
}

fn assert(i: &'static str, os: &[(usize, time::OffsetDateTime)]) {
    let mut ct = crontime::build(ORIGIN, i).expect("build").enumerate();

    for (n, o) in os {
        let oo = ct
            .find_map(|(nn, oo)| (nn == *n).then_some(oo))
            .expect("find");

        assert_eq!(oo, *o);
    }
}
