#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageCode {
    De, // German
    En, // English
    Es, // Spanish (Latin America)
    Fr, // French
    It, // Italian
    Ja, // Japanese
    Ko, // Korean
    Pl, // Polish
    Pt, // Portuguese (Brazil)
    Ru, // Russian
    Tc, // Chinese (Traditional)
    Th, // Thai
    Tr, // Turkish
    Uk, // Ukrainian
    Zh, // Chinese (Simplified)
}

impl LanguageCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::It => "it",
            Self::Ja => "ja",
            Self::Ko => "ko",
            Self::Pl => "pl",
            Self::Pt => "pt",
            Self::Ru => "ru",
            Self::Tc => "tc",
            Self::Th => "th",
            Self::Tr => "tr",
            Self::Uk => "uk",
            Self::Zh => "zh",
        }
    }

    pub fn index_url(&self) -> String {
        format!(
            "https://origin.warframe.com/PublicExport/index_{}.txt.lzma",
            self.as_str()
        )
    }
}

impl Default for LanguageCode {
    fn default() -> Self {
        Self::En
    }
}

impl std::fmt::Display for LanguageCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for LanguageCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "es" => Ok(Self::Es),
            "fr" => Ok(Self::Fr),
            "it" => Ok(Self::It),
            "ja" => Ok(Self::Ja),
            "ko" => Ok(Self::Ko),
            "pl" => Ok(Self::Pl),
            "pt" => Ok(Self::Pt),
            "ru" => Ok(Self::Ru),
            "tc" => Ok(Self::Tc),
            "th" => Ok(Self::Th),
            "tr" => Ok(Self::Tr),
            "uk" => Ok(Self::Uk),
            "zh" => Ok(Self::Zh),
            _ => Err(format!("Unknown language code: '{}'", s)),
        }
    }
}
