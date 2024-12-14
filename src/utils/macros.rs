/**
https://github.com/rust-lang/rust/blob/1.83.0/library/core/src/internal_macros.rs#L23
*/
#[macro_export]
macro_rules! forward_ref_binop {
    ($imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl $imp<$u> for $t {
            type Output = $t;

            #[inline]
            #[track_caller]
            fn $method(self, other: $u) -> Self::Output {
                $imp::$method(&self, &other)
            }
        }
        impl<'a> $imp<$u> for &'a $t {
            type Output = $t;

            #[inline]
            #[track_caller]
            fn $method(self, other: $u) -> Self::Output {
                $imp::$method(self, &other)
            }
        }
        impl $imp<&$u> for $t {
            type Output = $t;

            #[inline]
            #[track_caller]
            fn $method(self, other: &$u) -> Self::Output {
                $imp::$method(&self, other)
            }
        }
    };
}

/**
https://github.com/rust-lang/rust/blob/1.83.0/library/core/src/ops/arith.rs#L94
*/
#[macro_export]
macro_rules! impl_ops_ref_copy {
    ($imp:ident, $method:ident |$t_i:ident: $t:ty, $u_i:ident: $u:ty| $ex:expr) => {
        impl<'a, 'b> $imp<&'b $u> for &'a $t {
            type Output = $t;

            #[inline]
            #[track_caller]
            fn $method(self, $u_i: &'b $u) -> $t {
                let $t_i = self;
                $ex
            }
        }
        forward_ref_binop! { $imp, $method for $t, $u }
    };
}

#[macro_export]
macro_rules! impl_ops_assign_ref_copy {
    ($imp:ident, $method:ident |$t_i:ident: $t:ty, $u_i:ident: $u:ty| $ex:expr) => {
        impl<'a, 'b> $imp<&'b $u> for $t {
            #[inline]
            #[track_caller]
            fn $method(&mut self, $u_i: &'b $u) {
                let $t_i = self;
                $ex
            }
        }
        impl<'a> $imp<$u> for $t {
            #[inline]
            #[track_caller]
            fn $method(&mut self, $u_i: $u) {
                let $t_i = self;
                $ex
            }
        }
    };
}