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

    fn eday(&self) -> bitvec::BitArr!(for 31) {
        dbg!(self.o);
        let offset = usize::from(
            self.o
                .replace_day(1)
                .expect("day")
                .weekday()
                .number_days_from_sunday(),
        );

        if self.e.dayw.first_zero().is_none() {
            return self.e.daym;
        }

        let mut dayw = bitvec::array::BitArray::ZERO;

        dayw[..7 - offset].copy_from_bitslice(&self.e.dayw[offset..7]);
        let mut cursor = &mut dayw[7 - offset..usize::from(self.o.month().length(self.o.year()))];

        while cursor.len() > 0 {
            let i = 7.min(cursor.len());
            cursor[..i].copy_from_bitslice(&self.e.dayw[..i]);
            cursor = &mut cursor[i..];
        }

        if self.e.daym.first_zero().is_none() {
            return dayw;
        }

        dayw | self.e.daym
    }

    fn step_day_and_month(&mut self) {
        let max = usize::from(self.o.month().length(self.o.year()));
        let (next_daym, loop_daym) = next(self.o.day() as usize, max, &self.eday());

        if loop_daym {
            let (next_month, loop_month) = next(self.o.month() as usize, 12, &self.e.month);

            self.o = self
                .o
                .replace_day(1)
                .expect("hour")
                .replace_month(time::Month::try_from(next_month as u8 + 1).unwrap())
                .expect("month");

            if loop_month {
                self.o = self.o.replace_year(self.o.year() + 1).expect("year");
            }

            loop {
                let max = usize::from(self.o.month().length(self.o.year()));
                let (next_daym, loop_daym) = next(self.o.day() as usize - 1, max, &self.eday());

                if !loop_daym && next_daym == usize::from(self.o.day() - 1) {
                    break;
                }

                if loop_daym {
                    self.o = self
                        .o
                        .replace_day(1)
                        .expect("hour")
                        .replace_month(self.o.month().next())
                        .expect("month");

                    if self.o.month() as u8 == 1 {
                        self.o = self.o.replace_year(self.o.year() + 1).expect("year");
                    }
                } else {
                    self.o = self.o.replace_day(next_daym as u8 + 1).expect("hour");
                    break;
                }
            }
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
