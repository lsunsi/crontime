pub struct Crontime {
    o: time::PrimitiveDateTime,
    minute: u8,
    _hour: (),
    _daym: (),
    _month: (),
    _dayw: (),
}

impl TryFrom<(time::PrimitiveDateTime, &str)> for Crontime {
    type Error = nom::Err<()>;

    fn try_from((o, s): (time::PrimitiveDateTime, &str)) -> Result<Self, Self::Error> {
        use nom::{
            character::complete::{char, u8},
            sequence::tuple,
        };

        tuple((
            u8,
            char(' '),
            char('*'),
            char(' '),
            char('*'),
            char(' '),
            char('*'),
            char(' '),
            char('*'),
        ))(s)
        .map(|(_, (minute, _, _, _, _, _, _, _, _))| Crontime {
            o,
            minute,
            _hour: (),
            _daym: (),
            _month: (),
            _dayw: (),
        })
    }
}

impl Iterator for Crontime {
    type Item = time::PrimitiveDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        let o0 = self.o;

        self.o = self.o.replace_minute(self.minute).unwrap();

        if self.o <= o0 {
            self.o += time::Duration::hours(1);
        }

        Some(self.o)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    #[test]
    fn test() {
        let now = datetime!(1917-11-07 00:00:00);

        let ios = [
            (
                "0 * * * *",
                &[
                    datetime!(1917-11-07 01:00:00),
                    datetime!(1917-11-07 02:00:00),
                ],
            ),
            (
                "7 * * * *",
                &[
                    datetime!(1917-11-07 00:07:00),
                    datetime!(1917-11-07 01:07:00),
                ],
            ),
        ];

        for (i, os) in ios {
            let mut ct: super::Crontime = (now, i).try_into().unwrap();

            for o in os {
                println!("{i} {o}");
                assert_eq!(ct.next().unwrap(), *o);
            }

            println!();
        }
    }
}
