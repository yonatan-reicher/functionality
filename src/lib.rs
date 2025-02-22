pub use pipe_trait::Pipe;

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

pub mod prelude {
    pub use crate::Pipe;
    pub use crate::Mutate;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_trait() {
        assert_eq!(
            1
            .pipe(|x| x + 1)
            .pipe(|x| x * 2),
            4,
        );
    }

    #[test]
    fn test_mutate() {
        assert_eq!(
            vec![1, 3, 2]
                .mutate(|x| x.sort())
                .len(),
            3,
        );
    }
}
