#![allow(dead_code)]
/* Cache Lines are 64 Bytes most systems now, Intel x64 chips pull 2 cachelines per call (128 bytes)
   DWCAS Backing, CAS Backing and LL/SC Backing's as seperate traits/structs?
   ARM has weird CAS rules that may break stuff. DW LL/SC (from architecture ARM6k onwards)
/// One LL/SC pair active per processor at any one time, uses an Exclusive Reservation Granule from 8 to 2048 bytes, but also
/// has the concept of a per-processor (local) monitor and a global (system-wide) monitor. If the LL/SC is on a cache-line 
/// which is exclusive, only the local monitor is invoked, and the local monitor can be very weak indeed - it may even only 
/// keep track of whether or not a second LL has been issued since the original! https://liblfds.org/mediawiki/index.php?title=Article:CAS_and_LL/SC_Implementation_Details_by_Processor_family
*/


mod allocators
{
    /// Allocator object that holds a pool of memory and gives portions out, What else would an allocator do?
    /// Explicitly bound to only one Type at a time per APAllocator instance.
    struct APAllocator<T>
    {
        size: usize,
        sections: Vec<APASection<T>>,
    }

    /// section of memory allocated by the APAllocator.
    struct APASection<T>
    {
        size: usize,
        flags: APAllocFlags,
        // Telling the compiler that this struct houses MANY T's, needed for quite a few reasons. See the Docs/Nomicon on PhantomData
        pd: std::marker::PhantomData<T>
    }

    /// Byte flags that can be modified from either side, needs syncronization to be safe.
    struct APAllocFlags(u8);

    impl<T> APAllocator<T>
    {
        /// Generates a new arena for the allocator to use. This is the only public function available to ensure everything else requires you to go through the instance, thus giving us the guarentee that APAllocator is always valid and initalized when calling other methods such as the actual allocation of memory to writer threads via section system.
        pub fn new(layout: std::alloc::Layout) -> std::pin::Pin<Self>
        {
            unimplemented!();
        }

        /// Creates a new section(block, slab, bucket, ect) for use by a writer thread, this memory acts as an ArrayDeque, not supporting reallocation or growth methods but wrapping it's index to act like always-fresh-unbounded-data storage area.
        /// PERF: This would be a good spot for a "restructuring" algorithm that would condense allocated sections into power of 2 alignment sections with NO INSTANCES of a contiguous section of memory being longer that cache-line-size, and can use padding to improve if PERF determines it helps
        fn new_section(&mut self, layout: std::alloc::Layout) -> bool
        {
            unimplemented!();
        }

        /// The same as above except this is in a special spot in the allocator that does not get released back to the Allocator on cycle restart, thus freeing up the allocations, not that it should matter much since it's a best fit algorithm and I kept the previous cycles allocations to help with assignment, 
        fn new_static_section(&mut self, layout: std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>, AllocError>
        {
            unimplemented!();
        }

        /// Step on it
        fn destroy_section(&mut self)
        {
            unimplemented!();
        }
    }

    impl<T> std::ops::Drop for APAllocator<T>
    {
        fn drop(&mut self)
        {
            // TODO: This needs an ARC like drop method due to there being multiple threads accessing the memory at any given time.
            unimplemented!();
        }
    }

    impl<T> APASection<T>
    {
        pub(super) fn new(size: usize) -> Self
        {
            Self
            {
                size: size,
                flags: APAllocFlags::new(),
                pd: std::marker::PhantomData<T>,
            }
        }
    }



    /*
    Bit flags per block (Split only for readability)
    b0000_0000 - No Flags (Uninitalized Block)
    b0000_0001 - Mem In Use
    b0000_0010 - Read-Locked
    b0000_0100 - Wait
    b0000_1000 - Awaiting Waker
    b0001_0000 - Completed
    b0010_0000 - Sticky
    b0100_0000 - Reserved
    b1000_0000 - Reserved
    */
    impl APAllocFlags
    {
        const MEM_IN_USE: u8    = 0b0000_0001;
        const READ_ONLY: u8     = 0b0000_0010;
        const WAITING: u8       = 0b0000_0100;
        const WAKER: u8         = 0b0000_1000;
        const COMPLETED: u8     = 0b0001_0000;
        const STICKY: u8        = 0b0010_0000;

        fn new() -> APAllocFlags
        {
            Self(APAllocFlags::MEM_IN_USE)
        }

