


/// Cache Balanced Container
/// Creates a "container" type class like a `Box` with allocation and deallocation support.
/// More Concretely: This creates a container of memory aligned by 64 bytes to ensure cache friendliness, also to reduce memory allocation, and finally to increase locality without thread thrashing, each block inside the CBC is also cacheline sized and unshared across threads.


mod CacheBalancedAllocator
{
    
}