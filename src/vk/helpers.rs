#[macro_export]
macro_rules! decl_enum {
    ($name:ident) => {
        #[derive(Clone, Eq, PartialEq, Debug)]
        pub struct $name(pub i32);
    }
}

#[macro_export]
macro_rules! impl_enum {
    ($name:ident; $($variant:ident = $value:expr,)*) => {
        #[allow(non_upper_case_globals)]
        impl $name {
            $(pub const $variant: $name = $name($value);)*
        }
    }
}

#[macro_export]
macro_rules! make_enum {
    ($name:ident; $($variant:ident = $value:expr,)*) => {
        decl_enum!{$name}
        impl_enum!{$name; $($variant = $value,)*}
    }
}
