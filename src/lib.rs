use std::num::NonZeroUsize;

pub trait CountNonZeroExt: Iterator {
    fn count_non_zero(self) -> Option<NonZeroUsize>;
}

// Implement the trait for all types that implement Iterator
impl<T> CountNonZeroExt for T
where
    T: Iterator,
{
    fn count_non_zero(self) -> Option<NonZeroUsize> {
        let count = self.count();
        NonZeroUsize::new(count)
    }
}
