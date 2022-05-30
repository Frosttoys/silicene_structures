#[feature(allocator_api)]
#[allow(dead_code)]

/// A generational allocation manager based off of Catherine West's keynote @ RustConf 2018 and the generational-arena crate
/// Mostly to prove I can, also incase I need to manipulate memory structures, double indirect or make custom fn's


use std::iter::{self, Extend, FromIterator, FusedIterator};
use std::sync::atomic::{AtomicU64};
use std::marker::PhantomData;
use std::cell::UnsafeCell;


#[derive(Debug)]
struct Arena<T> {
    sections: Vec<Section<T>>,   // Sections inside the arena that it manages.
}

#[derive(Debug)]
struct Section<T> {
    inner: SectionInner<T>,
    _marker: PhantomData<T>,
}

#[derive(Debug)]
struct SectionInner<T> {
    rc: AtomicU64, // The generation number.
    rows: Vec<Chair<T>>, // Holds "Rows" of "Chairs" inside the "Section", get it? "Arena" Allocator, ok.
}

#[derive(Debug)]
// Due to it implementing `!Sync` UnsafeCell may cause problems here, If it does, `impl Chair` needs to emulate the `get , get_mut , and into_inner` functions from UnsafeCell
struct Chair<T> (UnsafeCell<T>, AtomicU64); // Gives the interior mutability required for data to be modified after allocation, also give the generation number.

impl<T> SectionInner<T> {
    fn new() -> Self { 
        Self {
            rc:  AtomicU64::new(0), // The number of threads attached to the arena.
            rows: Vec::new(), // Can't use an Arc due to its shared access constraints.
        }
    }
}

impl<T> Chair<T> {
    /// Creates a new Chair so data can be modified after allocation using interior mutability.
    pub(crate) fn new(this: T) -> Self { Chair(UnsafeCell::new(this), AtomicU64::new(0)) }

    /// Unwraps the cell and returns the owned value
    pub(crate) fn get(self) -> T { self.0.into_inner() }

    /// Returns a mutable pointer to the value behind the UnsafeCell
    pub(crate) fn get_mut_unsafe(&self) -> *mut T { self.0.get() }

    /// Returns an exclusive mutable reference to the value behind the UnsafeCell
    /// Uses a mutable reference to help ensure exclusivity
    pub(crate) fn get_mut(&mut self) -> &mut T { self.0.get_mut() }
}

