// C enums

macro_rules! decl_enum {
    ($name:ident) => {
        #[repr(C)]
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub struct $name(pub i32);

        impl ::std::default::Default for $name {
            fn default() -> Self { $name(0) }
        }
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

// C flags

macro_rules! decl_flag {
    ($name:ident) => {
        #[repr(C)]
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub struct $name(pub u32);

        impl ::std::default::Default for $name {
            fn default() -> Self { $name(0) }
        }
    }
}

macro_rules! impl_bitwise {
    ($lhs:ident, $rhs:ident, $out:ident) => {
        impl ::std::ops::BitOr<$rhs> for $lhs {
            type Output = $out;

            #[inline]
            fn bitor(self, other: $rhs) -> Self::Output {
                $out(self.0 | other.0)
            }
        }

        impl ::std::ops::BitAnd<$rhs> for $lhs {
            type Output = $out;

            #[inline]
            fn bitand(self, other: $rhs) -> Self::Output {
                $out(self.0 & other.0)
            }
        }
    }
}

#[macro_export]
macro_rules! make_flag {
    ($flag:ident; $flags:ident; $($variant:ident = $value:expr,)*) => {
        decl_flag!{$flag}
        decl_flag!{$flags}
        impl_enum!{$flag; $($variant = $value,)*}
        impl_bitwise!{$flag, $flag, $flags}
        impl_bitwise!{$flag, $flags, $flags}
        impl_bitwise!{$flags, $flag, $flags}
        impl_bitwise!{$flags, $flags, $flags}
    }
}
