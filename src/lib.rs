pub struct Crontime {
    o: time::PrimitiveDateTime,
    minute: bitvec::BitArr!(for 60),
    _hour: bitvec::BitArr!(for 24),
    _daym: bitvec::BitArr!(for 31),
    _month: bitvec::BitArr!(for 12),
    _dayw: bitvec::BitArr!(for 7),
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
        .map(|(_, (min, _, _, _, _, _, _, _, _))| {
            let mut minute = bitvec::array::BitArray::ZERO;
            minute.set(min as usize, true);

            Crontime {
                o,
                minute,
                _hour: bitvec::array::BitArray::ZERO,
                _daym: bitvec::array::BitArray::ZERO,
                _month: bitvec::array::BitArray::ZERO,
                _dayw: bitvec::array::BitArray::ZERO,
            }
        })
    }
}

impl Iterator for Crontime {
    type Item = time::PrimitiveDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        let o0 = self.o;

        for m in o0.minute() + 1..60 {
            if self.minute[m as usize] {
                self.o = self.o.replace_minute(m).unwrap();
                break;
            }
        }

        if self.o <= o0 {
            self.o += time::Duration::hours(1);

            for m in 0..60 {
                if self.minute[m as usize] {
                    self.o = self.o.replace_minute(m).unwrap();
                    break;
                }
            }
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
