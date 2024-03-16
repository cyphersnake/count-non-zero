use std::num::NonZeroUsize;

/// An extension trait for iterators to count non-zero elements.
///
/// This trait provides an additional method to iterators, `count_non_zero`,
/// which returns an `Option<NonZeroUsize>` representing the number of elements
/// iterated over that are not zero. This method leverages Rust's type system
/// to provide a guarantee at compile time that the count is non-zero when
/// Some value is returned.
pub trait CountNonZeroExt: Iterator {
    /// Counts the number of non-zero elements the iterator iterates over.
    ///
    /// # Returns
    ///
    /// - `Some(NonZeroUsize)` if the iterator contains non-zero elements, with
    /// the count of those elements.
    /// - `None` if the iterator is empty or only contains zeros.
    ///
    /// # Examples
    ///
    /// ```
    /// use count_non_zero::CountNonZeroExt;
    /// let data = vec![0, 1, 2, 0, 3];
    /// let count = data.into_iter().count_non_zero();
    /// assert_eq!(count, Some(std::num::NonZeroUsize::new(5).unwrap()));
    /// ```
    fn count_non_zero(self) -> Option<NonZeroUsize>;
}

impl<T> CountNonZeroExt for T
where
    T: Iterator,
{
    fn count_non_zero(self) -> Option<NonZeroUsize> {
        let count = self.count();
        NonZeroUsize::new(count)
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use super::*;

    #[test]
    fn empty_iterator() {
        let data: Vec<u32> = vec![];
        let count = data.into_iter().count_non_zero();
        assert_eq!(count, None);
    }

    #[test]
    fn all_non_zeros_iterator() {
        let data = vec![1, 2, 3];
        let count = data.into_iter().count_non_zero();
        assert_eq!(count, Some(NonZeroUsize::new(3).unwrap()));
    }
}
