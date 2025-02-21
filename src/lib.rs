pub use bitvec::BitArr;

/// A simple sequential id allocator.
///
/// The allocator will allocate new ids sequentially from 0 to u8::MAX. Unlike
/// other bitmap / slab allocators, freed ids will not be reused right away,
/// only when the next id pointer wraps around.
#[macro_export]
macro_rules! sequential_id_alloc {
    ($ty:ident, $output_ty:ident, $max:expr, $arr_ty:ident) => {
        #[derive(Debug)]
pub struct $ty<T = $output_ty, A = $crate::BitArr![for $max - 1, in $arr_ty]> {
            next_ptr: usize,
            bits: A,
            _output_type: std::marker::PhantomData<T>,
        }

        impl<T> Default for $ty<T> {
            fn default() -> Self {
                Self {
                    next_ptr: Default::default(),
                    bits: Default::default(),
                    _output_type: Default::default(),
                }
            }
        }

        impl<T> $ty<T>
        where
            T: Into<usize>,
            T: TryFrom<usize>,
        {
            pub const fn max() -> usize {
                $max
            }

            pub fn alloc(&mut self) -> Option<T> {
                let index = self
                    .bits
                    .into_iter()
                    .enumerate()
                    .cycle()
                    .skip(self.next_ptr)
                    .take($max)
                    .filter_map(|(i, b)| if !b { Some(i) } else { None })
                    .next()?;

                self.bits.set(index, true);
                self.next_ptr = index + 1;

                T::try_from(index).ok()
            }

            pub fn dealloc(&mut self, id: T) {
                let id = id.into();
                self.bits.set(id, false);
            }

            pub fn contains(&self, id: T) -> bool {
                let id = id.into();
                self.bits[id]
            }

            pub fn is_full(&self) -> bool {
                self.bits.count_zeros() == 0
            }

            pub fn size(&self) -> usize {
                self.bits.count_ones()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    sequential_id_alloc!(SequentialIdAllocU8, u8, 256, u8);

    #[test]
    fn test_sequential_alloc_1() {
        let mut ids = SequentialIdAllocU8::default();

        assert!(!ids.contains(0u8));
        assert_eq!(ids.alloc(), Some(0u8));
        assert!(ids.contains(0u8));
        assert_eq!(ids.size(), 1usize);

        ids.dealloc(0);
        assert_eq!(ids.alloc(), Some(1));
        assert_eq!(ids.size(), 1);
    }

    #[test]
    fn test_sequential_alloc_2() {
        let mut ids = SequentialIdAllocU8::default();

        assert_eq!(ids.alloc(), Some(0u8));
        assert_eq!(ids.size(), 1);

        assert_eq!(ids.alloc(), Some(1));
        assert_eq!(ids.size(), 2);

        ids.dealloc(0);
        assert_eq!(ids.alloc(), Some(2));
        assert_eq!(ids.size(), 2);
    }

    #[test]
    fn test_wrap() {
        let mut ids = SequentialIdAllocU8::<u8>::default();
        for _ in 0..SequentialIdAllocU8::<u8>::max() {
            assert!(ids.alloc().is_some());
        }

        assert!(ids.is_full());
        assert!(ids.alloc().is_none());

        ids.dealloc(5);
        assert!(!ids.is_full());
        assert_eq!(ids.alloc(), Some(5));
    }

    #[test]
    fn test_arbitrary_inputs() {
        let alloc = std::cell::RefCell::new(SequentialIdAllocU8::default());
        proptest!(ProptestConfig::with_cases(1000), |(n in 0..u8::MAX, allocate in proptest::bool::weighted(0.8))| {
            let mut alloc = alloc.borrow_mut();
            let size = alloc.size();
            let is_full = alloc.is_full();
            if allocate {
                let n = alloc.alloc();
                match (is_full, n) {
                    (true, None) => {
                        assert_eq!(alloc.size(), size);
                    }
                    (false, Some(n)) => {
                        assert_eq!(alloc.size(), size + 1);
                        assert!(alloc.contains(n));
                    }
                    _ => unreachable!(),
                }
            } else if alloc.contains(n) {
                alloc.dealloc(n);
                assert_eq!(alloc.size(), size - 1);
                assert!(!alloc.contains(n));
                assert!(!alloc.is_full());
            }
        });
    }
}
