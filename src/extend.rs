use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataEnum, DeriveInput, Expr,
    ExprPath, ExprTuple, Ident,
};

pub(crate) fn expand_extend(ast: DeriveInput) -> TokenStream {
    // let mut impls = vec![];
    let mut broad_enum = match ast.data {
        Data::Enum(data) => data,
        _ => panic!("Extend can only be derived for enums"),
    };

    // these should really be ident
    let narrow_enums = ast
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("extends"))
        .flat_map(|attribute| match attribute.parse_args::<ExprTuple>() {
            Ok(item) => item.elems.into_iter(),
            Err(err) => panic!(
                "Expected a tuple of enums separated by commas, if single item use (item,): {err}"
            ),
        });

    for other_enum_path in narrow_enums {
        match other_enum_path {
            Expr::Path(path) => {
                println!("{:#?}", path.path.segments[0].ident);
            }
            _ => panic!("Expected a path"),
        }

        // println!("{:#?}", other_enum_path);

        // for token in impl_from(&ast.ident, &broad_enum, &enum_data, &ident) {
        //     impls.push(token);
        // }

        // for variant in enum_data.variants {
        //     broad_enum.variants.push(variant);
        // }
    }

    // let expanded = quote! {
    //     #ast
    //     #(#impls)*
    // };

    // expanded.into()
    TokenStream::new()
}

/// This macro extends a given enum with a enum provided in the function call.
/// The enum provided in the function call must be a valid enum. otherwise it will fail
/// for example
/// ```
/// enum A {
///     A1,
///     A2,
/// }
///
/// enum B {
///     B1,
///     B2,
///     B3,
/// }
///
/// #[derive(Extend)]
/// #[extends(A, B)]
/// enum C {
///     C1,
///     C2,
///     C3,
/// }
///
/// ```
/// This will generate the following code
/// ```
/// enum C {
///     A1,
///     A2,
///     B1,
///     B2,
///     B3,
///     C1,
///     C2,
///     C3,
/// }
/// ```
/// other examples
/// ```
/// enum A {
///    A1,
///    A2,
/// }
///
/// #[derive(Extend)]
/// #[extends(A)]
/// enum B {
///    B1,
///    B2,
///    B3,
/// }
///
/// #[derive(Extend)]
/// #[extends(B)]
/// enum C {
///     C1,
///     C2,
/// }
///
/// ```
/// This will generate the following code
/// ```
///
/// enum B {
///     A1,
///     A2,
///     B1,
///     B2,
///     B3,  
/// }
///
/// enum C {
///     A1,
///     A2,
///     B1,
///     B2,
///     B3,
///     C1,
///     C2,
///     C3,
/// }
/// ```
/// This will also implement the `From` trait for specific enums to broader enums
/// so in the above example
/// A will implement `From<A> for B` and `From<A> for C` without any restrictions
/// B will implement `From<B> for C` without any restrictions
/// but in order to implement `From<C> for B` or `From<C> for A`,
/// you need to implement the `Default` trait for the enums A and B
/// ```
/// impl Default for A {
///     fn default() -> Self {
///         A::A1
///     }
/// }
///
/// impl Default for B {
///     fn default() -> Self {
///         B::B1
///     }
/// }
///
/// impl Default for C {
///     fn default() -> Self {
///         C::A1
///     }
/// }
///
/// ```
/// Then the `From` trait will be implemented all the enums.
pub fn derive_extend(mut ast: DeriveInput) -> TokenStream {
    // println!("{:#?}", attr.into_token_stream());

    ast.into_token_stream().into()

    // only keep the attributes that are #[extends(...)]
    // let narrow_enums = ast
    //     .attrs
    //     .iter()
    //     .filter(|attr| attr.path().is_ident("extends"));

    // let broader_enum = match &mut ast.data {
    //     Data::Enum(data) => data,
    //     _ => panic!("Extend can only be derived for enums"),
    // };
    // let mut all_from_impls = vec![];
    // for other_enum in narrow_enums {
    //     let (enum_data, ident) = match other_enum.parse_args::<DeriveInput>() {
    //         Ok(data_input) => match data_input.data {
    //             Data::Enum(data) => (data, data_input.ident),
    //             _ => panic!("Enums can only be extended with other enums"),
    //         },
    //         Err(err) => panic!("Invalid enum provided: {err}"),
    //     };

    //     for token in impl_from(&ast.ident, broader_enum, &enum_data, &ident) {
    //         all_from_impls.push(token);
    //     }

    //     for variant in enum_data.variants {
    //         broader_enum.variants.push(variant);
    //     }
    // }

    // println!("{:#?}", all_from_impls);

    // let expanded = quote! {
    //     #ast
    // };

    // expanded.into()
}

fn impl_from(
    broad_enum_name: &Ident,
    broad_enum: &DataEnum,
    narrow_enum: &DataEnum,
    narrow_enum_name: &Ident,
) -> Vec<TokenStream> {
    let mut impls = vec![];
    let narrow_quoted_variants = narrow_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        quote! {
            #narrow_enum_name::#ident => Self::#ident,
        }
    });

    let impl_narrow_to_broad = quote! {
        impl From<#narrow_enum_name> for #broad_enum_name {
            fn from(value: #narrow_enum_name) -> Self {
                match value {
                    #(#narrow_quoted_variants)*
                }
            }
        }
    };

    impls.push(impl_narrow_to_broad.into());

    let broad_quoted_variants = broad_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        quote! {
            #broad_enum_name::#ident => Self::#ident,
        }
    });

    let impl_broad_to_narrow = quote! {
        impl From<#broad_enum_name> for #narrow_enum_name {
            fn from(value: #broad_enum_name) -> Self {
                match value {
                    #(#broad_quoted_variants)*
                    _ => Self::default(),
                }
            }
        }
    };

    impls.push(impl_broad_to_narrow.into());

    impls
}
