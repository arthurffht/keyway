// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Query grammar: an optional scope character, free text and an optional trailing
//! value (docs/PLAN.md §11.2).
//!
//! Unit expressions such as `5 in to cm` are left in the text for the calculator.

use std::time::Duration;

/// Decimal separator of the user's locale. Numbers never contain thousands separators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecimalSeparator {
    Point,
    Comma,
}

impl DecimalSeparator {
    fn as_char(self) -> char {
        match self {
            Self::Point => '.',
            Self::Comma => ',',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    Calculator,
    Settings,
    Files,
    Commands,
    Web,
    Emoji,
}

impl Scope {
    fn from_prefix(c: char) -> Option<Self> {
        Some(match c {
            '=' => Self::Calculator,
            '/' => Self::Settings,
            '@' => Self::Files,
            '>' => Self::Commands,
            '?' => Self::Web,
            ':' => Self::Emoji,
            _ => return None,
        })
    }

    /// Only these scopes address actions, which are the ones that take values.
    fn accepts_value(scope: Option<Self>) -> bool {
        matches!(scope, None | Some(Self::Settings | Self::Commands))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Percent(f64),
    Duration(Duration),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Query<'a> {
    pub scope: Option<Scope>,
    /// The body without its value, trimmed and otherwise unchanged.
    pub text: &'a str,
    pub value: Option<Value>,
}

pub fn parse(input: &str, decimal: DecimalSeparator) -> Query<'_> {
    let input = input.trim();
    let scope = input.chars().next().and_then(Scope::from_prefix);
    let body = match scope {
        Some(_) => input[1..].trim_start(),
        None => input,
    };
    if Scope::accepts_value(scope) {
        // Longest suffix first, so `10 min` is a duration and not the number 10.
        let starts = std::iter::once(0).chain(
            body.char_indices()
                .filter(|(_, c)| c.is_whitespace())
                .map(|(i, c)| i + c.len_utf8()),
        );
        for start in starts {
            if let Some(value) = parse_value(&body[start..], decimal) {
                return Query {
                    scope,
                    text: body[..start].trim_end(),
                    value: Some(value),
                };
            }
        }
    }
    Query {
        scope,
        text: body,
        value: None,
    }
}

fn parse_value(s: &str, decimal: DecimalSeparator) -> Option<Value> {
    let (n, rest) = number(s, decimal)?;
    let rest = rest.trim_start();
    if rest.is_empty() {
        Some(Value::Number(n))
    } else if rest == "%" {
        Some(Value::Percent(n))
    } else {
        duration(s, decimal).map(Value::Duration)
    }
}

/// Digits with at most one decimal separator, returning the rest of the input.
fn number(s: &str, decimal: DecimalSeparator) -> Option<(f64, &str)> {
    let digits = |s: &str| s.bytes().take_while(u8::is_ascii_digit).count();
    let whole = digits(s);
    if whole == 0 {
        return None;
    }
    let mut end = whole;
    if s[end..].starts_with(decimal.as_char()) {
        let fraction = digits(&s[end + 1..]);
        if fraction > 0 {
            end += 1 + fraction;
        }
    }
    let n = s[..end]
        .replace(decimal.as_char(), ".")
        .parse::<f64>()
        .ok()?;
    n.is_finite().then_some((n, &s[end..]))
}

/// Components in descending order: `1h30`, `1h 30m`, `10 min`, `2 h`. A bare number after
/// hours counts as minutes.
fn duration(s: &str, decimal: DecimalSeparator) -> Option<Duration> {
    let mut rest = s;
    let mut total = 0.0;
    let mut last_rank = None;
    loop {
        let (n, after) = number(rest, decimal)?;
        let after = after.trim_start();
        let word_len = after.bytes().take_while(u8::is_ascii_alphabetic).count();
        let (rank, seconds) = if word_len == 0 {
            if last_rank != Some(0) || !after.is_empty() {
                return None;
            }
            (1, 60.0)
        } else {
            unit(&after[..word_len])?
        };
        if last_rank.is_some_and(|last| rank <= last) {
            return None;
        }
        last_rank = Some(rank);
        total += n * seconds;
        rest = after[word_len..].trim_start();
        if rest.is_empty() {
            return Duration::try_from_secs_f64(total).ok();
        }
    }
}

fn unit(word: &str) -> Option<(u8, f64)> {
    Some(match word.to_ascii_lowercase().as_str() {
        "h" | "hr" | "hrs" | "hour" | "hours" => (0, 3600.0),
        "m" | "min" | "mins" | "minute" | "minutes" => (1, 60.0),
        "s" | "sec" | "secs" | "second" | "seconds" => (2, 1.0),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        DecimalSeparator::{Comma, Point},
        *,
    };
    use proptest::prelude::*;

    fn secs(s: u64) -> Value {
        Value::Duration(Duration::from_secs(s))
    }

    #[test]
    fn each_scope_character_selects_its_scope() {
        for (prefix, scope) in [
            ('=', Scope::Calculator),
            ('/', Scope::Settings),
            ('@', Scope::Files),
            ('>', Scope::Commands),
            ('?', Scope::Web),
            (':', Scope::Emoji),
        ] {
            let input = format!("{prefix} text");
            let q = parse(&input, Point);
            assert_eq!((q.scope, q.text), (Some(scope), "text"));
        }
    }

    #[test]
    fn text_without_a_scope_character_has_no_scope() {
        let q = parse("  night light ", Point);
        assert_eq!((q.scope, q.text, q.value), (None, "night light", None));
    }

    #[test]
    fn empty_input_is_an_empty_query() {
        let q = parse("", Point);
        assert_eq!((q.scope, q.text, q.value), (None, "", None));
        assert_eq!(parse("/", Point).scope, Some(Scope::Settings));
    }

    #[test]
    fn numbers_attach_to_the_text() {
        let q = parse("vol 30", Point);
        assert_eq!((q.text, q.value), ("vol", Some(Value::Number(30.0))));
        assert_eq!(parse("30", Point).text, "");
    }

    #[test]
    fn percentages_allow_a_space() {
        assert_eq!(parse("vol 30%", Point).value, Some(Value::Percent(30.0)));
        assert_eq!(parse("vol 30 %", Point).value, Some(Value::Percent(30.0)));
        assert_eq!(parse("vol 30 %", Point).text, "vol");
    }

    #[test]
    fn the_decimal_separator_follows_the_locale() {
        assert_eq!(parse("vol 1,5", Comma).value, Some(Value::Number(1.5)));
        assert_eq!(parse("vol 1.5", Point).value, Some(Value::Number(1.5)));
        assert_eq!(parse("vol 1.5", Comma).value, None);
        assert_eq!(parse("vol 1,5", Point).value, None);
    }

    #[test]
    fn durations_accept_each_documented_form() {
        assert_eq!(parse("timer 10 min", Point).value, Some(secs(600)));
        assert_eq!(parse("dnd 1h", Point).value, Some(secs(3600)));
        assert_eq!(parse("dnd 1h30", Point).value, Some(secs(5400)));
        assert_eq!(parse("dnd 1h 30m", Point).value, Some(secs(5400)));
        assert_eq!(parse("dnd 2 h", Point).value, Some(secs(7200)));
        assert_eq!(parse("timer 90 seconds", Point).value, Some(secs(90)));
        assert_eq!(parse("timer 10 min", Point).text, "timer");
        assert_eq!(parse("timer 1,5 h", Comma).value, Some(secs(5400)));
    }

    #[test]
    fn components_out_of_order_are_not_summed() {
        let q = parse("dnd 30m 1h", Point);
        assert_eq!((q.text, q.value), ("dnd 30m", Some(secs(3600))));
        let q = parse("dnd 1h 1h", Point);
        assert_eq!((q.text, q.value), ("dnd 1h", Some(secs(3600))));
        assert_eq!(parse("dnd 1 apple", Point).value, None);
    }

    #[test]
    fn only_action_scopes_take_values() {
        assert_eq!(parse("/vol 30", Point).value, Some(Value::Number(30.0)));
        assert_eq!(parse(">timer 10 min", Point).value, Some(secs(600)));
        for input in ["=10 min", "@report 30", "?rust 2024", ":smile 3"] {
            assert_eq!(parse(input, Point).value, None, "{input}");
        }
        assert_eq!(parse("=5 in to cm", Point).text, "5 in to cm");
    }

    #[test]
    fn values_outside_the_f64_range_are_text() {
        let huge = format!("vol {}", "9".repeat(400));
        assert_eq!(parse(&huge, Point).value, None);
        assert_eq!(
            parse(&format!("dnd {}h", "9".repeat(30)), Point).value,
            None
        );
    }

    proptest! {
        #[test]
        fn never_panics(input in any::<String>(), comma in any::<bool>()) {
            let q = parse(&input, if comma { Comma } else { Point });
            prop_assert!(input.contains(q.text));
        }
    }
}
