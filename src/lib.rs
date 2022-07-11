mod day_kind;

use chrono::{Date, Datelike, Duration, TimeZone, Weekday};
use chrono_tz::{Europe::Stockholm, Tz};
use std::{fmt, iter, str};

#[cfg(feature = "serde")]
mod serde;

pub use day_kind::{day_kind, slice_on_day_kind, DayKind, DayKindSlice};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Holiday {
    Nyarsdagen,
    TrettondedagJul,
    Langfredagen,
    Paskdagen,
    AnnandagPask,
    ForstaMaj,
    KristiHimmelfardsdag,
    Pingstdagen,
    Nationaldagen,
    Midsommarafton,
    Midsommardagen,
    AllaHelgonsDag,
    Julafton,
    Juldagen,
    AnnandagJul,
    Nyarsafton,
}

impl Holiday {
    pub fn in_year(self, year: i32) -> Date<Tz> {
        match self {
            Self::Nyarsdagen => Stockholm.ymd(year, 1, 1),
            Self::TrettondedagJul => Stockholm.ymd(year, 1, 6),
            Self::ForstaMaj => Stockholm.ymd(year, 5, 1),
            Self::Nationaldagen => Stockholm.ymd(year, 6, 6),
            Self::Julafton => Stockholm.ymd(year, 12, 24),
            Self::Juldagen => Stockholm.ymd(year, 12, 25),
            Self::AnnandagJul => Stockholm.ymd(year, 12, 26),
            Self::Nyarsafton => Stockholm.ymd(year, 12, 31),

            // Weekday related stuff.
            Self::Midsommarafton => closest_next(Stockholm.ymd(year, 6, 19), Weekday::Fri),
            Self::Midsommardagen => closest_next(Stockholm.ymd(year, 6, 20), Weekday::Sat),

            // Saturday between 31 oct - 6 nov.
            Self::AllaHelgonsDag => closest_next(Stockholm.ymd(year, 10, 31), Weekday::Sat),

            // Easter related stuff.....
            Self::Langfredagen => easter_day_for_year(year) - Duration::days(2),
            Self::Paskdagen => easter_day_for_year(year),
            Self::AnnandagPask => easter_day_for_year(year) + Duration::days(1),
            Self::KristiHimmelfardsdag => {
                easter_day_for_year(year) + Duration::weeks(5) + Duration::days(4)
            }
            Self::Pingstdagen => easter_day_for_year(year) + Duration::weeks(7),
        }
    }
}

impl fmt::Display for Holiday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Holiday::*;
        match self {
            Nyarsdagen => f.write_str("Nyårsdagen"),
            TrettondedagJul => f.write_str("Trettondedag jul"),
            Langfredagen => f.write_str("Långfredagen"),
            Paskdagen => f.write_str("Påskdagen"),
            AnnandagPask => f.write_str("Annandag påsk"),
            ForstaMaj => f.write_str("Första maj"),
            KristiHimmelfardsdag => f.write_str("Kristi himmelsfärdsdag"),
            Pingstdagen => f.write_str("Pingstdagen"),
            Nationaldagen => f.write_str("Nationaldagen"),
            Midsommarafton => f.write_str("Midsommarafton"),
            Midsommardagen => f.write_str("Midsommardagen"),
            AllaHelgonsDag => f.write_str("Alla helgons dag"),
            Julafton => f.write_str("Julafton"),
            Juldagen => f.write_str("Juldagen"),
            AnnandagJul => f.write_str("Annandag jul"),
            Nyarsafton => f.write_str("Nyårsafton"),
        }
    }
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("Invalid Holiday")]
pub struct FromStrError;

impl str::FromStr for Holiday {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Holiday::*;

