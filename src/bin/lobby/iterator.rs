use std::iter::Iterator;

pub trait IteratorExt: Iterator {
    /// Returns the index of the maximum element of an iterator.
    ///
    /// If several elements are equally maximum, the index of the first element
    /// is returned.  If the iterator is empty, `None` is returned.
    fn first_max_position(self) -> Option<usize>
    where
        Self::Item: Ord;
}

impl<I> IteratorExt for I
where
    I: Iterator,
{
    fn first_max_position(self) -> Option<usize>
    where
        Self::Item: Ord,
    {
        self.enumerate()
            .reduce(|(imax, vmax), (icur, vcur)| {
                if vcur > vmax {
                    (icur, vcur)
                } else {
                    (imax, vmax)
                }
            })
            .map(|(imax, _)| imax)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_max_position() {
        assert!(matches!(b"".iter().first_max_position(), None));
        assert_eq!(b"1".iter().first_max_position().unwrap(), 0);
        assert_eq!(b"12".iter().first_max_position().unwrap(), 1);
        assert_eq!(b"21".iter().first_max_position().unwrap(), 0);
        assert_eq!(b"22".iter().first_max_position().unwrap(), 0);
        assert_eq!(b"212".iter().first_max_position().unwrap(), 0);
    }
}
