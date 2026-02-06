//! Configuration types for token estimation.

/// Average characters per token for general English/Latin text.
pub const DEFAULT_CHARS_PER_TOKEN: f64 = 6.0;

/// A language-specific rule that adjusts characters-per-token when matched.
///
/// The `matcher` function is called for each character in a word segment.
/// If it returns `true` for any character, the segment uses `chars_per_token`
/// instead of the default ratio.
#[derive(Clone)]
pub struct LanguageConfig {
    /// Character-level predicate that detects the language.
    pub matcher: fn(char) -> bool,
    /// Average characters per token for this language.
    pub chars_per_token: f64,
}

/// Options for [`estimate_token_count_with_options`](crate::estimate_token_count_with_options).
#[derive(Clone)]
pub struct EstimationOptions {
    /// Fallback characters-per-token ratio when no language matches.
    pub default_chars_per_token: f64,
    /// Language-specific overrides, checked in order.
    pub language_configs: Vec<LanguageConfig>,
}

impl Default for EstimationOptions {
    fn default() -> Self {
        Self {
            default_chars_per_token: DEFAULT_CHARS_PER_TOKEN,
            language_configs: default_language_configs(),
        }
    }
}

/// Options for [`split_by_tokens`](crate::split_by_tokens).
#[derive(Clone, Default)]
pub struct SplitOptions {
    /// Base estimation options.
    pub estimation: EstimationOptions,
    /// Number of overlapping tokens between consecutive chunks.
    pub overlap: usize,
}

/// Returns `true` if `c` is a German diacritic character.
pub fn is_german(c: char) -> bool {
    matches!(c, 'ä' | 'Ä' | 'ö' | 'Ö' | 'ü' | 'Ü' | 'ß' | 'ẞ')
}

/// Returns `true` if `c` is a French diacritic character.
pub fn is_french(c: char) -> bool {
    matches!(
        c,
        'é' | 'É'
            | 'è'
            | 'È'
            | 'ê'
            | 'Ê'
            | 'ë'
            | 'Ë'
            | 'à'
            | 'À'
            | 'â'
            | 'Â'
            | 'î'
            | 'Î'
            | 'ï'
            | 'Ï'
            | 'ô'
            | 'Ô'
            | 'û'
            | 'Û'
            | 'ù'
            | 'Ù'
            | 'ü'
            | 'Ü'
            | 'ÿ'
            | 'Ÿ'
            | 'ç'
            | 'Ç'
            | 'œ'
            | 'Œ'
            | 'æ'
            | 'Æ'
    )
}

/// Returns `true` if `c` is a Spanish diacritic character.
pub fn is_spanish(c: char) -> bool {
    matches!(
        c,
        'á' | 'Á' | 'é' | 'É' | 'í' | 'Í' | 'ó' | 'Ó' | 'ú' | 'Ú' | 'ü' | 'Ü' | 'ñ' | 'Ñ'
    )
}

/// Returns the default language configurations (German, French, Spanish).
pub fn default_language_configs() -> Vec<LanguageConfig> {
    vec![
        LanguageConfig {
            matcher: |c| is_german(c) || is_french(c),
            chars_per_token: 3.0,
        },
        LanguageConfig {
            matcher: is_spanish,
            chars_per_token: 3.5,
        },
    ]
}
