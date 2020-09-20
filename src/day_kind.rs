use chrono::{DateTime, Datelike, Duration, Weekday};
use chrono_tz::Tz;
use std::{iter, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayKind {
    Weekday,
    DayBeforeHoliday,
    Holiday,
}

trait HasDayKind {
    fn day_kind(&self) -> DayKind;
}

impl<D> HasDayKind for D
where
    D: Datelike,
{
    fn day_kind(&self) -> DayKind {
        let weekday = self.weekday();
        if weekday == Weekday::Sun {
            return DayKind::Holiday;
        }

        let self_ord = self.ordinal();
        let (_, next_holiday) = super::next_upcoming_holiday(self);
        let next_holiday_ord = next_holiday.ordinal();

        if self_ord == next_holiday_ord {
            DayKind::Holiday
        } else if self_ord == next_holiday_ord - 1 || weekday == Weekday::Sat {
            DayKind::DayBeforeHoliday
        } else {
            DayKind::Weekday
        }
    }
}

#[derive(Clone)]
struct SliceIterator {
    // Stepped forward.
    start: chrono::NaiveDateTime,
    end: chrono::NaiveDateTime,
}

impl iter::Iterator for SliceIterator {
    type Item = DayKindSlice;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end <= self.start {
            return None;
        }

        let start_kind = dbg!(self.start.day_kind());
        let mut step = self.start;

        loop {
            let next_day = (step.date() + Duration::days(1)).and_hms(0, 0, 0);

            // We reached the end of given range.
            if self.end < next_day {
                let res = DayKindSlice {
                    range: (self.start..self.end),
                    kind: start_kind,
                };

                // move start forward to mark end.
                self.start = self.end;
                return Some(res);
            }

            if next_day.day_kind() != start_kind {
                let res = DayKindSlice {
                    range: (self.start..next_day),
                    kind: start_kind,
                };
                self.start = next_day;
                return Some(res);
            }

            step = next_day;
        }
    }
}

