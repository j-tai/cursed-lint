use once_cell::sync::Lazy;

use crate::lexer::{lex, Token};

const RULES_TXT: &str = include_str!("rules.txt");

struct Rule {
    pub name: &'static str,
    pub tokens: Vec<Vec<Token<'static>>>,
}

static RULES: Lazy<Vec<Rule>> = Lazy::new(|| {
    let mut rules = vec![];
    let mut current_rule: Option<Rule> = None;
    for line in RULES_TXT.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if line.starts_with(' ') {
            // Add to current rule
            current_rule.as_mut().unwrap().tokens.push(lex(line));
        } else {
            // New rule
            if let Some(rule) = current_rule.take() {
                rules.push(rule);
            }
            current_rule = Some(Rule {
                name: line,
                tokens: vec![],
            })
        }
    }
    rules
});

pub struct Warning<'a> {
    pub description: &'static str,
    pub tokens: &'a [Token<'a>],
}

pub fn find_matches<'a>(tokens: &'a [Token<'a>]) -> Vec<Warning<'a>> {
    let mut warnings = vec![];
    for i in 0..tokens.len() - 1 {
        let target = &tokens[i..];
        for rule in RULES.iter() {
            for pattern in &rule.tokens {
                if pattern.len() > target.len() {
                    continue;
                }
                if let Some(matched_tokens) = check_match(tokens, pattern) {
                    // Matched here
                    warnings.push(Warning {
                        description: rule.name,
                        tokens: matched_tokens,
                    })
                }
            }
        }
    }
    warnings
}

/// Checks if the given token sequence matches the given pattern. Returns the matched portion.
pub fn check_match<'a>(
    tokens: &'a [Token<'a>],
    pattern: &'a [Token<'a>],
) -> Option<&'a [Token<'a>]> {
    let mut index = 0;
    let mut pattern = pattern.iter().peekable();
    while let Some(expected) = pattern.next() {
        match expected.value {
            "_" => {
                // Match an identifier or keyword
                if index >= tokens.len() {
                    return None;
                }
                let token = &tokens[index];
                index += 1;
                if !matches!(token.value.chars().next().unwrap(), 'a'..='z' | 'A'..='Z' | '_') {
                    return None;
                }
            }
            "__" => {
                // Match any token
                if index >= tokens.len() {
                    return None;
                }
                index += 1;
            }
            "___" => {
                // Match any sequence of tokens
                let ending = *pattern.peek().expect("cannot end pattern with ___");
                while tokens.get(index) != Some(&ending) {
                    if index >= tokens.len() {
                        return None;
                    }
                    index += 1;
                }
            }
            _ => {
                // Match the token
                if index >= tokens.len() {
                    return None;
                }
                if &tokens[index] != expected {
                    return None;
                }
                index += 1;
            }
        }
    }

    Some(&tokens[..index])
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex;
    use crate::matcher::check_match;

    #[test]
    fn test_literal() {
        assert_eq!(
            check_match(&lex("foo\n\tbar\t3.14   baz   a"), &lex("foo bar 3.14 baz"),),
            Some(lex("foo bar 3.14 baz").as_slice()),
        )
    }

    #[test]
    fn test_any_identifier() {
        assert_eq!(
            check_match(&lex("foo\n\tbar\t3.14   baz   a"), &lex("foo _ 3.14 baz"),),
            Some(lex("foo bar 3.14 baz").as_slice()),
        )
    }

    #[test]
    fn test_any_identifier_not_matching() {
        assert_eq!(
            check_match(&lex("foo\n\tbar\t3.14   baz   a"), &lex("foo bar _ baz"),),
            None,
        )
    }

    #[test]
    fn test_any_token() {
        assert_eq!(
            check_match(&lex("foo\n\tbar\t3.14   baz   a"), &lex("foo bar __ baz"),),
            Some(lex("foo bar 3.14 baz").as_slice()),
        )
    }

    #[test]
    fn test_any_token_sequence() {
        assert_eq!(
            check_match(&lex("foo\n\tbar\t3.14   baz   a"), &lex("foo ___ baz"),),
            Some(lex("foo bar 3.14 baz").as_slice()),
        )
    }

    #[test]
    fn test_any_token_sequence_empty() {
        assert_eq!(
            check_match(
                &lex("foo\n\tbar\t3.14   baz   a"),
                &lex("foo bar ___ 3.14 baz"),
            ),
            Some(lex("foo bar 3.14 baz").as_slice()),
        )
    }
}
