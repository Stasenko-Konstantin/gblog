use pretty_html as pretty;

mod pretty_html;

fn main() {
    let my_html =
        pretty::make_html("its title".to_string(),
                          pretty::h1_(pretty::escape("its <body>".to_string())) +
                              (pretty::p_("Paragraph #1".to_string()) +
                                  pretty::p_("Paragraph #2".to_string())));
    println!("{}", my_html);
}
