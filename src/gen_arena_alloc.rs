#[feature(allocator_api)]
#[allow(dead_code)]

/// A generational allocation manager based off of Catherine West's keynote @ RustConf 2018 and the generational-arena crate
/// Mostly to prove I can, also incase I need to manipulate memory structures, double indirect or make custom fn's


use std::iter::{self, Extend, FromIterator, FusedIterator}; // Reminder to make this iter friendly
use std::ops::{Index, IndexMut};
use std::sync::atomic::{AtomicU64, Ordering};
use std::ptr::NonNull;
use std::marker::PhantomData;


#[derive(Debug)]
struct Arena<T> {
    _sections: NonNull<Section<T>>,   // Pointer to the allocated memory. Only valid when used with the safe impl
    size: usize, // Size of the allocated memory.
    capacity: usize, // Memory left to allocate.
}

#[derive(Debug)]
struct Section<T> {
    inner: *const SectionInner<T>,
    _marker: PhantomData<T>, // Does this mean that the clones of this think they own the Data?
}

#[derive(Debug)]
struct SectionInner<T> {
    ptr: NonNull<Chair<T>>, // The pointer to the start of the section.
    arc: AtomicU64, // The counter for how many referencers are alive.
    size: usize, // Count of how many 'T' are owned by this section
}

#[derive(Debug)]
struct Chair<T: Sized> {
    data: T,
    gen: AtomicU64,
}

impl<T> Index<usize> for Arena<T> {
    type Output = Section<T>;

    fn index(&self, idx: usize) -> &Section<T> {
        Index::index(self, idx)
    }
}

impl<T> Index<usize> for SectionInner<T> {
    type Output = Chair<T>;

    fn index(&self, idx: usize) -> &Self::Output {
        Index::index(self, idx)
    }
}

impl<T> IndexMut<usize> for SectionInner<T> {
    fn index_mut(&mut self, idx:usize) -> &mut Self::Output {
        IndexMut::index_mut(self, idx)
    }
}

impl<T> Clone for Section<T> {
    fn clone(&self) -> Section<T> {
        let atomic = unsafe{ &(*self.inner).arc }; // Deref the pointer, get the inner atomic, and reference it.
        atomic.fetch_add(1, Ordering::AcqRel);
        Section {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

