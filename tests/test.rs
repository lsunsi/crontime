use time::macros::datetime;

#[test]
fn seconds_any() {
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
fn seconds_single() {
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
fn seconds_many() {
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
fn seconds_range() {
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
    let origin = datetime!(1917-11-07 00:00:00 UTC);

    let mut ct = crontime::build(origin, i).expect("build").enumerate();

    for (n, o) in os {
        let oo = ct
            .find_map(|(nn, oo)| (nn == *n).then_some(oo))
            .expect("find");

        assert_eq!(oo, *o);
    }
}
