macro_rules! decl_enum {
    ($name:ident) => {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

macro_rules! decl_flag {
    ($name:ident) => {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub struct $name(pub u32);
    }
}

macro_rules! impl_flag {
    ($name:ident) => {
        impl ::std::ops::BitOr for $name {
            type Output = $name;

            #[inline]
            fn bitor(self, other: $name) -> $name {
                $name(self.0 | other.0)
            }
        }

        impl ::std::ops::BitAnd for $name {
            type Output = $name;

            #[inline]
            fn bitand(self, other: $name) -> $name {
                $name(self.0 & other.0)
            }
        }
    }
}

#[macro_export]
macro_rules! make_flag {
    ($name:ident; $($variant:ident = $value:expr,)*) => {
        decl_flag!{$name}
        impl_enum!{$name; $($variant = $value,)*}
        impl_flag!{$name}
    }
}
