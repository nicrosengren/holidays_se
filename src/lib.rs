use chrono::{Date, Datelike, Duration, TimeZone, Weekday};
use chrono_tz::{Europe::Stockholm, Tz};
use std::{fmt, iter};

pub type Error = &'static str;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayKind {
    Weekday,
    DayBeforeHoliday,
    Holiday,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            // Easter related stuff.....
            Self::Langfredagen => easter_day_for_year(year) - Duration::days(2),
            Self::Paskdagen => easter_day_for_year(year),
            Self::AnnandagPask => easter_day_for_year(year) + Duration::days(1),
            Self::KristiHimmelfardsdag => {
                easter_day_for_year(year) + Duration::weeks(5) + Duration::days(4)
            }
            Self::Pingstdagen => easter_day_for_year(year) + Duration::weeks(7),

            // Weekday related stuff.
            Self::Midsommarafton => closest_next(Stockholm.ymd(year, 6, 19), Weekday::Fri),
            Self::Midsommardagen => closest_next(Stockholm.ymd(year, 6, 20), Weekday::Sat),

            // Saturday between 31 oct - 6 nov.
            Self::AllaHelgonsDag => closest_next(Stockholm.ymd(year, 10, 31), Weekday::Sat),
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
            KristiHimmelfardsdag => f.write_str(""),
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

#[derive(Clone, Copy)]
struct Holidays {
    next: Option<Holiday>,
}

pub fn holidays() -> impl Iterator<Item = Holiday> + Clone + Copy {
    Holidays {
        next: Some(Holiday::Nyarsdagen),
    }
}

pub fn holidays_in_year(year: i32) -> impl Iterator<Item = (Holiday, Date<Tz>)> + Clone {
    holidays().map(move |h| (h, h.in_year(year)))
}

impl iter::Iterator for Holidays {
    type Item = Holiday;

    fn next(&mut self) -> Option<Self::Item> {
        use Holiday::*;

        match self.next.take() {
            Some(Nyarsdagen) => {
                self.next = Some(TrettondedagJul);
                Some(Nyarsdagen)
            }
            Some(TrettondedagJul) => {
                self.next = Some(Langfredagen);
                Some(TrettondedagJul)
            }

            Some(Langfredagen) => {
                self.next = Some(Paskdagen);
                Some(Langfredagen)
            }

            Some(Paskdagen) => {
                self.next = Some(AnnandagPask);
                Some(Paskdagen)
            }

            Some(AnnandagPask) => {
                self.next = Some(ForstaMaj);
                Some(AnnandagPask)
            }

            Some(ForstaMaj) => {
                self.next = Some(Holiday::KristiHimmelfardsdag);
                Some(ForstaMaj)
            }

            Some(KristiHimmelfardsdag) => {
                self.next = Some(Holiday::Pingstdagen);
                Some(KristiHimmelfardsdag)
            }

            Some(Pingstdagen) => {
                self.next = Some(Nationaldagen);
                Some(Pingstdagen)
            }

            Some(Nationaldagen) => {
                self.next = Some(Midsommarafton);
                Some(Nationaldagen)
            }

            Some(Midsommarafton) => {
                self.next = Some(Midsommardagen);
                Some(Midsommarafton)
            }

            Some(Midsommardagen) => {
                self.next = Some(AllaHelgonsDag);
                Some(Midsommardagen)
            }

            Some(AllaHelgonsDag) => {
                self.next = Some(Julafton);
                Some(AllaHelgonsDag)
            }

            Some(Julafton) => {
                self.next = Some(Juldagen);
                Some(Julafton)
            }

            Some(Juldagen) => {
                self.next = Some(AnnandagJul);
                Some(Juldagen)
            }

            Some(AnnandagJul) => {
                self.next = Some(Nyarsafton);
                Some(AnnandagJul)
            }

            Some(Nyarsafton) => {
                self.next = None;
                Some(Nyarsafton)
            }

            None => None,
        }
    }
}

// returns the number of days needed to jump forward in order to reach a given
// weekday.
pub fn closest_next(d: Date<Tz>, target: chrono::Weekday) -> Date<Tz> {
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
}
