// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Text normalization for haystacks and queries (docs/PLAN.md §11.1).

use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

/// Returns the search form of `text`: accents removed, lowercase, special
/// letters folded, and every run of whitespace or punctuation reduced to a
/// single space with none at the ends.
pub fn normalize(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pending_separator = false;
    for c in text.nfkd().filter(|&c| !is_combining_mark(c)) {
        for lower in c.to_lowercase() {
            // Lowercasing can reintroduce a mark (İ becomes i + U+0307).
            if is_combining_mark(lower) {
                continue;
            }
            if !lower.is_alphanumeric() {
                pending_separator = !out.is_empty();
                continue;
            }
            if pending_separator {
                out.push(' ');
                pending_separator = false;
            }
            match lower {
                'ß' => out.push_str("ss"),
                'æ' => out.push_str("ae"),
                'œ' => out.push_str("oe"),
                'ø' => out.push('o'),
                'ł' => out.push('l'),
                'đ' => out.push('d'),
                _ => out.push(lower),
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::normalize;
    use proptest::prelude::*;

    #[test]
    fn accents_and_case_are_equivalent() {
        let expected = normalize("configuracoes");
        assert_eq!(normalize("Configurações"), expected);
        assert_eq!(normalize("CONFIGURAÇÕES"), expected);
        assert_eq!(expected, "configuracoes");
    }

    #[test]
    fn letters_without_decomposition_are_folded() {
        assert_eq!(
            normalize("Straße æon Ørsted Łódź Đà"),
            "strasse aeon orsted lodz da"
        );
    }

    #[test]
    fn separators_collapse_and_trim() {
        assert_eq!(normalize("  Night--Light, \t on! "), "night light on");
        assert_eq!(normalize("---"), "");
    }

    #[test]
    fn compatibility_forms_are_flattened() {
        assert_eq!(normalize("ﬁle Ⅳ"), "file iv");
    }

    proptest! {
        #[test]
        fn is_idempotent(text in any::<String>()) {
            let once = normalize(&text);
            prop_assert_eq!(normalize(&once), once);
        }

        #[test]
        fn output_has_no_stray_separators(text in any::<String>()) {
            let out = normalize(&text);
            prop_assert!(!out.starts_with(' ') && !out.ends_with(' ') && !out.contains("  "));
        }
    }
}
