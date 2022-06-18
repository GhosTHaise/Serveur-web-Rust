use std::thread;
pub struct ThreadPool{
    threads : Vec<thread::JoinHandle<()>>
}

impl ThreadPool {
    //Creer un ThreadPool
    //
    //Size : nombre de thread
    //
    //la nouvel fonction Panic si thread < 0
    //
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {

        }
        ThreadPool {threads}
    }

    pub fn execute<F>(&self,f: F)
        where
            F: FnOnce() + Send + 'static
                {

                }
}