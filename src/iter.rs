use bitvec::slice::BitSlice;

impl Iterator for super::Crontime {
    type Item = time::PrimitiveDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        let (next_minute, loop_minute) = next(self.o.minute() as usize + 1, 60, &self.e.minute);
        self.o = self.o.replace_minute(next_minute as u8).expect("minute");

        if loop_minute {
            let (next_hour, loop_hour) = next(self.o.hour() as usize + 1, 24, &self.e.hour);
            self.o = self.o.replace_hour(next_hour as u8).expect("hour");

            if loop_hour {
                self.o += time::Duration::days(1);
            }
        }

        Some(self.o)
    }
}

fn next(from: usize, to: usize, bits: &BitSlice) -> (usize, bool) {
    if let Some(i) = bits[from..to].first_one() {
        return (from + i, false);
    };

    if let Some(i) = bits.first_one() {
        return (i, true);
    };

    unreachable!()
}
