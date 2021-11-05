#[derive(macro_pckg::EnumVariantCount)]
pub enum Foo {
    Bar,
    Baz,
}

fn main() {
    println!("Macro length: {}", LENGTH);
}
