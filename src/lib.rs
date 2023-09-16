#![allow(dead_code, unused)]
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod extend;

/// This macro extends a given enum with a enum provided in the function call.
/// The enum provided in the function call must be a valid enum. otherwise it will fail
/// for example
/// ```
/// enum A {
///     A1,
///     A2,
/// }
///
/// #[derive(Extend(A))]
/// enum B {
///     B1,
///     B2,
///     B3,
/// }
///
/// ```
/// This will generate the following code
/// ```
/// enum B {
///     B1,
///     B2,
///     B3,
///     A1,
///     A2,
/// }
/// ```
#[proc_macro_derive(Extend, attributes(extends))]
pub fn extends_(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    extend::expand_extend(input)
}
