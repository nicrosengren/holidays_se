use chrono::TimeZone;
use chrono_tz::Europe::Stockholm;

#[test]
fn test_iterator_2020() {
    use holidays_se::Holiday::*;

    let mut iter = holidays_se::holidays_in_year(2020);

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
