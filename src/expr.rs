use nom::{
    character::complete::{char, u8},
    combinator::{eof, map, verify},
    error::ParseError,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    Parser,
};

#[derive(Clone, Copy)]
pub(super) struct Expr {
    pub second: bitvec::BitArr!(for 60),
    pub minute: bitvec::BitArr!(for 60),
    pub hour: bitvec::BitArr!(for 24),
    pub daym: bitvec::BitArr!(for 31),
    pub month: bitvec::BitArr!(for 12),
    pub dayw: bitvec::BitArr!(for 7),
}

impl std::str::FromStr for Expr {
    type Err = nom::Err<()>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        tuple((
            Pat::<0, 60>::parser(),
            char(' '),
            Pat::<0, 60>::parser(),
            char(' '),
            Pat::<0, 24>::parser(),
            char(' '),
            Pat::<1, 32>::parser(),
            char(' '),
            Pat::<1, 13>::parser(),
            char(' '),
            Pat::<0, 7>::parser(),
            eof,
        ))(s)
        .map(
            |(_, (secondp, _, minutep, _, hourp, _, daymp, _, monthp, _, daywp, _))| {
                let mut second = bitvec::array::BitArray::ZERO;
                secondp.render(&mut second);

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

                Expr {
                    second,
                    minute,
                    hour,
                    daym,
                    month,
                    dayw,
                }
            },
        )
    }
}

enum Pat<const MIN: u8, const MAX: u8> {
    Any,
    Single(u8),
    Many(Vec<u8>),
    Range((u8, u8)),
}

impl<const MIN: u8, const MAX: u8> Pat<MIN, MAX> {
    fn u8n<'a, E: ParseError<&'a str>>() -> impl Parser<&'a str, u8, E> {
        verify(u8, |n| (MIN..MAX).contains(n))
    }

    fn parser<'a, E: ParseError<&'a str>>() -> impl Parser<&'a str, Pat<MIN, MAX>, E> {
        map(char('*'), |_| Pat::Any)
            .or(map(
                separated_pair(Self::u8n(), char('-'), Self::u8n()),
                Pat::Range,
            ))
            .or(map(separated_list1(char(','), Self::u8n()), Pat::Many))
            .or(map(Self::u8n(), Pat::Single))
    }

    fn render(self, bits: &mut bitvec::slice::BitSlice) {
        let m = usize::from(MIN);
        match self {
            Pat::Any => bits.fill(true),
            Pat::Single(i) => bits.set(i as usize - m, true),
            Pat::Range((i, j)) => bits[i as usize - m..=j as usize - m].fill(true),
            Pat::Many(is) => {
                for i in is {
                    bits.set(i as usize - m, true);
                }
            }
        }
    }
}
