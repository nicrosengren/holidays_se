# Holidays Sweden

A simple crate to keep track of holidays each year in sweden.


Getting the date of a Holiday in a given year:
```
use holidays_se::Holiday;
assert_eq!(Holiday::Paskdagen.in_year(2020), Stockholm.ymd(2020, 4, 12));
```
