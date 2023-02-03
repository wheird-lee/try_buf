macro_rules! define_try_get {
    (@def_be $($type: ty)*) => {
        $(
            $crate::paste! {
                fn [<try_get_ $type>](&mut self) -> $crate::Result<$type>;
            }
        )*
    };

    ($($type: ty)*) => {
        $(
            $crate::paste! {
                fn [<try_get_ $type _le>](&mut self) -> $crate::Result<$type>;
            }
        )*
        define_try_get!(@def_be $($type)*);
    };
}

macro_rules! impl_try_get {
    (@impl $type:ty) => {
        $crate::paste! {
            fn [<try_get_ $type>](&mut self) -> $crate::Result<$type> {
                use std::mem::size_of;
                if self.remaining() >= size_of::<$type>() {
                    $crate::Result::Ok(self.[<get_ $type>]())
                } else {
                    $crate::Result::Err($crate::ErrorKind::EOF)
                }
            }
        }
    };

    (@impl_le $type:ty) => {
        $crate::paste!{
            fn [<try_get_ $type _le>](&mut self) -> $crate::Result<$type> {
                use std::mem::size_of;
                if self.remaining() >= size_of::<$type>() {
                    $crate::Result::Ok(self.[<get_ $type _le>]())
                } else {
                    $crate::Result::Err($crate::ErrorKind::EOF)
                }
            }
        }
    };

    (@impls_be $($type:ty)*) => {
        $(
            impl_try_get!(@impl $type);
        )*
    };

    ($($type: ty)*) => {
        $(
            impl_try_get!(@impl $type);
            impl_try_get!(@impl_le $type);
        )*
    };

}
