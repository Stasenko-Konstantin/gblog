use pretty_html as pr;

mod pretty_html;

fn main() {
    let my_html =
        pr::make_html("its title".to_string(),
                      pr::h1_("its <body>".to_string()) +
                          pr::ol_(vec!["item 1", "item 2", "item 3"]
                              .iter()
                              .map(|x| pr::p_(x.to_string()))
                              .collect()) +
                          (pr::p_("Paragraph #1".to_string()) +
                              pr::code_("Paragraph #2".to_string())));
    println!("{}", my_html);
}
