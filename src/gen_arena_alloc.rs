#[allow(dead_code)]

/// Multithreaded asynchronously-accessible Vector with atomic generational counter
/// This turned into a Vector wrapper where the data contains it's generation and a `push()` past the end just wraps (like VecDeque)


use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
struct Arena<T> {
    elements: Vec<ArenaInner<T>>, 
    cursor: AtomicUsize, // Writer cursor
}

#[derive(Debug)]
struct ArenaInner<T> {
    data: T,
    gen: u64, // The data's current generation
}

impl<T> Arena<T> {
    pub fn new(data: T) -> Self {
        Self { 
            elements: vec!(ArenaInner::new(data)),
            cursor: AtomicUsize::new(0),
        }
    }

    pub fn new_sized(size: usize) -> Self {
        Self { 
            elements: Vec::with_capacity(size),
            cursor: AtomicUsize::new(0),
        }
    }

    pub fn new_uninit() -> Self {
        Self { 
            elements: Vec::new(),
            cursor: AtomicUsize::new(0),
        }
    }

    pub fn push(&mut self, data: T) -> Result<(), std::io::Error> {
        
    }
    
    fn generation(&self) -> (u64, u64) { // Returns the current generation of the section, usually split between 2 generations.
        let mut gensplit = (0u64, 0u64);

        for chair in self.elements.iter() {
            let cmpr = chair.gen;
            if cmpr != gensplit.0 && cmpr != gensplit.1 {
                if gensplit.0 == 0 { gensplit.0 = cmpr; }
                else if gensplit.1 == 0 { gensplit.1 = cmpr; }
                else { unreachable!(); } // Really hope this is actually unreachable...
            };
        }
        gensplit
    }

    fn index_generation(&self, idx: usize) -> u64 { // Generation of the data at the specific index
        self.elements[idx].gen
    }
}

impl<T> ArenaInner<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            gen: 0, // Starting at gen 0 makes sense I think.
        }
    }

    fn update(&mut self, data: T) { // Could be bumped out of the processor mid-call, need to fix that for this to be "atomic"
        self.gen += 1;
        self.data = data;
    }
}



mod tests {
    // Functions to generate arena's, make random data, and other utility functions.
    mod test_helpers {

    }

    #[cfg(test)]
    mod general_tests {
        #[test]
        fn static_types() { // I didn't model for statics/consts at all so this should expose a lot of problems.

        }
        #[test]
        fn sized_types() { // Primatives/Structs/Enums/Tuples etc.
    
        }
        #[test]
        fn unsized_types() { // Currently broken due to deriving Debug on the structs.
    
        }
        #[test]
        fn generational_test() { // Tests the unreachability of Section::generation
    
        }
    }
    
    #[cfg(test)]
    // Atomic Ordering and Memory Fence testing
    mod ordering_tests {
    
    }
    
    #[cfg(test)]
    // Memory safety violation testing, Trying to break this everyway I know of.
    mod safety_tests {
        
    }
}


