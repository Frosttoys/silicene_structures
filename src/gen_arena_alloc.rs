#[allow(dead_code)]

/// A generational allocation manager based off of Catherine West's keynote @ RustConf 2018 and the generational-arena crate
/// Mostly to prove I can, also incase I need to manipulate memory structures, double indirect or make custom fn's


use std::iter::{self, Extend, FromIterator, FusedIterator}; // Reminder to make this iter friendly
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug)]
struct Arena<T> {
    sections: Vec<Section<T>>,
}

#[derive(Debug)]
struct Section<T> {
    row: Arc<Vec<Chair<T>>>,
}

#[derive(Debug)]
struct Chair<T> {
    data: T,
    gen: AtomicU64, // The data's current generation.
}

impl<T> Arena<T> {
    fn new(data: T) -> Self {
        Self { sections: vec!(Section::new(data)) }
    }

    fn new_uninit() -> Self {
        Self { sections: Vec::new() }
    }
}

impl<T> Section<T> {
    fn new(data: T) -> Self {
        Self { row: Arc::new(vec!(Chair::new(data))), }
    }

    fn new_uninit() -> Self {
        Self { row: Arc::new(Vec::new()), }
    }

    fn generation(&self) -> (usize, usize) {
        let mut gensplit = (0usize, 0usize);
        for chair in self.row.iter() {
            let cmpr = chair.gen.load(Ordering::Relaxed) as usize;
            if cmpr != gensplit.0 && cmpr != gensplit.1 {
                if gensplit.0 == 0 { gensplit.0 = cmpr; }
                else if gensplit.1 == 0 { gensplit.1 = cmpr; }
                else {unreachable!();} // Really hope this is actually unreachable...
            };
        }
        gensplit
    }

    fn index_generation(&self, idx: usize) -> usize {
        (*self.row)[idx].gen.load(Ordering::Relaxed) as usize
    }
}

impl<T> Chair<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            gen: AtomicU64::new(0), // Starting at gen 0 makes sense I think.
        }
    }

    fn update(&mut self, data: T) {
        self.gen.fetch_add(1, Ordering::AcqRel); // Increment the generation
        self.data = data;
    }
}