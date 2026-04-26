use std::cell::Ref;

/// An iterator adapter for iterating over the contents of a `Ref` which holds an iterable type.
pub struct RefIter<'a, T, Iter, F> {
    #[allow(unused)]
    ref_: Ref<'a, T>,
    iter: Iter,
    mapping: F,
}

impl<'a, T, Iter, F, U> RefIter<'a, T, Iter, F>
where
    for<'b> &'a T: IntoIterator<IntoIter = Iter>,
    Iter: Iterator,
    F: FnMut(Iter::Item) -> U,
{
    pub fn new(ref_: Ref<'a, T>, mapping: F) -> Self {
        // While `Ref` itself only let's you access temporary references to it's inside, the
        // reference itself is valid for as long as the `Ref` is alive, so we can safely create a
        // reference, elongate it, and use it to create an iterator, as long as the iterator does
        // not outlive the `Ref` itself, and the data returned by the iterator does not reference
        // the `Ref` itself.
        let evil_reference: &'a T = unsafe { &*(&*ref_ as *const T) };
        let iter = evil_reference.into_iter();
        Self {
            ref_,
            iter,
            mapping,
        }
    }
}

impl<'a, T, Iter, F, U> Iterator for RefIter<'a, T, Iter, F>
where
    for<'b> &'a T: IntoIterator<IntoIter = Iter>,
    Iter: Iterator,
    F: FnMut(Iter::Item) -> U,
    // The items returned must be 'static so they do not reference the data inside the Ref, as it
    // might be dropped before the them.
    U: 'static,
{
    type Item = U;

    fn next(&mut self) -> Option<U> {
        self.iter.next().map(&mut self.mapping)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_ref_iter() {
        let x = RefCell::new(vec![1, 2, 3]);
        let ref_iter = RefIter::new(x.borrow(), |x| *x);
        let collected: Vec<_> = ref_iter.collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }
}
