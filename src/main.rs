mod pretty_html;
use pretty_html as pretty;

fn main() {
    println!("{}", pretty::make_html("its title", "its body"));
}
