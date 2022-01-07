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

impl Structure {
    pub fn from(s: &str) -> Structure {
        Structure(s.to_string())
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

fn close(tag: String) -> String {
    let tag = &tag[1..];
    "</".to_owned() + tag
}

fn wrap(tag: &str, content: String) -> String {
    format!("{}{}{}", tag, content, close(tag.to_string()))
}

fn p_(content: String) -> Structure {
    Structure(wrap("<p>", content))
}

fn h1_(content: String) -> Structure {
    Structure(wrap("<h1>", content))
}

pub fn make_html(title: Title, body: Structure) -> Html {
    Html(
        wrap("<html>", wrap(
            "<head>", wrap("<title>", title)
        )) + &*wrap("<body>", body.0)
    )
}
