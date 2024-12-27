impl Iterator for super::Crontime {
    type Item = time::OffsetDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        let (next_second, loop_second) = next(self.o.second() as usize + 1, 60, &self.e.second);
        self.o = self.o.replace_second(next_second as u8).expect("second");

        if !loop_second {
            return Some(self.o);
        }

        let (next_minute, loop_minute) = next(self.o.minute() as usize + 1, 60, &self.e.minute);
        self.o = self.o.replace_minute(next_minute as u8).expect("minute");

        if !loop_minute {
            return Some(self.o);
        }

        let (next_hour, loop_hour) = next(self.o.hour() as usize + 1, 24, &self.e.hour);
        self.o = self.o.replace_hour(next_hour as u8).expect("hour");

        if !loop_hour {
            return Some(self.o);
        }

        let max = usize::from(self.o.month().length(self.o.year()));
        let (next_daym, loop_daym) = next(self.o.day() as usize, max, &self.e.daym);

        if loop_daym {
            let (next_month, loop_month) = next(self.o.month() as usize, 12, &self.e.month);

            let mut o = self
                .o
                .replace_day(1)
                .expect("hour")
                .replace_month(time::Month::try_from(next_month as u8 + 1).unwrap())
                .expect("month");

            if loop_month {
                o = o.replace_year(o.year() + 1).expect("year");
            }

            loop {
                let max = usize::from(o.month().length(o.year()));
                let (next_daym, loop_daym) = next(o.day() as usize - 1, max, &self.e.daym);

                if !loop_daym && next_daym == usize::from(o.day()) {
                    break;
                }

                if loop_daym {
                    o = o
                        .replace_day(1)
                        .expect("hour")
                        .replace_month(o.month().next())
                        .expect("month");

                    if o.month() as u8 == 1 {
                        o = o.replace_year(self.o.year() + 1).expect("year");
                    }
                } else {
                    o = o.replace_day(next_daym as u8 + 1).expect("hour");
                    break;
                }
            }

            self.o = o;
        } else {
            self.o = self.o.replace_day(next_daym as u8 + 1).expect("day");
            return Some(self.o);
        }

        Some(self.o)
    }
}

pub(super) fn init(mut o: time::OffsetDateTime, e: super::expr::Expr) -> time::OffsetDateTime {
    let (next_minute, loop_minute) = next(o.minute() as usize, 60, &e.minute);
    o = o.replace_minute(next_minute as u8).expect("minute");

    let (next_hour, _loop_hour) = next(o.hour() as usize + loop_minute as usize, 24, &e.hour);
    o = o.replace_hour(next_hour as u8).expect("hour");

    o - time::Duration::SECOND
}

fn next(from: usize, to: usize, bits: &bitvec::slice::BitSlice) -> (usize, bool) {
    if let Some(i) = bits[from..to].first_one() {
        return (from + i, false);
    };

    if let Some(i) = bits.first_one() {
        return (i, true);
    };

    unreachable!()
}
