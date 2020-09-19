use chrono::{TimeZone};
use chrono_tz::Europe::Stockholm;
use holidays_se::easter_day_for_year;

#[test]
fn the_great_easter_test() {
    assert_eq!(
        Stockholm.ymd(1900, 4, 15),
        easter_day_for_year(1900)
    );
    assert_eq!(
        Stockholm.ymd(1901, 4, 7),
        easter_day_for_year(1901)
    );
    assert_eq!(
        Stockholm.ymd(1902, 3, 30),
        easter_day_for_year(1902)
    );
    assert_eq!(
        Stockholm.ymd(1903, 4, 12),
        easter_day_for_year(1903)
    );
    assert_eq!(
        Stockholm.ymd(1904, 4, 3),
        easter_day_for_year(1904)
    );
    assert_eq!(
        Stockholm.ymd(1905, 4, 23),
        easter_day_for_year(1905)
    );
    assert_eq!(
        Stockholm.ymd(1906, 4, 15),
        easter_day_for_year(1906)
    );
    assert_eq!(
        Stockholm.ymd(1907, 3, 31),
        easter_day_for_year(1907)
    );
    assert_eq!(
        Stockholm.ymd(1908, 4, 19),
        easter_day_for_year(1908)
    );
    assert_eq!(
        Stockholm.ymd(1909, 4, 11),
        easter_day_for_year(1909)
    );
    assert_eq!(
        Stockholm.ymd(1910, 3, 27),
        easter_day_for_year(1910)
    );
    assert_eq!(
        Stockholm.ymd(1911, 4, 16),
        easter_day_for_year(1911)
    );
    assert_eq!(
        Stockholm.ymd(1912, 4, 7),
        easter_day_for_year(1912)
    );
    assert_eq!(
        Stockholm.ymd(1913, 3, 23),
        easter_day_for_year(1913)
    );
    assert_eq!(
        Stockholm.ymd(1914, 4, 12),
        easter_day_for_year(1914)
    );
    assert_eq!(
        Stockholm.ymd(1915, 4, 4),
        easter_day_for_year(1915)
    );
    assert_eq!(
        Stockholm.ymd(1916, 4, 23),
        easter_day_for_year(1916)
    );
    assert_eq!(
        Stockholm.ymd(1917, 4, 8),
        easter_day_for_year(1917)
    );
    assert_eq!(
        Stockholm.ymd(1918, 3, 31),
        easter_day_for_year(1918)
    );
    assert_eq!(
        Stockholm.ymd(1919, 4, 20),
        easter_day_for_year(1919)
    );
    assert_eq!(
        Stockholm.ymd(1920, 4, 4),
        easter_day_for_year(1920)
    );
    assert_eq!(
        Stockholm.ymd(1921, 3, 27),
        easter_day_for_year(1921)
    );
    assert_eq!(
        Stockholm.ymd(1922, 4, 16),
        easter_day_for_year(1922)
    );
    assert_eq!(
        Stockholm.ymd(1923, 4, 1),
        easter_day_for_year(1923)
    );
    assert_eq!(
        Stockholm.ymd(1924, 4, 20),
        easter_day_for_year(1924)
    );
    assert_eq!(
        Stockholm.ymd(1925, 4, 12),
        easter_day_for_year(1925)
    );
    assert_eq!(
        Stockholm.ymd(1926, 4, 4),
        easter_day_for_year(1926)
    );
    assert_eq!(
        Stockholm.ymd(1927, 4, 17),
        easter_day_for_year(1927)
    );
    assert_eq!(
        Stockholm.ymd(1928, 4, 8),
        easter_day_for_year(1928)
    );
    assert_eq!(
        Stockholm.ymd(1929, 3, 31),
        easter_day_for_year(1929)
    );
    assert_eq!(
        Stockholm.ymd(1930, 4, 20),
        easter_day_for_year(1930)
    );
    assert_eq!(
        Stockholm.ymd(1931, 4, 5),
        easter_day_for_year(1931)
    );
    assert_eq!(
        Stockholm.ymd(1932, 3, 27),
        easter_day_for_year(1932)
    );
    assert_eq!(
        Stockholm.ymd(1933, 4, 16),
        easter_day_for_year(1933)
    );
    assert_eq!(
        Stockholm.ymd(1934, 4, 1),
        easter_day_for_year(1934)
    );
    assert_eq!(
        Stockholm.ymd(1935, 4, 21),
        easter_day_for_year(1935)
    );
    assert_eq!(
        Stockholm.ymd(1936, 4, 12),
        easter_day_for_year(1936)
    );
    assert_eq!(
        Stockholm.ymd(1937, 3, 28),
        easter_day_for_year(1937)
    );
    assert_eq!(
        Stockholm.ymd(1938, 4, 17),
        easter_day_for_year(1938)
    );
    assert_eq!(
        Stockholm.ymd(1939, 4, 9),
        easter_day_for_year(1939)
    );
    assert_eq!(
        Stockholm.ymd(1940, 3, 24),
        easter_day_for_year(1940)
    );
    assert_eq!(
        Stockholm.ymd(1941, 4, 13),
        easter_day_for_year(1941)
    );
    assert_eq!(
        Stockholm.ymd(1942, 4, 5),
        easter_day_for_year(1942)
    );
    assert_eq!(
        Stockholm.ymd(1943, 4, 25),
        easter_day_for_year(1943)
    );
    assert_eq!(
        Stockholm.ymd(1944, 4, 9),
        easter_day_for_year(1944)
    );
    assert_eq!(
        Stockholm.ymd(1945, 4, 1),
        easter_day_for_year(1945)
    );
    assert_eq!(
        Stockholm.ymd(1946, 4, 21),
        easter_day_for_year(1946)
    );
    assert_eq!(
        Stockholm.ymd(1947, 4, 6),
        easter_day_for_year(1947)
    );
    assert_eq!(
        Stockholm.ymd(1948, 3, 28),
        easter_day_for_year(1948)
    );
    assert_eq!(
        Stockholm.ymd(1949, 4, 17),
        easter_day_for_year(1949)
    );
    assert_eq!(
        Stockholm.ymd(1950, 4, 9),
        easter_day_for_year(1950)
    );
    assert_eq!(
        Stockholm.ymd(1951, 3, 25),
        easter_day_for_year(1951)
    );
    assert_eq!(
        Stockholm.ymd(1952, 4, 13),
        easter_day_for_year(1952)
    );
    assert_eq!(
        Stockholm.ymd(1953, 4, 5),
        easter_day_for_year(1953)
    );
    assert_eq!(
        Stockholm.ymd(1954, 4, 18),
        easter_day_for_year(1954)
    );
    assert_eq!(
        Stockholm.ymd(1955, 4, 10),
        easter_day_for_year(1955)
    );
    assert_eq!(
        Stockholm.ymd(1956, 4, 1),
        easter_day_for_year(1956)
    );
    assert_eq!(
        Stockholm.ymd(1957, 4, 21),
        easter_day_for_year(1957)
    );
    assert_eq!(
        Stockholm.ymd(1958, 4, 6),
        easter_day_for_year(1958)
    );
    assert_eq!(
        Stockholm.ymd(1959, 3, 29),
        easter_day_for_year(1959)
    );
    assert_eq!(
        Stockholm.ymd(1960, 4, 17),
        easter_day_for_year(1960)
    );
    assert_eq!(
        Stockholm.ymd(1961, 4, 2),
        easter_day_for_year(1961)
    );
    assert_eq!(
        Stockholm.ymd(1962, 4, 22),
        easter_day_for_year(1962)
    );
    assert_eq!(
        Stockholm.ymd(1963, 4, 14),
        easter_day_for_year(1963)
    );
    assert_eq!(
        Stockholm.ymd(1964, 3, 29),
        easter_day_for_year(1964)
    );
    assert_eq!(
        Stockholm.ymd(1965, 4, 18),
        easter_day_for_year(1965)
    );
    assert_eq!(
        Stockholm.ymd(1966, 4, 10),
        easter_day_for_year(1966)
    );
    assert_eq!(
        Stockholm.ymd(1967, 3, 26),
        easter_day_for_year(1967)
    );
    assert_eq!(
        Stockholm.ymd(1968, 4, 14),
        easter_day_for_year(1968)
    );
    assert_eq!(
        Stockholm.ymd(1969, 4, 6),
        easter_day_for_year(1969)
    );
    assert_eq!(
        Stockholm.ymd(1970, 3, 29),
        easter_day_for_year(1970)
    );
    assert_eq!(
        Stockholm.ymd(1971, 4, 11),
        easter_day_for_year(1971)
    );
    assert_eq!(
        Stockholm.ymd(1972, 4, 2),
        easter_day_for_year(1972)
    );
    assert_eq!(
        Stockholm.ymd(1973, 4, 22),
        easter_day_for_year(1973)
    );
    assert_eq!(
        Stockholm.ymd(1974, 4, 14),
        easter_day_for_year(1974)
    );
    assert_eq!(
        Stockholm.ymd(1975, 3, 30),
        easter_day_for_year(1975)
    );
    assert_eq!(
        Stockholm.ymd(1976, 4, 18),
        easter_day_for_year(1976)
    );
    assert_eq!(
        Stockholm.ymd(1977, 4, 10),
        easter_day_for_year(1977)
    );
    assert_eq!(
        Stockholm.ymd(1978, 3, 26),
        easter_day_for_year(1978)
    );
    assert_eq!(
        Stockholm.ymd(1979, 4, 15),
        easter_day_for_year(1979)
    );
    assert_eq!(
        Stockholm.ymd(1980, 4, 6),
        easter_day_for_year(1980)
    );
    assert_eq!(
        Stockholm.ymd(1981, 4, 19),
        easter_day_for_year(1981)
    );
    assert_eq!(
        Stockholm.ymd(1982, 4, 11),
        easter_day_for_year(1982)
    );
    assert_eq!(
        Stockholm.ymd(1983, 4, 3),
        easter_day_for_year(1983)
    );
    assert_eq!(
        Stockholm.ymd(1984, 4, 22),
        easter_day_for_year(1984)
    );
    assert_eq!(
        Stockholm.ymd(1985, 4, 7),
        easter_day_for_year(1985)
    );
    assert_eq!(
        Stockholm.ymd(1986, 3, 30),
        easter_day_for_year(1986)
    );
    assert_eq!(
        Stockholm.ymd(1987, 4, 19),
        easter_day_for_year(1987)
    );
    assert_eq!(
        Stockholm.ymd(1988, 4, 3),
        easter_day_for_year(1988)
    );
    assert_eq!(
        Stockholm.ymd(1989, 3, 26),
        easter_day_for_year(1989)
    );
    assert_eq!(
        Stockholm.ymd(1990, 4, 15),
        easter_day_for_year(1990)
    );
    assert_eq!(
        Stockholm.ymd(1991, 3, 31),
        easter_day_for_year(1991)
    );
    assert_eq!(
        Stockholm.ymd(1992, 4, 19),
        easter_day_for_year(1992)
    );
    assert_eq!(
        Stockholm.ymd(1993, 4, 11),
        easter_day_for_year(1993)
    );
    assert_eq!(
        Stockholm.ymd(1994, 4, 3),
        easter_day_for_year(1994)
    );
    assert_eq!(
        Stockholm.ymd(1995, 4, 16),
        easter_day_for_year(1995)
    );
    assert_eq!(
        Stockholm.ymd(1996, 4, 7),
        easter_day_for_year(1996)
    );
    assert_eq!(
        Stockholm.ymd(1997, 3, 30),
        easter_day_for_year(1997)
    );
    assert_eq!(
        Stockholm.ymd(1998, 4, 12),
        easter_day_for_year(1998)
    );
    assert_eq!(
        Stockholm.ymd(1999, 4, 4),
        easter_day_for_year(1999)
    );

    assert_eq!(
        Stockholm.ymd(2000, 4, 23),
        easter_day_for_year(2000)
    );
    assert_eq!(
        Stockholm.ymd(2001, 4, 15),
        easter_day_for_year(2001)
    );
    assert_eq!(
        Stockholm.ymd(2002, 3, 31),
        easter_day_for_year(2002)
    );
    assert_eq!(
        Stockholm.ymd(2003, 4, 20),
        easter_day_for_year(2003)
    );
    assert_eq!(
        Stockholm.ymd(2004, 4, 11),
        easter_day_for_year(2004)
    );
    assert_eq!(
        Stockholm.ymd(2005, 3, 27),
        easter_day_for_year(2005)
    );
    assert_eq!(
        Stockholm.ymd(2006, 4, 16),
        easter_day_for_year(2006)
    );
    assert_eq!(
        Stockholm.ymd(2007, 4, 8),
        easter_day_for_year(2007)
    );
    assert_eq!(
        Stockholm.ymd(2008, 3, 23),
        easter_day_for_year(2008)
    );
    assert_eq!(
        Stockholm.ymd(2009, 4, 12),
        easter_day_for_year(2009)
    );
    assert_eq!(
        Stockholm.ymd(2010, 4, 4),
        easter_day_for_year(2010)
    );
    assert_eq!(
        Stockholm.ymd(2011, 4, 24),
        easter_day_for_year(2011)
    );
    assert_eq!(
        Stockholm.ymd(2012, 4, 8),
        easter_day_for_year(2012)
    );
    assert_eq!(
        Stockholm.ymd(2013, 3, 31),
        easter_day_for_year(2013)
    );
    assert_eq!(
        Stockholm.ymd(2014, 4, 20),
        easter_day_for_year(2014)
    );
    assert_eq!(
        Stockholm.ymd(2015, 4, 5),
        easter_day_for_year(2015)
    );
    assert_eq!(
        Stockholm.ymd(2016, 3, 27),
        easter_day_for_year(2016)
    );
    assert_eq!(
        Stockholm.ymd(2017, 4, 16),
        easter_day_for_year(2017)
    );
    assert_eq!(
        Stockholm.ymd(2018, 4, 1),
        easter_day_for_year(2018)
    );
    assert_eq!(
        Stockholm.ymd(2019, 4, 21),
        easter_day_for_year(2019)
    );
    assert_eq!(
        Stockholm.ymd(2020, 4, 12),
        easter_day_for_year(2020)
    );
    assert_eq!(
        Stockholm.ymd(2021, 4, 4),
        easter_day_for_year(2021)
    );
    assert_eq!(
        Stockholm.ymd(2022, 4, 17),
        easter_day_for_year(2022)
    );
    assert_eq!(
        Stockholm.ymd(2023, 4, 9),
        easter_day_for_year(2023)
    );
    assert_eq!(
        Stockholm.ymd(2024, 3, 31),
        easter_day_for_year(2024)
    );

    assert_eq!(
        Stockholm.ymd(2000, 4, 23),
        easter_day_for_year(2000)
    );
    assert_eq!(
        Stockholm.ymd(2001, 4, 15),
        easter_day_for_year(2001)
    );
    assert_eq!(
        Stockholm.ymd(2002, 3, 31),
        easter_day_for_year(2002)
    );
    assert_eq!(
        Stockholm.ymd(2003, 4, 20),
        easter_day_for_year(2003)
    );
    assert_eq!(
        Stockholm.ymd(2004, 4, 11),
        easter_day_for_year(2004)
    );
    assert_eq!(
        Stockholm.ymd(2005, 3, 27),
        easter_day_for_year(2005)
    );
    assert_eq!(
        Stockholm.ymd(2006, 4, 16),
        easter_day_for_year(2006)
    );
    assert_eq!(
        Stockholm.ymd(2007, 4, 8),
        easter_day_for_year(2007)
    );
    assert_eq!(
        Stockholm.ymd(2008, 3, 23),
        easter_day_for_year(2008)
    );
    assert_eq!(
        Stockholm.ymd(2009, 4, 12),
        easter_day_for_year(2009)
    );
    assert_eq!(
        Stockholm.ymd(2010, 4, 4),
        easter_day_for_year(2010)
    );
    assert_eq!(
        Stockholm.ymd(2011, 4, 24),
        easter_day_for_year(2011)
    );
    assert_eq!(
        Stockholm.ymd(2012, 4, 8),
        easter_day_for_year(2012)
    );
    assert_eq!(
        Stockholm.ymd(2013, 3, 31),
        easter_day_for_year(2013)
    );
    assert_eq!(
        Stockholm.ymd(2014, 4, 20),
        easter_day_for_year(2014)
    );
    assert_eq!(
        Stockholm.ymd(2015, 4, 5),
        easter_day_for_year(2015)
    );
    assert_eq!(
        Stockholm.ymd(2016, 3, 27),
        easter_day_for_year(2016)
    );
    assert_eq!(
        Stockholm.ymd(2017, 4, 16),
        easter_day_for_year(2017)
    );
    assert_eq!(
        Stockholm.ymd(2018, 4, 1),
        easter_day_for_year(2018)
    );
    assert_eq!(
        Stockholm.ymd(2019, 4, 21),
        easter_day_for_year(2019)
    );
    assert_eq!(
        Stockholm.ymd(2020, 4, 12),
        easter_day_for_year(2020)
    );
    assert_eq!(
        Stockholm.ymd(2021, 4, 4),
        easter_day_for_year(2021)
    );
    assert_eq!(
        Stockholm.ymd(2022, 4, 17),
        easter_day_for_year(2022)
    );
    assert_eq!(
        Stockholm.ymd(2023, 4, 9),
        easter_day_for_year(2023)
    );
    assert_eq!(
        Stockholm.ymd(2024, 3, 31),
        easter_day_for_year(2024)
    );
    assert_eq!(
        Stockholm.ymd(2025, 4, 20),
        easter_day_for_year(2025)
    );
    assert_eq!(
        Stockholm.ymd(2026, 4, 5),
        easter_day_for_year(2026)
    );
    assert_eq!(
        Stockholm.ymd(2027, 3, 28),
        easter_day_for_year(2027)
    );
    assert_eq!(
        Stockholm.ymd(2028, 4, 16),
        easter_day_for_year(2028)
    );
    assert_eq!(
        Stockholm.ymd(2029, 4, 1),
        easter_day_for_year(2029)
    );
    assert_eq!(
        Stockholm.ymd(2030, 4, 21),
        easter_day_for_year(2030)
    );
    assert_eq!(
        Stockholm.ymd(2031, 4, 13),
        easter_day_for_year(2031)
    );
    assert_eq!(
        Stockholm.ymd(2032, 3, 28),
        easter_day_for_year(2032)
    );
    assert_eq!(
        Stockholm.ymd(2033, 4, 17),
        easter_day_for_year(2033)
    );
    assert_eq!(
        Stockholm.ymd(2034, 4, 9),
        easter_day_for_year(2034)
    );
    assert_eq!(
        Stockholm.ymd(2035, 3, 25),
        easter_day_for_year(2035)
    );
    assert_eq!(
        Stockholm.ymd(2036, 4, 13),
        easter_day_for_year(2036)
    );
    assert_eq!(
        Stockholm.ymd(2037, 4, 5),
        easter_day_for_year(2037)
    );
    assert_eq!(
        Stockholm.ymd(2038, 4, 25),
        easter_day_for_year(2038)
    );
    assert_eq!(
        Stockholm.ymd(2039, 4, 10),
        easter_day_for_year(2039)
    );
    assert_eq!(
        Stockholm.ymd(2040, 4, 1),
        easter_day_for_year(2040)
    );
    assert_eq!(
        Stockholm.ymd(2041, 4, 21),
        easter_day_for_year(2041)
    );
    assert_eq!(
        Stockholm.ymd(2042, 4, 6),
        easter_day_for_year(2042)
    );
    assert_eq!(
        Stockholm.ymd(2043, 3, 29),
        easter_day_for_year(2043)
    );
    assert_eq!(
        Stockholm.ymd(2044, 4, 17),
        easter_day_for_year(2044)
    );
    assert_eq!(
        Stockholm.ymd(2045, 4, 9),
        easter_day_for_year(2045)
    );
    assert_eq!(
        Stockholm.ymd(2046, 3, 25),
        easter_day_for_year(2046)
    );
    assert_eq!(
        Stockholm.ymd(2047, 4, 14),
        easter_day_for_year(2047)
    );
    assert_eq!(
        Stockholm.ymd(2048, 4, 5),
        easter_day_for_year(2048)
    );
    assert_eq!(
        Stockholm.ymd(2049, 4, 18),
        easter_day_for_year(2049)
    );
    assert_eq!(
        Stockholm.ymd(2050, 4, 10),
        easter_day_for_year(2050)
    );
    assert_eq!(
        Stockholm.ymd(2051, 4, 2),
        easter_day_for_year(2051)
    );
    assert_eq!(
        Stockholm.ymd(2052, 4, 21),
        easter_day_for_year(2052)
    );
    assert_eq!(
        Stockholm.ymd(2053, 4, 6),
        easter_day_for_year(2053)
    );
    assert_eq!(
        Stockholm.ymd(2054, 3, 29),
        easter_day_for_year(2054)
    );
    assert_eq!(
        Stockholm.ymd(2055, 4, 18),
        easter_day_for_year(2055)
    );
    assert_eq!(
        Stockholm.ymd(2056, 4, 2),
        easter_day_for_year(2056)
    );
    assert_eq!(
        Stockholm.ymd(2057, 4, 22),
        easter_day_for_year(2057)
    );
    assert_eq!(
        Stockholm.ymd(2058, 4, 14),
        easter_day_for_year(2058)
    );
    assert_eq!(
        Stockholm.ymd(2059, 3, 30),
        easter_day_for_year(2059)
    );
    assert_eq!(
        Stockholm.ymd(2060, 4, 18),
        easter_day_for_year(2060)
    );
    assert_eq!(
        Stockholm.ymd(2061, 4, 10),
        easter_day_for_year(2061)
    );
    assert_eq!(
        Stockholm.ymd(2062, 3, 26),
        easter_day_for_year(2062)
    );
    assert_eq!(
        Stockholm.ymd(2063, 4, 15),
        easter_day_for_year(2063)
    );
    assert_eq!(
        Stockholm.ymd(2064, 4, 6),
        easter_day_for_year(2064)
    );
    assert_eq!(
        Stockholm.ymd(2065, 3, 29),
        easter_day_for_year(2065)
    );
    assert_eq!(
        Stockholm.ymd(2066, 4, 11),
        easter_day_for_year(2066)
    );
    assert_eq!(
        Stockholm.ymd(2067, 4, 3),
        easter_day_for_year(2067)
    );
    assert_eq!(
        Stockholm.ymd(2068, 4, 22),
        easter_day_for_year(2068)
    );
    assert_eq!(
        Stockholm.ymd(2069, 4, 14),
        easter_day_for_year(2069)
    );
    assert_eq!(
        Stockholm.ymd(2070, 3, 30),
        easter_day_for_year(2070)
    );
    assert_eq!(
        Stockholm.ymd(2071, 4, 19),
        easter_day_for_year(2071)
    );
    assert_eq!(
        Stockholm.ymd(2072, 4, 10),
        easter_day_for_year(2072)
    );
    assert_eq!(
        Stockholm.ymd(2073, 3, 26),
        easter_day_for_year(2073)
    );
    assert_eq!(
        Stockholm.ymd(2074, 4, 15),
        easter_day_for_year(2074)
    );
    assert_eq!(
        Stockholm.ymd(2075, 4, 7),
        easter_day_for_year(2075)
    );
    assert_eq!(
        Stockholm.ymd(2076, 4, 19),
        easter_day_for_year(2076)
    );
    assert_eq!(
        Stockholm.ymd(2077, 4, 11),
        easter_day_for_year(2077)
    );
    assert_eq!(
        Stockholm.ymd(2078, 4, 3),
        easter_day_for_year(2078)
    );
    assert_eq!(
        Stockholm.ymd(2079, 4, 23),
        easter_day_for_year(2079)
    );
    assert_eq!(
        Stockholm.ymd(2080, 4, 7),
        easter_day_for_year(2080)
    );
    assert_eq!(
        Stockholm.ymd(2081, 3, 30),
        easter_day_for_year(2081)
    );
    assert_eq!(
        Stockholm.ymd(2082, 4, 19),
        easter_day_for_year(2082)
    );
    assert_eq!(
        Stockholm.ymd(2083, 4, 4),
        easter_day_for_year(2083)
    );
    assert_eq!(
        Stockholm.ymd(2084, 3, 26),
        easter_day_for_year(2084)
    );
    assert_eq!(
        Stockholm.ymd(2085, 4, 15),
        easter_day_for_year(2085)
    );
    assert_eq!(
        Stockholm.ymd(2086, 3, 31),
        easter_day_for_year(2086)
    );
    assert_eq!(
        Stockholm.ymd(2087, 4, 20),
        easter_day_for_year(2087)
    );
    assert_eq!(
        Stockholm.ymd(2088, 4, 11),
        easter_day_for_year(2088)
    );
    assert_eq!(
        Stockholm.ymd(2089, 4, 3),
        easter_day_for_year(2089)
    );
    assert_eq!(
        Stockholm.ymd(2090, 4, 16),
        easter_day_for_year(2090)
    );
    assert_eq!(
        Stockholm.ymd(2091, 4, 8),
        easter_day_for_year(2091)
    );
    assert_eq!(
        Stockholm.ymd(2092, 3, 30),
        easter_day_for_year(2092)
    );
    assert_eq!(
        Stockholm.ymd(2093, 4, 12),
        easter_day_for_year(2093)
    );
    assert_eq!(
        Stockholm.ymd(2094, 4, 4),
        easter_day_for_year(2094)
    );
    assert_eq!(
        Stockholm.ymd(2095, 4, 24),
        easter_day_for_year(2095)
    );
    assert_eq!(
        Stockholm.ymd(2096, 4, 15),
        easter_day_for_year(2096)
    );
    assert_eq!(
        Stockholm.ymd(2097, 3, 31),
        easter_day_for_year(2097)
    );
    assert_eq!(
        Stockholm.ymd(2098, 4, 20),
        easter_day_for_year(2098)
    );
    assert_eq!(
        Stockholm.ymd(2099, 4, 12),
        easter_day_for_year(2099)
    );

}
