use local_thread::LocalThread;

fn main() {
    let vec: Vec<i32> = (0..100).into_iter().collect();

    let mut thread = LocalThread::new(|| {
        // Nested loop.
        // Local vec reference being used inside thread
        for &i in &vec {
            println!("nested: {i}");
        }
    });

    // Handle will complete the thread when dropped. We must drop handle at the end of the
    // main func to not force the LocalThread to be completed before the primary loop starts.
    let _handle = thread.spawn();

    // Primary loop.
    // Same vec reference being used outside thread
    for &i in &vec {
        println!("primary: {i}");
    }
}
