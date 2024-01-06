use nom::Parser;

pub struct Crontime {
    o: time::PrimitiveDateTime,
    minute: bitvec::BitArr!(for 60),
    hour: bitvec::BitArr!(for 24),
    daym: bitvec::BitArr!(for 31),
    month: bitvec::BitArr!(for 12),
    dayw: bitvec::BitArr!(for 7),
}

pub fn build(origin: time::PrimitiveDateTime, expr: &str) -> Result<Crontime, nom::Err<()>> {
    use nom::{
        character::complete::{char, u8},
        combinator::map,
        multi::separated_list1,
        sequence::{separated_pair, tuple},
    };

    enum P {
        Any,
        Single(u8),
        Range((u8, u8)),
        Many(Vec<u8>),
    }

    impl P {
        fn render(self, bits: &mut bitvec::slice::BitSlice) {
            match self {
                P::Any => bits.fill(true),
                P::Single(i) => bits.set(i as usize, true),
                P::Range((i, j)) => bits[i as usize..j as usize].fill(true),
                P::Many(is) => {
                    for i in is {
                        bits.set(i as usize, true);
                    }
                }
            }
        }
    }

    tuple((
        map(u8, P::Single)
            .or(map(char('*'), |_| P::Any))
            .or(map(separated_list1(char(','), u8), P::Many))
            .or(map(separated_pair(u8, char('-'), u8), P::Range)),
        char(' '),
        map(u8, P::Single)
            .or(map(char('*'), |_| P::Any))
            .or(map(separated_list1(char(','), u8), P::Many))
            .or(map(separated_pair(u8, char('-'), u8), P::Range)),
        char(' '),
        map(u8, P::Single)
            .or(map(char('*'), |_| P::Any))
            .or(map(separated_list1(char(','), u8), P::Many))
            .or(map(separated_pair(u8, char('-'), u8), P::Range)),
        char(' '),
        map(u8, P::Single)
            .or(map(char('*'), |_| P::Any))
            .or(map(separated_list1(char(','), u8), P::Many))
            .or(map(separated_pair(u8, char('-'), u8), P::Range)),
        char(' '),
        map(u8, P::Single)
            .or(map(char('*'), |_| P::Any))
            .or(map(separated_list1(char(','), u8), P::Many))
            .or(map(separated_pair(u8, char('-'), u8), P::Range)),
    ))(expr)
    .map(|(_, (minutep, _, hourp, _, daymp, _, monthp, _, daywp))| {
        let mut minute = bitvec::array::BitArray::ZERO;
        minutep.render(&mut minute);

        let mut hour = bitvec::array::BitArray::ZERO;
        hourp.render(&mut hour);

        let mut daym = bitvec::array::BitArray::ZERO;
        daymp.render(&mut daym);

        let mut month = bitvec::array::BitArray::ZERO;
        monthp.render(&mut month);

        let mut dayw = bitvec::array::BitArray::ZERO;
        daywp.render(&mut dayw);

        Crontime {
            o: origin,
            minute,
            hour,
            daym,
            month,
            dayw,
        }
    })
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
                "* * * * *",
                &[
                    datetime!(1917-11-07 00:01:00),
                    datetime!(1917-11-07 00:02:00),
                ],
            ),
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
            let mut ct = super::build(now, i).unwrap();

            for o in os {
                println!("{i} {o}");
                assert_eq!(ct.next().unwrap(), *o);
            }

            println!();
        }
    }
}
