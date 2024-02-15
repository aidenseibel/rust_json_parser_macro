use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::spanned::Spanned;

#[proc_macro_derive(Parser)]
pub fn parser_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_parser(&ast)
}

fn impl_parser(ast: &syn::DeriveInput) -> TokenStream {
    //get struct name
    let struct_name = &ast.ident;

    // get the data
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only supporting structs")
    };

    // parse and add to vectors
    let mut keys = Vec::new();
    let mut idents = Vec::new();
    for field in fields.named.iter() {
        let field_name: &syn::Ident = field.ident.as_ref().unwrap();
        let name: String = field_name.to_string();
        let literal_key_str = syn::LitStr::new(&name, field.span());
        keys.push(quote! { #literal_key_str });
        idents.push(&field.ident);
    }

    // print out the vectors
    let expanded = quote! {
        impl Parser for #struct_name {
            fn parse_struct(&self) {
                println!("\"{}\": {{", stringify!(#struct_name));

                #(
                    println!(
                        "\t\"{key}\": \"{value}\",",
                        key = #keys,
                        value = self.#idents,
                    );
                )*
                println!("}}");
            }
        }
    };
    expanded.into()
}
