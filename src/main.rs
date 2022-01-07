mod pretty_html;

use pretty_html as pretty;

fn main() {
    println!("{}", pretty::make_html("its title".to_string(), pretty::Structure::from("its body")));
}
