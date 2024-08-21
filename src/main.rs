extern crate gettext;

fn main() {
    let domain = gettext::Catalog::domain("messages").unwrap();
    let translated_text = domain.gettext("Hello, world!");
    println!("{}", translated_text);
}
