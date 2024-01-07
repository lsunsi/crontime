use time::{macros::datetime, PrimitiveDateTime};

#[test]
fn test() {
    let now = datetime!(1917-11-07 00:00:00);

    let ios: &[(&str, &[(usize, PrimitiveDateTime)])] = &[
        (
            "* * * * *",
            &[
                (0, datetime!(1917-11-07 00:01:00)),
                (1, datetime!(1917-11-07 00:02:00)),
            ],
        ),
        (
            "0 * * * *",
            &[
                (0, datetime!(1917-11-07 01:00:00)),
                (1, datetime!(1917-11-07 02:00:00)),
            ],
        ),
        (
            "7 * * * *",
            &[
                (0, datetime!(1917-11-07 00:07:00)),
                (1, datetime!(1917-11-07 01:07:00)),
            ],
        ),
        (
            "2-4 * * * *",
            &[
                (0, datetime!(1917-11-07 00:02:00)),
                (1, datetime!(1917-11-07 00:03:00)),
                (2, datetime!(1917-11-07 00:04:00)),
                (3, datetime!(1917-11-07 01:02:00)),
            ],
        ),
        (
            "2,4 * * * *",
            &[
                (0, datetime!(1917-11-07 00:02:00)),
                (1, datetime!(1917-11-07 00:04:00)),
                (2, datetime!(1917-11-07 01:02:00)),
            ],
        ),
        (
            "* 0 * * *",
            &[
                (0, datetime!(1917-11-07 00:01:00)),
                (1, datetime!(1917-11-07 00:02:00)),
                (59, datetime!(1917-11-08 00:00:00)),
                (120, datetime!(1917-11-09 00:01:00)),
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
