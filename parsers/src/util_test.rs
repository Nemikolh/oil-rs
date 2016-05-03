#[macro_export]
macro_rules! assert_variant {
    ($e:expr, $variant:path) => (
        if let $variant(..) = $e {
        } else {
            assert!(false);
        }
    );
    ($e:expr, brace $variant:path) => (
        if let $variant{..} = $e {
        } else {
            assert!(false);
        }
    );
}
