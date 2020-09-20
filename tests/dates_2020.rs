use chrono::TimeZone;
use chrono_tz::Europe::Stockholm;

#[test]
fn test_holidays_2020() {
    use holidays_se::Holiday::*;

    assert_eq!(Nyarsdagen.in_year(2020), Stockholm.ymd(2020, 1, 1));
    assert_eq!(TrettondedagJul.in_year(2020), Stockholm.ymd(2020, 1, 6));
    assert_eq!(Langfredagen.in_year(2020), Stockholm.ymd(2020, 4, 10));
    assert_eq!(Paskdagen.in_year(2020), Stockholm.ymd(2020, 4, 12));
    assert_eq!(AnnandagPask.in_year(2020), Stockholm.ymd(2020, 4, 13));
    assert_eq!(ForstaMaj.in_year(2020), Stockholm.ymd(2020, 5, 1));
    assert_eq!(
        KristiHimmelfardsdag.in_year(2020),
        Stockholm.ymd(2020, 5, 21)
    );
    assert_eq!(Pingstdagen.in_year(2020), Stockholm.ymd(2020, 5, 31));
    assert_eq!(Nationaldagen.in_year(2020), Stockholm.ymd(2020, 6, 6));
    assert_eq!(Midsommarafton.in_year(2020), Stockholm.ymd(2020, 6, 19));
    assert_eq!(Midsommardagen.in_year(2020), Stockholm.ymd(2020, 6, 20));
    assert_eq!(AllaHelgonsDag.in_year(2020), Stockholm.ymd(2020, 10, 31));
    assert_eq!(Julafton.in_year(2020), Stockholm.ymd(2020, 12, 24));
    assert_eq!(Juldagen.in_year(2020), Stockholm.ymd(2020, 12, 25));
    assert_eq!(AnnandagJul.in_year(2020), Stockholm.ymd(2020, 12, 26));
    assert_eq!(Nyarsafton.in_year(2020), Stockholm.ymd(2020, 12, 31));
}
