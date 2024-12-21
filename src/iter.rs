impl Iterator for super::Crontime {
    type Item = time::OffsetDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        let (next_second, loop_second) = next(self.o.second() as usize + 1, 60, &self.e.second);
        self.o = self.o.replace_second(next_second as u8).expect("second");

        if loop_second {
            let (next_minute, loop_minute) = next(self.o.minute() as usize + 1, 60, &self.e.minute);
            self.o = self.o.replace_minute(next_minute as u8).expect("minute");

            if loop_minute {
                let (next_hour, loop_hour) = next(self.o.hour() as usize + 1, 24, &self.e.hour);
                self.o = self.o.replace_hour(next_hour as u8).expect("hour");

                if loop_hour {
                    self.o += time::Duration::days(1);
                }
            }
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
