#![allow(dead_code)]
use strum_ext::Extend;

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

enum A {
    A1,
    A2,
}

enum C<T> {
    C1,
    C2(T),
}

#[derive(Extend)]
#[extends((A, C))]
enum B {
    B1,
    B2,
    B3,
}
