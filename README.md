# Holidays Sweden

A simple crate to keep track of holidays each year in sweden.


Getting the date of a Holiday in a given year:
```
use holidays_se::Holiday;
assert_eq!(Holiday::Paskdagen.in_year(2020), Stockholm.ymd(2020, 4, 12));
```



A more useful case might be to partition a given time range into slices depending on the kind of day:
```

 use chrono::TimeZone;
 use chrono_tz::Europe::Stockholm;

 let start = Stockholm.ymd(2020, 9, 18).and_hms(0, 0, 0); // Friday
 let end = Stockholm.ymd(2020, 9, 21).and_hms(13, 15, 0); // Monday at 13:15

 let mut iter = holidays_se::slice_on_day_kind(start..end);

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

```
