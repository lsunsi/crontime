use nom::{
    character::complete::{char, u8},
    combinator::map,
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
    pub _daym: bitvec::BitArr!(for 31),
    pub _month: bitvec::BitArr!(for 12),
    pub _dayw: bitvec::BitArr!(for 7),
}

impl std::str::FromStr for Expr {
    type Err = nom::Err<()>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        tuple((
            Pat::parser(),
            char(' '),
            Pat::parser(),
            char(' '),
            Pat::parser(),
            char(' '),
            Pat::parser(),
            char(' '),
            Pat::parser(),
            char(' '),
            Pat::parser(),
        ))(s)
        .map(
            |(_, (secondp, _, minutep, _, hourp, _, daymp, _, monthp, _, daywp))| {
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
                    _daym: daym,
                    _month: month,
                    _dayw: dayw,
                }
            },
        )
    }
}

enum Pat {
    Any,
    Single(u8),
    Range((u8, u8)),
    Many(Vec<u8>),
}

impl Pat {
    fn parser<'a, E: ParseError<&'a str>>() -> impl Parser<&'a str, Pat, E> {
        map(char('*'), |_| Pat::Any)
            .or(map(separated_pair(u8, char('-'), u8), Pat::Range))
            .or(map(separated_list1(char(','), u8), Pat::Many))
            .or(map(u8, Pat::Single))
    }

    fn render(self, bits: &mut bitvec::slice::BitSlice) {
        match self {
            Pat::Any => bits.fill(true),
            Pat::Single(i) => bits.set(i as usize, true),
            Pat::Range((i, j)) => bits[i as usize..=j as usize].fill(true),
            Pat::Many(is) => {
                for i in is {
                    bits.set(i as usize, true);
                }
            }
        }
    }
}
