use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::spanned::Spanned;

#[proc_macro_derive(Parser)]
pub fn parser_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_parser(&ast)
}

// the actual macro
fn impl_parser(ast: &syn::DeriveInput) -> TokenStream {
    //get struct name
    let struct_name = &ast.ident;

    // get the fields
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only supporting structs")
    };

    // create vectors for keys and values
    let mut keys = Vec::new();
    let mut values = Vec::new();

    // parse and add to appropriate vectors
    for field in fields.named.iter() {
        let field_name: &syn::Ident = field.ident.as_ref().unwrap();
        let name: String = field_name.to_string();
        let literal_key_str = syn::LitStr::new(&name, field.span());
        keys.push(quote! { #literal_key_str });
        values.push(&field.ident);
    }

    // print
    let expanded = quote! {
        impl Parser for #struct_name {
            fn parse_struct(&self) {
                // open json
                println!("\"{}\": {{", stringify!(#struct_name));
                
                // body, cool looping style
                #(
                    println!(
                        "\t\"{key}\": \"{value}\",",
                        key = #keys,
                        value = self.#values,
                    );
                )*
                
                // close json
                println!("}}");
            }
        }
    };

    expanded.into()
}
