mod ref_iter;
pub use ref_iter::RefIter;

// =================================================================================================
//                                         Mutate Trait
// =================================================================================================

// Just re-export from this awesome crate
pub use pipe_trait::Pipe;

// =================================================================================================
//                                         Mutate Trait
// =================================================================================================

pub trait Mutate {
    fn mutate<F>(self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&mut Self),
    {
        let mut this = self;
        f(&mut this);
        this
    }
}

impl<T> Mutate for T {}

// =================================================================================================
//                                         Dynamic Stuff
// =================================================================================================

#[macro_export]
macro_rules! dyn_fn {
    ($e:expr) => {{
        (&$e) as &dyn Fn(_) -> _
    }};
}

#[macro_export]
macro_rules! dyn_fn_mut {
    ($e:expr) => {{
        (&mut $e) as &mut dyn FnMut(_) -> _
    }};
}

// =================================================================================================
//                                            Default
// =================================================================================================

/// A short-hand for calling `Default::default()`. You may also specify `T`, but I recommend
/// using `T::default()` over `default::<T>()` where you may.
pub fn default<T: Default>() -> T {
    T::default()
}

// =================================================================================================
//                                             Into
// =================================================================================================

/// This trait exposes another function named `into`, that can take a generic argument, making it
/// nicer to use in pipe-lines.
///
/// ## Example
/// ```
/// use functionality::IntoExt;
/// let str = "hello";
/// // You can't do this with `into`!
/// let string = str.into_::<String>();
/// // You'd have to use `String::from(str)`
/// ```
#[rustfmt::skip]
pub trait IntoExt {
    fn into_<T>(self) -> T where Self: Into<T> { Into::<T>::into(self) }
}

impl<T> IntoExt for T {}

// =================================================================================================
//                                              New
// =================================================================================================

/// This trait is a generalization of `Default::default`. The difference is that some types may
/// provide a constructor with no arguments, but not implement `Default` for some reason. That
/// might be that it's expensive to create, or mutates some global state, or does some IO, etc.
///
/// Every type implementing `Default` gets a `New` instance.
pub trait New {
    fn new() -> Self;
}

impl<T: Default> New for T {
    fn new() -> Self {
        T::default()
    }
}

// =================================================================================================
//                                              Try
// =================================================================================================

/// Just like the unstable `try` block, this macro runs the code inside of it, but in a context
/// where returns return from it instead of the top function, acting like a try-catch in a
/// traditional language. This is done by immediately calling a closure.
///
/// # Example
///
/// ```
/// use functionality::try_scope;
/// let r = try_scope! {
///     let x = Err("helloo")?;
///     Ok(())
/// };
/// assert_eq!(r, Err("helloo"));
/// ```
#[macro_export]
macro_rules! try_scope {
    ($($t:tt)*) => {
        (|| { $($t)* })()
    }
}

// =================================================================================================
//                                             Other
// =================================================================================================

pub mod prelude {
    pub use crate::default;
    pub use crate::IntoExt;
    pub use crate::Mutate;
    pub use crate::Pipe;
    pub use crate::{dyn_fn, dyn_fn_mut};
    pub use crate::try_scope;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_trait() {
        assert_eq!(1.pipe(|x| x + 1).pipe(|x| x * 2), 4,);
    }

    #[test]
    fn test_mutate() {
        assert_eq!(vec![1, 3, 2].mutate(|x| x.sort()).len(), 3,);
    }

    #[test]
    fn test_dyn_fn() {
        let f = dyn_fn!(|x| { x + 1 });
        assert_eq!(f(1), 2);
        assert_eq!(f(2), 3);
    }
}
