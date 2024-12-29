fn next(from: usize, to: usize, bits: &bitvec::slice::BitSlice) -> (usize, bool) {
    if let Some(i) = bits[from..to].first_one() {
        return (from + i, false);
    };

    if let Some(i) = bits.first_one() {
        return (i, true);
    };

    unreachable!()
}

impl super::Crontime {
    fn step_second(&mut self) -> bool {
        let (second, carry) = next(self.o.second() as usize + 1, 60, &self.e.second);
        self.o = self.o.replace_second(second as u8).expect("second");
        carry
    }

    fn step_minute(&mut self) -> bool {
        let (minute, carry) = next(self.o.minute() as usize + 1, 60, &self.e.minute);
        self.o = self.o.replace_minute(minute as u8).expect("minute");
        carry
    }

    fn step_hour(&mut self) -> bool {
        let (hour, carry) = next(self.o.hour() as usize + 1, 24, &self.e.hour);
        self.o = self.o.replace_hour(hour as u8).expect("hour");
        carry
    }

    fn step_day_and_month(&mut self) {
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
        }
    }
}

impl Iterator for super::Crontime {
    type Item = time::OffsetDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step_second() && self.step_minute() && self.step_hour() {
            self.step_day_and_month()
        }

        Some(self.o)
    }
}
