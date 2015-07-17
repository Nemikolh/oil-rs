use std::slice;
use Store;

/// `StoreValue` is the type that encapsulate
/// a value extracted from a Store  
pub enum StoreValue<'a> {
    String(String),
    Integer(i64),
    Boolean(bool),
    List(Box<Iterator<Item=&'a Store> + 'a>)
}

/// Equivalent to of `StoreValue<'static>` to allow the implementation
/// of a :
/// ```ignore
/// impl<'a> From<StoreValue<'static>> for StoreValue<'a> {
///     ...
/// }
/// ```
#[derive(Clone)]
pub struct StoreValueStatic(pub StoreValue<'static>);

impl<'a> Clone for StoreValue<'a> {
    fn clone(&self) -> Self {
        match self {
            &StoreValue::String(ref s) => StoreValue::String(s.clone()),
            &StoreValue::Integer(ref i) => StoreValue::Integer(i.clone()),
            &StoreValue::Boolean(ref b) => StoreValue::Boolean(b.clone()),
            &StoreValue::List(_) => panic!("StoreValue::List can't be cloned")
        }
    }
}

impl<'a> From<StoreValueStatic> for StoreValue<'a> {
    fn from(s: StoreValueStatic) -> StoreValue<'a> {
        let StoreValueStatic(v) = s;
        match v {
            StoreValue::String(s) => StoreValue::String(s),
            StoreValue::Integer(i) => StoreValue::Integer(i),
            StoreValue::Boolean(b) => StoreValue::Boolean(b),
            StoreValue::List(_) => panic!("StoreValue::List can't be converted")
        }
    }
}

macro_rules! impl_for_integer {
    ($int_type:ident) => (
        impl<'a> From<$int_type> for StoreValue<'a> {
            fn from(i: $int_type) -> StoreValue<'a> {
                StoreValue::Integer(i as i64)
            }
        }
    )
}

impl_for_integer!(i64);
impl_for_integer!(i32);
impl_for_integer!(i16);
impl_for_integer!(i8);

impl_for_integer!(u32);
impl_for_integer!(u16);
impl_for_integer!(u8);

impl<'a> From<String> for StoreValue<'a> {
    fn from(s: String) -> StoreValue<'a> {
        StoreValue::String(s)
    }
}

impl<'a> From<bool> for StoreValue<'a> {
    fn from(b: bool) -> StoreValue<'a> {
        StoreValue::Boolean(b)
    }
}

struct WrapperIter<'a, T: 'a> {
    it: slice::Iter<'a, T>,
}

impl<'a, T> Iterator for WrapperIter<'a, T>
    where T: Store + 'a
{
    type Item = &'a Store;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|i| i as &Store)
    }
}

impl<'a, T> From<&'a [T]> for StoreValue<'a> 
    where T: Store
{

    fn from(t: &'a [T]) -> StoreValue<'a> {
        let i = Box::new(WrapperIter { it: t.iter() }) as Box<Iterator<Item=&'a Store> + 'a>;
        StoreValue::List(i)
    }
}