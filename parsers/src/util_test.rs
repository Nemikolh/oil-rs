#[macro_export]
macro_rules! assert_variant {
    ($e:expr, $variant:path) => (
        if let $variant(..) = $e {
        } else {
            panic!("Expected variant {}!", stringify!($variant));
        }
    );
    ($e:expr, brace $variant:path) => (
        if let $variant{..} = $e {
        } else {
            panic!("Expected variant {}!", stringify!($variant));
        }
    );
}
