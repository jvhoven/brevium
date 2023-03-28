#[macro_export]
macro_rules! item {
    ($name:ident<'a> { $($field:ident: $ty:ty),* $(,)* }) => {
        #[derive(Default, Debug)]
        pub struct $name<'a> {
            pub name: &'a str,
            pub tags: HashMap<&'a str, &'a str>,
            $(pub $field: $ty),*
        }

        impl<'a> fmt::Display for $name<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}\n{:?}", self.name, self.tags)
            }
        }
    };
}
