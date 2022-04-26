// Channel implementation for MagLevCrabs

// STD Imports


// Maglev Imports


// 3rd Party Imports



pub(crate) mod channel
{
    struct raw_channel<T>
    {
        ptr: NonNull(u8),
        cursor: NonNull(u8),
        rear: NonNull(u8),
        senders: u16,
        recievers: u16,
        capacity: usize,
        usage: usize, //Maybe Atomic?
    }
}

pub mod spsc
{

}

pub mod mpmc
{

}

pub mod mpsc
{

}

pub mod spmc
{

}

pub(crate) mod securechannel
{
    
}

#[cfg(test)]
mod tests
{
    #[test]
    fn recurv_channel_flavors_creation()
    {
        /*
        
        */
    }
    
    #[test]
    fn single_message_passing()
    {
        /*
        (tx, rx) = channel::new()
        tx.emit("Test Token");

        assert!(rx.hasMessage());
        */
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

    //Test setup Functions

    fn create_channel()
    {
        //Takes a channel type, sender and recevier Options that defaults to 1 each, size of channel and flavor
    }
}
