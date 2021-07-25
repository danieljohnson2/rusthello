/// A extension to privde take_up_to()
pub trait IterExt<T> {
    /// This yields each item of the input until one passes the predicate;
    /// it then returns that item, and stops afterwards.
    fn take_up_to<P>(self, pred: P) -> TakeUpTo<Self, P>
    where
        P: FnMut(&T) -> bool,
        Self: Sized;
}

/// Holds the state needed for take_up_to.
pub struct TakeUpTo<I, P> {
    iterator: I,
    pred: P,
    done: bool,
}

impl<I, T> IterExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn take_up_to<P>(self, pred: P) -> TakeUpTo<Self, P>
    where
        P: FnMut(&T) -> bool,
    {
        TakeUpTo {
            iterator: self,
            pred,
            done: false,
        }
    }
}

impl<I, P> Iterator for TakeUpTo<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else if let Some(item) = self.iterator.next() {
            self.done = (self.pred)(&item);
            Some(item)
        } else {
            None
        }
    }
}
