//! Poor man's `yeet` keyword on stable.
#![no_std]
#![forbid(unsafe_code, missing_docs)]
#![cfg_attr(test, deny(clippy::pedantic, clippy::nursery))]

/*
    This is free and unencumbered software released into the public domain.

    Anyone is free to copy, modify, publish, use, compile, sell, or
    distribute this software, either in source code form or as a compiled
    binary, for any purpose, commercial or non-commercial, and by any
    means.

    In jurisdictions that recognize copyright laws, the author or authors
    of this software dedicate any and all copyright interest in the
    software to the public domain. We make this dedication for the benefit
    of the public at large and to the detriment of our heirs and
    successors. We intend this dedication to be an overt act of
    relinquishment in perpetuity of all present and future rights to this
    software under copyright law.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
    IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
    OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
    ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
    OTHER DEALINGS IN THE SOFTWARE.

    For more information, please refer to <http://unlicense.org/>.

    You should credit me, if possible. But if you don't, I don't care.
*/

/// A trait for a return type that can be converted from a `yeet!`ed value.
pub trait Yeetable {
    /// The type of the `yeet!`ed value.
    type Err;

    /// Convert the `yeet!`ed value to the return type.
    fn from_err(err: Self::Err) -> Self;
}

impl<T> Yeetable for Option<T> {
    type Err = ();
    fn from_err((): ()) -> Self {
        None
    }
}

impl<T, E> Yeetable for Result<T, E> {
    type Err = E;
    fn from_err(err: E) -> Self {
        Err(err)
    }
}

/// The `yeet` macro. Does not work inside `try` blocks.
#[macro_export]
macro_rules! yeet {
    () => {{return $crate::Yeetable::from_err(())}};
    ($e:expr) => {{return $crate::Yeetable::from_err($e)}};
}

pub use self::yeet as raise;
pub use self::yeet as throw;

#[cfg(test)]
mod tests {
    use crate::yeet;

    fn foo() -> Option<u32> {
        yeet!();
    }

    fn bar() -> Result<u32, u32> {
        yeet!(42);
    }

    fn baz(x: u32) -> Result<u32, u32> {
        if x % 2 == 1 {
            yeet!(x);
        }

        Ok(x)
    }

    #[test]
    fn test() {
        assert_eq!(foo(), None);
        assert_eq!(bar(), Err(42));
        assert_eq!(baz(42), Ok(42));
        assert_eq!(baz(43), Err(43));
    }
}
