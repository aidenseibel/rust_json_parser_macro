use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Parser)]
pub fn parser_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_parser(&ast)
}

fn impl_parser(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let gen = quote! {
        impl Parser for #name {
            fn parse_struct(){
                println!("\"{}\": {{", stringify!(#name));
                match #data{
                    Struct(s) => {println!("Struct!")}
                    Enum(e) => {println!("Enum!")}
                    Union(d) => {println!("Union")}
                    _ => {println!("What?")}
                }
                println!("}}");
            }
        }
    };
    gen.into()
}
