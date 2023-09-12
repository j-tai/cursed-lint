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
                if (0..pattern.len()).all(|j| target[j] == pattern[j] || pattern[j] == "_") {
                    // Matched here
                    warnings.push(Warning {
                        description: rule.name,
                        tokens: target,
                    })
                }
            }
        }
    }
    warnings
}