        /// Returns a list of all the flags currently of the called evokation type, so all the flags that are true will be returned if passed in `evokation = true`, similarly if set to false, it shows all false flags.
        fn flags(&self, evokation: bool) -> u8
        {
            unimplemented!();
        }

        /// Tries to set the READ_ONLY flag, which it can only do if no Waiting or Waker Flags (AKA in a "First Run State"), Also due to it not relying on the COMPLETED flag, this should just bulldoze over any sort of wonky state.
        /// No writes are allowed to start during this, and Waiting/Waker Flags are always changed to Completed when writes are finished so its easy to check for safety.
        /// If a Writer attempts to write while the READ_ONLY flag is set, it will fail and drop the writes.
        /// TODO: Could use the spare two bits to set the failure type to either: dropping the writes or sleeping until writable again.
        // This should be the only function inside APAllocFlags that has a chance for contention, everything else has very strictly set rules about who wins at what time.
        // SAFETY: Using a &mut ref here to sneakily force the borrow system into enforcing its the only possible user of the APAllocFlags at this time
        fn make_ReadOnly(&mut self) -> bool
        {
            unimplemented!();
        }

        /// Sets the flags to signify the start of a write cycle, or fails back to the caller
        // SAFETY: Using a &mut ref here to sneakily force the borrow system into enforcing its the only possible user of the APAllocFlags at this time
        fn start_cycle(&mut self) -> Result<Ok, Err>
        {
            unimplemented!();
        }

        /// Sets the flag to signify this thread is waiting on something else for a response and is in a "dead-state"
        /// Something sitting in this forever is obviously a deadlock, so avoiding that scenario is rather important.
        // TODO: Decide on a waker format, could return a reference to self that it uses to wake the thread or could use FnOnce Closures to wake the thread, or something else not thought of. Also needs a method of setting this back to false via the Waker or waking operation
        fn set_Waker(&mut self)
        {
            unimplemented!();
        }

        /// Sets the STICKY flag that tells the allocator to recycle the memory per arena cycle or not.
        /// STICKY memory should be allocated to a portion of the arena reserved for unmoving memory, I think theres compiler optimizations for data structured this way as well as its easier to handle (from a design/effort standpoint)
        fn set_Sticky(&mut self)
        {
            unimplemented!();
        }

        /// Sets the Failure mode to either drop the value on failure or wait for the function to be available again
        fn set_Failure_Mode(&mut self)
        {
            unimplemented!();
        }
    }
}

fn main()
{

}

#[cfg(test)]
mod tests
{
    use test_helpers::*;
    mod test_helpers 
    {

    }
    // Need to make a Safety Argument that can run in test/debug and is optimized out on release builds, to verify thread-safety.
    
    #[test]
    fn single_message_passing()
    {

    }

    #[test]
    #[ignore]
    fn flood_recv()
    {

    }

    #[test]
    #[ignore]
    fn starve_recv()
    {

    }

    #[test]
    fn drop_recv()
    {

    }

    #[test]
    fn drop_sender()
    {

    }

    #[test]
    fn poison_resilience()
    {

    }

    #[test]
    fn index_overflow()
    {

    }

    #[test]
    #[ignore]
    fn seqcst_test()
    {

    }

    #[test]
    #[ignore]
    fn acqrel_test()
    {

    }

    #[test]
    #[ignore]
    fn relaxed_test()
    {

    }

    #[test]
    #[ignore]
    fn loom_tests()
    {

    }

    #[test]
    fn allocation_succeeded()
    {
        
    }

    #[test]
    #[ignore]
    fn force_allocation_failure()
    {

    }

    #[test]
    fn APAllocator_was_zeroed()
    {

    }

    ///Overflow by capacity + 1 byte in a single object
    #[test]
    #[ignore]
    fn APAllocator_overflow_single()
    {

    }

    ///Page-size objects are used to overflow the memory pool.
    #[test]
    #[ignore]
    fn APAllocator_overflow_large()
    {

    }

    ///Small objects are used to overflow the memory pool, Can take a REALLY long time.
    #[test]
    #[ignore]
    fn APAllocator_overflow_small()
    {

    }

    /// A reminder to look out for data-races
    #[test]
    #[ignore]
    fn APAllocator_multiwrite_race()
    {

    }

    /// Try to read while a write is being processed
    #[test]
    #[ignore]
    fn APAllocator_rw_race()
    {

    }

    /// See if flip-flopping between reading and writing and reading ultra fast will cause a data race
    #[test]
    #[ignore]
    fn APAllocator_flip_flop_race()
    {

    }
}