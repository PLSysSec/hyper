macro_rules! ffi_fn {
    ($(#[$doc:meta])* fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty $body:block ?= $default:expr) => {
        $(#[$doc])*
        #[no_mangle]
        pub extern fn $name($($arg: $arg_ty),*) -> $ret {
            use std::panic::{self, AssertUnwindSafe};

            match panic::catch_unwind(AssertUnwindSafe(move || $body)) {
                Ok(v) => v,
                Err(_) => {
                    $default
                }
            }
        }
    };

    ($(#[$doc:meta])* fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty $body:block) => {
        ffi_fn!($(#[$doc])* fn $name($($arg: $arg_ty),*) -> $ret $body ?= {
            eprintln!("panic unwind caught, aborting");
            std::process::abort()
        });
    };

    ($(#[$doc:meta])* fn $name:ident($($arg:ident: $arg_ty:ty),*) $body:block ?= $default:expr) => {
        ffi_fn!($(#[$doc])* fn $name($($arg: $arg_ty),*) -> () $body ?= $default);
    };

    ($(#[$doc:meta])* fn $name:ident($($arg:ident: $arg_ty:ty),*) $body:block) => {
        ffi_fn!($(#[$doc])* fn $name($($arg: $arg_ty),*) -> () $body);
    };
}

macro_rules! non_null {
    ($ptr:ident, $eval:expr, $err:expr) => {{
        debug_assert!(!$ptr.is_null(), "{:?} must not be null", stringify!($ptr));
        if $ptr.is_null() {
            return $err;
        }
        { $eval }
    }};
    (safe_as_ref($ptr:ident) ?= $err:expr) => {{
        non_null!($ptr, crate::ffi::safe_ffi::safe_as_ref($ptr), $err)
    }};
    (safe_as_ref_mut($ptr:ident) ?= $err:expr) => {{
        non_null!($ptr, crate::ffi::safe_ffi::safe_as_ref_mut($ptr), $err)
    }};
    (safe_box_from_raw($ptr:ident) ?= $err:expr) => {{
        non_null!($ptr, crate::ffi::safe_ffi::safe_box_from_raw($ptr), $err)
    }};
    (safe_arc_from_raw($ptr:ident) ?= $err:expr) => {{
        non_null!($ptr, crate::ffi::safe_ffi::safe_arc_from_raw($ptr), $err)
    }};
}
