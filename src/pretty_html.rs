fn close(tag: &str) -> String {
    let tag = &tag[1..];
    "</".to_owned() + tag
}

fn wrap<'a>(tag: &'a str, content: &'a str) -> String {
    tag.to_owned() + content + &close(tag)
}

fn wrap_html(content: String) -> String {
    wrap("<html>", &content)
}

fn wrap_body(content: &str) -> String {
    wrap("<body>", content)
}

fn wrap_head(content: &str) -> String {
    wrap("<head>", content)
}

fn wrap_title(content: &str) -> String {
    wrap("<title>", content)
}

pub fn make_html<'a>(title: &'a str, body: &'a str) -> String {
    wrap_html(wrap_head(&wrap_title(title)) + &wrap_body(body))
}
