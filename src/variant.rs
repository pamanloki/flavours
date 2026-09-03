use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Dark,
    Light,
}

impl Variant {
    pub fn opposite(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Infer whether a scheme slug names a light or dark variant.
///
/// Matches the conventions used by upstream base16 schemes: slugs ending with
/// `-light`, `-dawn`, or `-day`, or containing `-light-` as an infix, are
/// treated as light; everything else defaults to dark. A slug ending with
/// `-dark` or `-night`, or containing `-dark-` as an infix, is also
/// explicitly dark.
pub fn variant(slug: &str) -> Variant {
    if slug.ends_with("-light")
        || slug.ends_with("-dawn")
        || slug.ends_with("-day")
        || slug.contains("-light-")
    {
        Variant::Light
    } else {
        Variant::Dark
    }
}

/// Derive the family name of a scheme by stripping the light/dark suffix (or
/// infix). Slugs without a recognised suffix are returned as-is.
///
/// Examples:
///   `rose-pine-dawn` -> `rose-pine`
///   `gruvbox-dark-hard` -> `gruvbox-hard`
///   `gruvbox-material-dark-medium` -> `gruvbox-material-medium`
///   `nord` -> `nord`
pub fn family(slug: &str) -> String {
    let mut s = slug.to_string();
    // Collapse infix variant markers (e.g. `-dark-`, `-light-`) into a single
    // hyphen, so `gruvbox-dark-hard` becomes `gruvbox-hard`.
    for infix in ["-dark-", "-light-"] {
        while let Some(pos) = s.find(infix) {
            s.replace_range(pos..pos + infix.len(), "-");
        }
    }
    // Strip a trailing suffix once. Order matters only for longest-match
    // safety, but every suffix here is unambiguous.
    for suffix in ["-dark", "-light", "-dawn", "-night", "-day"] {
        if let Some(stripped) = s.strip_suffix(suffix) {
            return stripped.to_string();
        }
    }
    s
}

/// Find the partner scheme for `slug` from a list of all installed slugs.
///
/// Returns the first slug that shares the same family and has the opposite
/// variant. Returns `None` if no such partner exists.
pub fn partner<'a, I>(slug: &str, all: I) -> Option<String>
where
    I: IntoIterator<Item = &'a String>,
{
    let target_family = family(slug);
    let target_variant = variant(slug).opposite();
    all.into_iter()
        .find(|candidate| {
            candidate.as_str() != slug
                && family(candidate) == target_family
                && variant(candidate) == target_variant
        })
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_defaults_to_dark() {
        assert_eq!(variant("nord"), Variant::Dark);
        assert_eq!(variant("gruvbox-dark"), Variant::Dark);
        assert_eq!(variant("gruvbox-dark-hard"), Variant::Dark);
    }

    #[test]
    fn variant_detects_light_suffixes() {
        assert_eq!(variant("solarized-light"), Variant::Light);
        assert_eq!(variant("rose-pine-dawn"), Variant::Light);
        assert_eq!(variant("catppuccin-light-mocha"), Variant::Light);
    }

    #[test]
    fn family_strips_suffix() {
        assert_eq!(family("rose-pine-dawn"), "rose-pine");
        assert_eq!(family("rose-pine"), "rose-pine");
        assert_eq!(family("gruvbox-dark"), "gruvbox");
        assert_eq!(family("gruvbox-light"), "gruvbox");
    }

    #[test]
    fn family_strips_infix() {
        assert_eq!(family("gruvbox-dark-hard"), "gruvbox-hard");
        assert_eq!(family("gruvbox-light-hard"), "gruvbox-hard");
        assert_eq!(family("gruvbox-material-dark-medium"), "gruvbox-material-medium");
    }

    #[test]
    fn family_leaves_neutral_slugs_alone() {
        assert_eq!(family("nord"), "nord");
        assert_eq!(family("dracula"), "dracula");
    }

    #[test]
    fn partner_finds_opposite_variant() {
        let all = vec![
            "gruvbox-dark".to_string(),
            "gruvbox-light".to_string(),
            "nord".to_string(),
        ];
        assert_eq!(
            partner("gruvbox-dark", &all),
            Some("gruvbox-light".to_string())
        );
        assert_eq!(
            partner("gruvbox-light", &all),
            Some("gruvbox-dark".to_string())
        );
        assert_eq!(partner("nord", &all), None);
    }

    #[test]
    fn partner_pairs_rose_pine_dawn() {
        let all = vec![
            "rose-pine".to_string(),
            "rose-pine-dawn".to_string(),
            "rose-pine-moon".to_string(),
        ];
        assert_eq!(
            partner("rose-pine-dawn", &all),
            Some("rose-pine".to_string())
        );
        assert_eq!(
            partner("rose-pine", &all),
            Some("rose-pine-dawn".to_string())
        );
    }
}
