use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Clone, Eq, Debug)]
pub struct Token<'a> {
    pub value: &'a str,
    pub line: usize,
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<'a> PartialEq<&'a str> for Token<'a> {
    fn eq(&self, other: &&'a str) -> bool {
        self.value == *other
    }
}

/// Convert the given string into a list of tokens.
pub fn lex(text: &str) -> Vec<Token> {
    let mut tokens = vec![];
    for (line_index, line) in text.lines().enumerate() {
        let mut line = line.trim();
        while !line.is_empty() {
            let mtch = TOKEN_REGEX.find(line).unwrap();
            tokens.push(Token {
                value: &line[..mtch.end()],
                line: line_index + 1,
            });
            line = &line[mtch.end()..];
            line = line.trim_start();
        }
    }
    tokens
}

static TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!(
        r"^(?:",                    // prologue
        r"[a-zA-Z_][a-zA-Z0-9_]*|", // variable name or keyword
        r"\d[\w']*(?:\.[\w']*)?|",  // numeric constant
        r"\.\d[\w']*|",             // numeric constant starting with '.'
        r#""([^\\"]|\\.)*"|"#,      // string literal
        r"'([^\\']|\\.)*'|",        // character literal
        r"//[^\n]*|",               // line comment
        r"/\*.*\*/|",               // block comment
        r".",                       // symbol
        r")",                       // epilogue
    ))
    .unwrap()
});

#[cfg(test)]
mod tests {
    use crate::lexer::lex;

    #[test]
    fn lex_everything() {
        assert_eq!(
            lex("  foo \t \n\r\t bar^(3.14)foo \"str!lit eral \" '\"' "),
            vec![
                "foo",
                "bar",
                "^",
                "(",
                "3.14",
                ")",
                "foo",
                r#""str!lit eral ""#,
                "'\"'",
            ],
        );
    }
}
