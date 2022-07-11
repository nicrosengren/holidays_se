use crate::Holiday;

impl serde::Serialize for Holiday {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use Holiday::*;

        match self {
            Nyarsdagen => ser.serialize_str("Nyårsdagen"),
            TrettondedagJul => ser.serialize_str("Trettondedag jul"),
            Langfredagen => ser.serialize_str("Långfredagen"),
            Paskdagen => ser.serialize_str("Påskdagen"),
            AnnandagPask => ser.serialize_str("Annandag påsk"),
            ForstaMaj => ser.serialize_str("Första maj"),
            KristiHimmelfardsdag => ser.serialize_str("Kristi himmelsfärdsdag"),
            Pingstdagen => ser.serialize_str("Pingstdagen"),
            Nationaldagen => ser.serialize_str("Nationaldagen"),
            Midsommarafton => ser.serialize_str("Midsommarafton"),
            Midsommardagen => ser.serialize_str("Midsommardagen"),
            AllaHelgonsDag => ser.serialize_str("Alla helgons dag"),
            Julafton => ser.serialize_str("Julafton"),
            Juldagen => ser.serialize_str("Juldagen"),
            AnnandagJul => ser.serialize_str("Annandag jul"),
            Nyarsafton => ser.serialize_str("Nyårsafton"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Holiday {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor)
    }
}

struct Visitor;

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Holiday;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a string in [Nyårsdagen, Trettondedag jul, Långfredagen, Påskdagen, Annandag påsk, Första maj, Kristi himmelsfärdsdag, Pingstdagen, Nationaldagen, Midsommarafton, Midsommardagen, Alla helgons dag, Julafton, Juldagen,  Annandag jul, Nyårsafton]")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        s.parse().map_err(E::custom)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn serialize() {
        use Holiday::*;
        let tests = vec![
            ("Nyårsdagen", Nyarsdagen),
            ("Trettondedag jul", TrettondedagJul),
            ("Långfredagen", Langfredagen),
            ("Påskdagen", Paskdagen),
            ("Annandag påsk", AnnandagPask),
            ("Första maj", ForstaMaj),
            ("Kristi himmelsfärdsdag", KristiHimmelfardsdag),
            ("Pingstdagen", Pingstdagen),
            ("Nationaldagen", Nationaldagen),
            ("Midsommarafton", Midsommarafton),
            ("Midsommardagen", Midsommardagen),
            ("Alla helgons dag", AllaHelgonsDag),
            ("Julafton", Julafton),
            ("Juldagen", Juldagen),
            ("Annandag jul", AnnandagJul),
            ("Nyårsafton", Nyarsafton),
        ];

        for (exp, v) in tests {
            assert_eq!(
                format!(r#""{exp}""#),
                serde_json::to_string(&v).expect("Serializing")
            );
        }
    }

    #[test]
    fn deserialize() {
        use Holiday::*;
        let tests = vec![
            (Nyarsdagen, "Nyårsdagen"),
            (TrettondedagJul, "Trettondedag jul"),
            (Langfredagen, "Långfredagen"),
            (Paskdagen, "Påskdagen"),
            (AnnandagPask, "Annandag påsk"),
            (ForstaMaj, "Första maj"),
            (KristiHimmelfardsdag, "Kristi himmelsfärdsdag"),
            (Pingstdagen, "Pingstdagen"),
            (Nationaldagen, "Nationaldagen"),
            (Midsommarafton, "Midsommarafton"),
            (Midsommardagen, "Midsommardagen"),
            (AllaHelgonsDag, "Alla helgons dag"),
            (Julafton, "Julafton"),
            (Juldagen, "Juldagen"),
            (AnnandagJul, "Annandag jul"),
            (Nyarsafton, "Nyårsafton"),
            (Nyarsdagen, "nyårsdagen"),
            (TrettondedagJul, "trettondedag jul"),
            (Langfredagen, "långfredagen"),
            (Paskdagen, "påskdagen"),
            (AnnandagPask, "annandag påsk"),
            (ForstaMaj, "första maj"),
            (KristiHimmelfardsdag, "kristi himmelsfärdsdag"),
            (Pingstdagen, "pingstdagen"),
            (Nationaldagen, "nationaldagen"),
            (Midsommarafton, "midsommarafton"),
            (Midsommardagen, "midsommardagen"),
            (AllaHelgonsDag, "alla helgons dag"),
            (Julafton, "julafton"),
            (Juldagen, "juldagen"),
            (AnnandagJul, "annandag jul"),
            (Nyarsafton, "nyårsafton"),
        ];

        for (exp, s) in tests {
            assert_eq!(
                exp,
                serde_json::from_str(&format!(r#""{s}""#)).expect("Deserializing")
            );
        }
    }
}
