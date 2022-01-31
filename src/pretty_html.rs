#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use std::ops::Add;

pub struct Html(String);

pub struct Structure(String);

type Title = String;

impl Html {
    pub fn from(s: &str) -> Html {
        Html(s.to_string())
    }
}

impl Add for Html {
    type Output = String;
    fn add(self, rhs: Self) -> Self::Output {
        self.0 + rhs.0.as_str()
    }
}

impl Display for Html {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Structure {
    pub fn from(s: &str) -> Structure {
        Structure(s.to_string())
    }
}

impl Add for Structure {
    type Output = Structure;
    fn add(self, rhs: Self) -> Self::Output {
        Structure::from(&(self.0 + &*rhs.0))
    }
}

fn close(tag: String) -> String {
    let tag = &tag[1..];
    "</".to_owned() + tag
}

fn wrap(tag: &str, content: String) -> String {
    format!("{}{}{}", tag, content, close(tag.to_string()))
}

pub fn escape(content: String) -> String {
    let mut r: String = "".to_string();
    for c in content.chars() {
        let ac: String;
        match c {
            '<' => ac = "&lt;".to_string(),
            '>' => ac = "&gt;".to_string(),
            '&' => ac = "&amp;".to_string(),
            '"' => ac = "&quot;".to_string(),
            '\'' => ac = "&#39;".to_string(),
            _ => ac = c.to_string()
        }
        r += &*ac;
    }
    r
}

pub fn p_(content: String) -> Structure {
    Structure(wrap("<p>", content))
}

pub fn h1_(content: String) -> Structure {
    Structure(wrap("<h1>", content))
}

pub fn make_html(title: Title, body: Structure) -> Html {
    Html(
        wrap("<html>", wrap(
            "<head>", wrap("<title>", escape(title)),
        ) + &*wrap("<body>", body.0))
    )
}