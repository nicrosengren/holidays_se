use chrono::{Date, Datelike, Duration, TimeZone, Weekday};
use chrono_tz::{Europe::Stockholm, Tz};
use std::{fmt, iter};

pub type Error = &'static str;

pub type Result<T> = std::result::Result<T, Error>;

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

    /// Used by iterator
    fn tuple(self, year: i32) -> (Self, Date<Tz>) {
        (self, self.in_year(year))
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

#[derive(Clone, Copy, Debug)]
pub struct Holidays {
    year: i32,
    next: Option<Holiday>,
}

impl Holidays {
    pub fn year(year: i32) -> impl Iterator<Item = (Holiday, Date<Tz>)> {
        Self {
            year,
            next: Some(Holiday::Nyarsdagen),
        }
    }
}

impl iter::Iterator for Holidays {
    type Item = (Holiday, Date<Tz>);

    fn next(&mut self) -> Option<Self::Item> {
        use Holiday::*;

        match self.next.take() {
            Some(Nyarsdagen) => {
                self.next = Some(TrettondedagJul);
                Some(Nyarsdagen.tuple(self.year))
            }
            Some(TrettondedagJul) => {
                self.next = Some(Langfredagen);
                Some(TrettondedagJul.tuple(self.year))
            }

            Some(Langfredagen) => {
                self.next = Some(Paskdagen);
                Some(Langfredagen.tuple(self.year))
            }

            Some(Paskdagen) => {
                self.next = Some(AnnandagPask);
                Some(Paskdagen.tuple(self.year))
            }

            Some(AnnandagPask) => {
                self.next = Some(ForstaMaj);
                Some(AnnandagPask.tuple(self.year))
            }

            Some(ForstaMaj) => {
                self.next = Some(Holiday::KristiHimmelfardsdag);
                Some(ForstaMaj.tuple(self.year))
            }

            Some(KristiHimmelfardsdag) => {
                self.next = Some(Holiday::Pingstdagen);
                Some(KristiHimmelfardsdag.tuple(self.year))
            }

            Some(Pingstdagen) => {
                self.next = Some(Nationaldagen);
                Some(Pingstdagen.tuple(self.year))
            }

            Some(Nationaldagen) => {
                self.next = Some(Midsommarafton);
                Some(Nationaldagen.tuple(self.year))
            }

            Some(Midsommarafton) => {
                self.next = Some(Midsommardagen);
                Some(Midsommarafton.tuple(self.year))
            }

            Some(Midsommardagen) => {
                self.next = Some(AllaHelgonsDag);
                Some(Midsommardagen.tuple(self.year))
            }

            Some(AllaHelgonsDag) => {
                self.next = Some(Julafton);
                Some(AllaHelgonsDag.tuple(self.year))
            }

            Some(Julafton) => {
                self.next = Some(Juldagen);
                Some(Julafton.tuple(self.year))
            }

            Some(Juldagen) => {
                self.next = Some(AnnandagJul);
                Some(Juldagen.tuple(self.year))
            }
            Some(AnnandagJul) => {
                self.next = Some(Nyarsafton);
                Some(AnnandagJul.tuple(self.year))
            }
            Some(Nyarsafton) => {
                self.next = None;
                Some(Nyarsafton.tuple(self.year))
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

    #[test]
    fn test_holidays_2020() {
        assert_eq!(Holiday::Nyarsdagen.in_year(2020), Stockholm.ymd(2020, 1, 1));
        assert_eq!(
            Holiday::TrettondedagJul.in_year(2020),
            Stockholm.ymd(2020, 1, 6)
        );
        assert_eq!(
            Holiday::Langfredagen.in_year(2020),
            Stockholm.ymd(2020, 4, 10)
        );
        assert_eq!(Holiday::Paskdagen.in_year(2020), Stockholm.ymd(2020, 4, 12));
        assert_eq!(
            Holiday::AnnandagPask.in_year(2020),
            Stockholm.ymd(2020, 4, 13)
        );
        assert_eq!(Holiday::ForstaMaj.in_year(2020), Stockholm.ymd(2020, 5, 1));
        assert_eq!(
            Holiday::KristiHimmelfardsdag.in_year(2020),
            Stockholm.ymd(2020, 5, 21)
        );
        assert_eq!(
            Holiday::Pingstdagen.in_year(2020),
            Stockholm.ymd(2020, 5, 31)
        );
        assert_eq!(
            Holiday::Nationaldagen.in_year(2020),
            Stockholm.ymd(2020, 6, 6)
        );
        assert_eq!(
            Holiday::Midsommarafton.in_year(2020),
            Stockholm.ymd(2020, 6, 19)
        );
        assert_eq!(
            Holiday::Midsommardagen.in_year(2020),
            Stockholm.ymd(2020, 6, 20)
        );
        assert_eq!(
            Holiday::AllaHelgonsDag.in_year(2020),
            Stockholm.ymd(2020, 10, 31)
        );
        assert_eq!(Holiday::Julafton.in_year(2020), Stockholm.ymd(2020, 12, 24));
        assert_eq!(Holiday::Juldagen.in_year(2020), Stockholm.ymd(2020, 12, 25));
        assert_eq!(
            Holiday::AnnandagJul.in_year(2020),
            Stockholm.ymd(2020, 12, 26)
        );
        assert_eq!(
            Holiday::Nyarsafton.in_year(2020),
            Stockholm.ymd(2020, 12, 31)
        );
    }

    #[test]
    fn test_iterator_2020() {
        use Holiday::*;

        let mut iter = Holidays::year(2020);

        assert_eq!(
            (Nyarsdagen, Stockholm.ymd(2020, 1, 1)),
            iter.next().unwrap()
        );
        assert_eq!(
            (TrettondedagJul, Stockholm.ymd(2020, 1, 6)),
            iter.next().unwrap()
        );
        assert_eq!(
            (Langfredagen, Stockholm.ymd(2020, 4, 10)),
            iter.next().unwrap()
        );
        assert_eq!(
            (Paskdagen, Stockholm.ymd(2020, 4, 12)),
            iter.next().unwrap()
        );
        assert_eq!(
            (AnnandagPask, Stockholm.ymd(2020, 4, 13)),
            iter.next().unwrap()
        );
        assert_eq!((ForstaMaj, Stockholm.ymd(2020, 5, 1)), iter.next().unwrap());
        assert_eq!(
            (KristiHimmelfardsdag, Stockholm.ymd(2020, 5, 21)),
            iter.next().unwrap()
        );
        assert_eq!(
            (Pingstdagen, Stockholm.ymd(2020, 5, 31)),
            iter.next().unwrap()
        );
        assert_eq!(
            (Nationaldagen, Stockholm.ymd(2020, 6, 6)),
            iter.next().unwrap()
        );
        assert_eq!(
            (Midsommarafton, Stockholm.ymd(2020, 6, 19)),
            iter.next().unwrap()
        );
        assert_eq!(
            (Midsommardagen, Stockholm.ymd(2020, 6, 20)),
            iter.next().unwrap()
        );
        assert_eq!(
            (AllaHelgonsDag, Stockholm.ymd(2020, 10, 31)),
            iter.next().unwrap()
        );
        assert_eq!(
            (Julafton, Stockholm.ymd(2020, 12, 24)),
            iter.next().unwrap()
        );
        assert_eq!(
            (Juldagen, Stockholm.ymd(2020, 12, 25)),
            iter.next().unwrap()
        );
        assert_eq!(
            (AnnandagJul, Stockholm.ymd(2020, 12, 26)),
            iter.next().unwrap()
        );
        assert_eq!(
            (Nyarsafton, Stockholm.ymd(2020, 12, 31)),
            iter.next().unwrap()
        );
    }
}
