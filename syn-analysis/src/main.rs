use syn;

fn main() -> Result<(), ()> {
    let code = "assert_eq!(u8::max_value(), 255)";
    let ast = syn::parse_str::<syn::Expr>(code);
    println!("{:#?}", ast);
    Ok(())
}