        // We can use ignore ascii case since no big chars are utf8
        match s.trim() {
            s if s.eq_ignore_ascii_case("Nyårsdagen") => Ok(Nyarsdagen),
            s if s.eq_ignore_ascii_case("Trettondedag jul") => Ok(TrettondedagJul),
            s if s.eq_ignore_ascii_case("Långfredagen") => Ok(Langfredagen),
            s if s.eq_ignore_ascii_case("Påskdagen") => Ok(Paskdagen),
            s if s.eq_ignore_ascii_case("Annandag påsk") => Ok(AnnandagPask),
            s if s.eq_ignore_ascii_case("Första maj") => Ok(ForstaMaj),
            s if s.eq_ignore_ascii_case("Kristi himmelsfärdsdag") => Ok(KristiHimmelfardsdag),
            s if s.eq_ignore_ascii_case("Pingstdagen") => Ok(Pingstdagen),
            s if s.eq_ignore_ascii_case("Nationaldagen") => Ok(Nationaldagen),
            s if s.eq_ignore_ascii_case("Midsommarafton") => Ok(Midsommarafton),
            s if s.eq_ignore_ascii_case("Midsommardagen") => Ok(Midsommardagen),
            s if s.eq_ignore_ascii_case("Alla helgons dag") => Ok(AllaHelgonsDag),
            s if s.eq_ignore_ascii_case("Julafton") => Ok(Julafton),
            s if s.eq_ignore_ascii_case("Juldagen") => Ok(Juldagen),
            s if s.eq_ignore_ascii_case("Annandag jul") => Ok(AnnandagJul),
            s if s.eq_ignore_ascii_case("Nyårsafton") => Ok(Nyarsafton),

            _ => Err(FromStrError),
        }
    }
}

/// Magic function. No Idea what it does. Just know that i works.
#[allow(clippy::many_single_char_names)]
pub fn easter_day_for_year(year: i32) -> Date<Tz> {
    let a = year % 19;
    let b = year / 100;
    let c = (b - (b / 4) - ((8 * b + 13) / 25) + (19 * a) + 15) % 30;
    let d = c - (c / 28) * (1 - (c / 28) * (29 / (c + 1)) * ((21 - a) / 11));
    let e = d - ((year + (year / 4) + d + 2 - b + (b / 4)) % 7);
    let month = 3 + ((e + 40) / 44);
    let day = e + 28 - (31 * (month / 4));

    Stockholm.ymd(year, month as u32, day as u32)
}

pub fn next_upcoming_holiday<D>(date: &D) -> (Holiday, Date<Tz>)
where
    D: Datelike,
{
    let day_ordinal = date.ordinal();
    holidays_in_year(date.year())
        .find(|(_, d)| day_ordinal <= d.ordinal())
        // We're only considering Dates, not time here. So it's impossible to express a day
        // later than 31st december - and since 31st dec is New years eve, it is always last.
        // This expect is here to catch any faulty reasoning.
        .expect("Next upcoming holiday was somehow not found. This is unexpected!")
}

// Option to mark that we've yielded the last.
#[derive(Clone, Copy)]
struct Holidays(Option<Holiday>);

impl Default for Holidays {
    fn default() -> Self {
        Self(Some(Holiday::Nyarsdagen))
    }
}

pub fn holidays() -> impl Iterator<Item = Holiday> + Clone + Copy {
    Holidays::default()
}

pub fn holidays_in_year(year: i32) -> impl Iterator<Item = (Holiday, Date<Tz>)> + Clone {
    holidays().map(move |h| (h, h.in_year(year)))
}

impl iter::Iterator for Holidays {
    type Item = Holiday;

