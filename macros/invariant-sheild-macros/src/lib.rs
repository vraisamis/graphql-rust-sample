use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, Error as SynError, Ident, ImplItem, ItemImpl, Path};

const INVARIANT_CONDITION_ATTRIBUTE_NAME: &'static str = "sheild";

#[proc_macro_attribute]
pub fn invariant_sheild(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Path);
    let mut ast = parse_macro_input!(item as ItemImpl);
    match generate(&mut ast, attr) {
        Ok(result) => result,
        Err(e) => e.to_compile_error().into(),
    }
}

fn generate(item_impl: &mut ItemImpl, err_type: Path) -> Result<TokenStream, SynError> {
    // type crate
    let type_crate: Ident = detect_crate_name();
    // implement types
    let self_type = &item_impl.self_ty;

    // list invariant conditions
    let mut conditions: Vec<Ident> = vec![];
    for item in &mut item_impl.items {
        match item {
            ImplItem::Fn(f) => {
                if has_invariant_condition(&f.attrs) {
                    conditions.push(format_ident!("{}", f.sig.ident));
                    remove_invariant_condition_attrs(&mut f.attrs);
                }
            }
            _ => {}
        }
    }

    // output implementation of Invariants
    Ok(quote! {
        #item_impl

        impl ::#type_crate::InvariantSheild for #self_type {
            type Error = #err_type;
            fn sheilds() -> Vec<&'static dyn Fn(&Self) -> Result<(), <Self as ::#type_crate::InvariantSheild>::Error>> {
                vec![
                    #(&Self::#conditions),*
                ]
            }
        }
    }
    .into())
}

fn detect_crate_name() -> Ident {
    let name = match crate_name("invariant-sheild") {
        Ok(FoundCrate::Name(s)) => s,
        Ok(FoundCrate::Itself) | Err(_) => "invariant_sheild".to_owned(),
    };
    Ident::new(&name, Span::call_site())
}

fn has_invariant_condition(attrs: &[Attribute]) -> bool {
    attrs.iter().any(is_invariant_condition)
}

fn is_invariant_condition(attr: &Attribute) -> bool {
    match &attr.meta {
        syn::Meta::Path(path) => path.segments.last().map_or(false, |pathseg| {
            pathseg.ident == INVARIANT_CONDITION_ATTRIBUTE_NAME
        }),
        _ => false,
    }
}

fn remove_invariant_condition_attrs(attrs: &mut Vec<Attribute>) {
    attrs.retain(|a| !is_invariant_condition(a))
}
