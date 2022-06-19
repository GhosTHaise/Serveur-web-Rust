use std::{sync::{mpsc,Arc,Mutex},thread};

pub struct ThreadPool{
    workers : Vec<Worker>,
    sender : mpsc::Sender<Job>
}
struct Job;
impl ThreadPool {
    //Creer un ThreadPool
    //
    //Size : nombre de thread
    //
    //la nouvel fonction Panic si thread < 0
    //
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        //Creeons une channe;
        let (sender,receiver) = mpsc::channel();
        //redefinissons moved receiver 
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }  

        ThreadPool {workers,sender}
    }

    pub fn execute<F>(&self,f: F)
        where
            F: FnOnce() + Send + 'static
                {

                }
}
struct Worker {
    id : usize,
    thread : thread::JoinHandle<()>
}

impl Worker {
    fn new(id : usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(||{});
        Worker{id,thread}
    }
}