    fn next(&mut self) -> Option<Self::Item> {
        use Holiday::*;

        match self.0 {
            Some(Nyarsdagen) => {
                self.0 = Some(TrettondedagJul);
                Some(Nyarsdagen)
            }
            Some(TrettondedagJul) => {
                self.0 = Some(Langfredagen);
                Some(TrettondedagJul)
            }

            Some(Langfredagen) => {
                self.0 = Some(Paskdagen);
                Some(Langfredagen)
            }

            Some(Paskdagen) => {
                self.0 = Some(AnnandagPask);
                Some(Paskdagen)
            }

            Some(AnnandagPask) => {
                self.0 = Some(ForstaMaj);
                Some(AnnandagPask)
            }

            Some(ForstaMaj) => {
                self.0 = Some(Holiday::KristiHimmelfardsdag);
                Some(ForstaMaj)
            }

            Some(KristiHimmelfardsdag) => {
                self.0 = Some(Holiday::Pingstdagen);
                Some(KristiHimmelfardsdag)
            }

            Some(Pingstdagen) => {
                self.0 = Some(Nationaldagen);
                Some(Pingstdagen)
            }

            Some(Nationaldagen) => {
                self.0 = Some(Midsommarafton);
                Some(Nationaldagen)
            }

            Some(Midsommarafton) => {
                self.0 = Some(Midsommardagen);
                Some(Midsommarafton)
            }

            Some(Midsommardagen) => {
                self.0 = Some(AllaHelgonsDag);
                Some(Midsommardagen)
            }

            Some(AllaHelgonsDag) => {
                self.0 = Some(Julafton);
                Some(AllaHelgonsDag)
            }

            Some(Julafton) => {
                self.0 = Some(Juldagen);
                Some(Julafton)
            }

            Some(Juldagen) => {
                self.0 = Some(AnnandagJul);
                Some(Juldagen)
            }

            Some(AnnandagJul) => {
                self.0 = Some(Nyarsafton);
                Some(AnnandagJul)
            }

            Some(Nyarsafton) => {
                self.0 = None;
                Some(Nyarsafton)
            }

            None => None,
        }
    }
}

/// Jumps to the closest next coming day of target weekday
fn closest_next(d: Date<Tz>, target: chrono::Weekday) -> Date<Tz> {
    let days_left_in_week = 7 - d.weekday().num_days_from_monday();
    let days_to_jump = (target.num_days_from_monday() + days_left_in_week) % 7;
    d + Duration::days(days_to_jump.into())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_jumping_between_weekdays() {
        assert_eq!(
            Stockholm.ymd(2020, 9, 20),
            closest_next(Stockholm.ymd(2020, 9, 14), chrono::Weekday::Sun)
        );

        assert_eq!(
            Stockholm.ymd(2020, 9, 26),
            closest_next(Stockholm.ymd(2020, 9, 20), chrono::Weekday::Sat)
        );

        assert_eq!(
            Stockholm.ymd(2020, 9, 22),
            closest_next(Stockholm.ymd(2020, 9, 22), chrono::Weekday::Tue)
        );
    }

    #[test]
    fn test_jumping_to_kristi_flygare() {
        let easter = easter_day_for_year(2020);
        assert_eq!(
            Stockholm.ymd(2020, 5, 21),
            easter + Duration::weeks(5) + Duration::days(4)
        );
    }

    #[test]
    fn test_jumping_to_pingstdagen() {
        let easter = easter_day_for_year(2020);
        assert_eq!(Stockholm.ymd(2020, 5, 31), easter + Duration::weeks(7));
    }

    #[test]
    fn test_next_upcoming_holiday() {
        assert_eq!(
            (Holiday::Nationaldagen, Stockholm.ymd(2020, 6, 6)),
            super::next_upcoming_holiday(&Stockholm.ymd(2020, 6, 5))
        );

        assert_eq!(
            (Holiday::Langfredagen, Stockholm.ymd(2020, 4, 10)),
            super::next_upcoming_holiday(&Stockholm.ymd(2020, 3, 29))
        );

        assert_eq!(
            (Holiday::Nyarsafton, Stockholm.ymd(2020, 12, 31)),
            super::next_upcoming_holiday(&Stockholm.ymd(2020, 12, 31))
        );

        assert_eq!(
            (Holiday::Nyarsdagen, Stockholm.ymd(2020, 1, 1)),
            super::next_upcoming_holiday(&Stockholm.ymd(2020, 1, 1))
        );
    }
}
