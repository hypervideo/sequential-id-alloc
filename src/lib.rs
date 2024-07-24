use bitvec::prelude::*;

/// A simple sequential id allocator.
///
/// The allocator will allocate new ids sequentially from 0 to u8::MAX. Unlike
/// other bitmap / slab allocators, freed ids will not be reused right away,
/// only when the next id pointer wraps around.
pub struct SequentialIdAlloc<T = u8> {
    next_ptr: usize,
    bits: BitArr![for 255, in u8],
    _output_type: std::marker::PhantomData<T>,
}

impl<T> Default for SequentialIdAlloc<T> {
    fn default() -> Self {
        Self {
            next_ptr: Default::default(),
            bits: Default::default(),
            _output_type: Default::default(),
        }
    }
}

impl<T> SequentialIdAlloc<T>
where
    T: From<u8> + Into<u8>,
{
    const CAPACITY: usize = u8::MAX as usize + 1;

    pub fn alloc(&mut self) -> Option<T> {
        let index = self
            .bits
            .iter()
            .enumerate()
            .cycle()
            .skip(self.next_ptr)
            .take(Self::CAPACITY)
            .filter_map(|(i, b)| if !b { Some(i) } else { None })
            .next()?;

        self.bits.set(index, true);
        self.next_ptr = index + 1;

        Some(T::from(index as u8))
    }

    pub fn dealloc(&mut self, id: T) {
        let id = id.into() as usize;
        self.bits.set(id, false);
    }

    pub fn contains(&self, id: T) -> bool {
        let id = id.into() as usize;
        self.bits[id]
    }

    pub fn is_full(&self) -> bool {
        self.bits.count_zeros() == 0
    }

    pub fn size(&self) -> usize {
        self.bits.count_ones()
    }

    #[cfg(debug_assertions)]
    pub fn debug(&self) {
        let free = self
            .bits
            .iter()
            .enumerate()
            .cycle()
            .skip(self.next_ptr)
            .take(Self::CAPACITY)
            .map(|(i, b)| (i, *b))
            .collect::<Vec<_>>();
        dbg!(free);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_sequential_alloc_1() {
        let mut ids = SequentialIdAlloc::default();

        assert!(!ids.contains(0));
        assert_eq!(ids.alloc(), Some(0));
        assert!(ids.contains(0));
        assert_eq!(ids.size(), 1);

        ids.dealloc(0);
        assert_eq!(ids.alloc(), Some(1));
        assert_eq!(ids.size(), 1);
    }

    #[test]
    fn test_sequential_alloc_2() {
        let mut ids = SequentialIdAlloc::default();

        assert_eq!(ids.alloc(), Some(0));
        assert_eq!(ids.size(), 1);

        assert_eq!(ids.alloc(), Some(1));
        assert_eq!(ids.size(), 2);

        ids.dealloc(0);
        assert_eq!(ids.alloc(), Some(2));
        assert_eq!(ids.size(), 2);
    }

    #[test]
    fn test_wrap() {
        let mut ids = SequentialIdAlloc::<u8>::default();
        for _ in 0..SequentialIdAlloc::<u8>::CAPACITY {
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
        let alloc = std::cell::RefCell::new(SequentialIdAlloc::default());
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