/// Returns an iterator of DayKindSlices.
pub fn slice_on_day_kind(range: Range<DateTime<Tz>>) -> impl Iterator<Item = DayKindSlice> {
    SliceIterator {
        start: range.start.naive_local(),
        end: range.end.naive_local(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayKindSlice {
    pub range: Range<chrono::NaiveDateTime>,
    pub kind: DayKind,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use chrono_tz::Europe::Stockholm;

    #[test]
    fn slice_a_single_weekday() {
        let start = Stockholm.ymd(2020, 9, 17).and_hms(0, 0, 0);
        let end = Stockholm.ymd(2020, 9, 18).and_hms(23, 59, 59);

        let mut iter = slice_on_day_kind(start..end);

        assert_eq!(
            Some(DayKindSlice {
                range: start.naive_local()..end.naive_local(),
                kind: DayKind::Weekday
            }),
            iter.next(),
            "Should have seen a single slice over a single day"
        );

        assert!(
            iter.next().is_none(),
            "Expected only a single slice. But got another"
        );
    }

    #[test]
    fn slice_friday_to_monday() {
        let start = Stockholm.ymd(2020, 9, 18).and_hms(0, 0, 0); // Friday
        let end = Stockholm.ymd(2020, 9, 21).and_hms(13, 15, 0); // Monday at 13:15

        let mut iter = slice_on_day_kind(start..end);

        assert_eq!(
            Some(DayKindSlice {
                range: start.naive_local()
                    ..Stockholm.ymd(2020, 9, 19).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Weekday,
            }),
            iter.next(),
            "First slice should be the whole of Friday"
        );

        assert_eq!(
            Some(DayKindSlice {
                range: Stockholm.ymd(2020, 9, 19).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 9, 20).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::DayBeforeHoliday,
            }),
            iter.next(),
            "Second slice should be the whole of Saturday"
        );

        assert_eq!(
            Some(DayKindSlice {
                range: Stockholm.ymd(2020, 9, 20).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 9, 21).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Holiday,
            }),
            iter.next(),
            "Third slice should be the whole of Sunday"
        );

        assert_eq!(
            Some(DayKindSlice {
                range: Stockholm.ymd(2020, 9, 21).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 9, 21).and_hms(13, 15, 0).naive_local(),
                kind: DayKind::Weekday,
            }),
            iter.next(),
            "Fourth slice should be Monday until 13:15"
        );

        assert!(
            iter.next().is_none(),
            "Iterator should be empty after monday"
        );
    }

    #[test]
    fn test_slice_over_easter() {
        let start = Stockholm.ymd(2020, 4, 8).and_hms(0, 0, 0); // Wed before good friday.
        let end = Stockholm.ymd(2020, 4, 15).and_hms(0, 0, 0); // Wed after Easter.

        let mut iter = slice_on_day_kind(start..end);

        // Wednesday
        assert_eq!(
            DayKindSlice {
                range: start.naive_local()
                    ..Stockholm.ymd(2020, 4, 9).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Weekday
            },
            iter.next().unwrap()
        );

        // Thursday before Good Friday(Skärtorsdagen in swedish)
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 4, 9).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 4, 10).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::DayBeforeHoliday,
            },
            iter.next().unwrap()
        );

        // Good friday
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 4, 10).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 4, 11).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Holiday,
            },
            iter.next().unwrap()
        );

        // Saturday between good friday and easter day.
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 4, 11).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 4, 12).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::DayBeforeHoliday,
            },
            iter.next().unwrap()
        );

        // Easter day and `Annandag påsk`
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 4, 12).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 4, 14).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Holiday,
            },
            iter.next().unwrap()
        );

        // The lonely, utterly unspecial wednesday ending the range.
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 4, 14).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 4, 15).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Weekday,
            },
            iter.next().unwrap()
        );

        assert!(iter.next().is_none(), "No more slices should have existed");
    }

    #[test]
    fn test_slice_over_christmas() {
        // Monday 2 days before christmas
        let start = Stockholm.ymd(2020, 12, 21).and_hms(0, 0, 0);

        // Tuesday week after christmas. 2 days before new years eve.
        let end = Stockholm.ymd(2020, 12, 29).and_hms(0, 0, 0);

        let mut iter = slice_on_day_kind(start..end);

        // The 2 normal weekdays before the 23rd
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 12, 21).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 12, 23).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Weekday,
            },
            iter.next().unwrap()
        );

        // The 23rd, day before christmas
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 12, 23).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 12, 24).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::DayBeforeHoliday,
            },
            iter.next().unwrap()
        );

        // The 24th (a thursday), all the way to monday.
        // since 25th and 26th are holidays, and all sundays are holidays.
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 12, 24).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 12, 28).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Holiday,
            },
            iter.next().unwrap()
        );

        // The monday after christmas weekend.
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 12, 28).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 12, 29).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Weekday,
            },
            iter.next().unwrap()
        );

        assert!(iter.next().is_none(), "No more day kinds exist in slice");
    }

    #[test]
    fn slice_over_new_years() {
        let start = Stockholm.ymd(2020, 12, 29).and_hms(0, 0, 0);
        let end = Stockholm.ymd(2021, 1, 8).and_hms(0, 0, 0);
        let mut iter = slice_on_day_kind(start..end);

        // The tuesday, 2 days before new years.
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 12, 29).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 12, 30).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Weekday,
            },
            iter.next().unwrap()
        );

        // The wednesday before new years eve.
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 12, 30).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2020, 12, 31).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::DayBeforeHoliday,
            },
            iter.next().unwrap()
        );

        // New years eve and new years day.
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2020, 12, 31).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2021, 1, 2).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Holiday,
            },
            iter.next().unwrap()
        );

        // The Saturday after new years.
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2021, 1, 2).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2021, 1, 3).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::DayBeforeHoliday,
            },
            iter.next().unwrap()
        );

        // The Sunday after new years.
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2021, 1, 3).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2021, 1, 4).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Holiday,
            },
            iter.next().unwrap()
        );

        // Monday, the 4th
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2021, 1, 4).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2021, 1, 5).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Weekday,
            },
            iter.next().unwrap()
        );

        // Tuesday, the 5th. Day before Trettondagsafton
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2021, 1, 5).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2021, 1, 6).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::DayBeforeHoliday,
            },
            iter.next().unwrap()
        );

        // Wednesday, the 6th. Trettondagsafton.
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2021, 1, 6).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2021, 1, 7).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Holiday,
            },
            iter.next().unwrap()
        );

        // Thursday, the 7th
        assert_eq!(
            DayKindSlice {
                range: Stockholm.ymd(2021, 1, 7).and_hms(0, 0, 0).naive_local()
                    ..Stockholm.ymd(2021, 1, 8).and_hms(0, 0, 0).naive_local(),
                kind: DayKind::Weekday,
            },
            iter.next().unwrap()
        );

        assert!(iter.next().is_none(), "No more time exists in slice");
    }
}
