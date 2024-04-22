use regex::Regex;

pub struct SplitExplorers {
    pattern: Regex,
}
impl Default for SplitExplorers {
     fn default() -> Self {
        let pattern = Regex::new(
            r"<Explorer>(?<explorers>.*)</Explorer><Surveyor>(?<surveyors>.*)</Surveyor>",
        )
        .unwrap();
        Self { pattern }
    }
}
impl SplitExplorers {
    #[allow(dead_code)]
    pub fn split_explorers(&self, raw: &str) -> Option<(Vec<String>, Vec<String>)> {
        if raw.is_empty() {
            return None;
        }

        if let Some(cap) = &self.pattern.captures(raw) {
            let explorers = &cap["explorers"];
            let surveyors = &cap["surveyors"];
            Some((
                explorers
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>(),
                surveyors
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>(),
            ))
        } else {
            None
        }
    }

    pub fn split_explorers_string(&self, raw: &str) -> Option<(String, String)> {
        self.pattern
            .captures(raw)
            .as_ref()
            .map(|cap| (cap["explorers"].to_string(), cap["surveyors"].to_string()))
    }
}

#[cfg(test)]
mod tests {

    use super::SplitExplorers;
    #[test]
    fn test() {
        let a = "<Explorer>Carla Tortelli, Diane Chambers, Sam Malone</Explorer><Surveyor>Lilith Sternin, Sam Malone, Norm Peterson</Surveyor>".to_string();
        let splitter = SplitExplorers::default();
        let (e, s) = splitter.split_explorers(&a).unwrap();
        assert_eq!(e, vec!["Carla Tortelli", "Diane Chambers", "Sam Malone"]);
        assert_eq!(s, vec!["Lilith Sternin", "Sam Malone", "Norm Peterson"]);
    }
}