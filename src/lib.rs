use std::{sync::{mpsc,Arc,Mutex},thread};

pub struct ThreadPool{
    workers : Vec<Worker>,
    sender : mpsc::Sender<Job>
}
type Job = Box<dyn FnOnce() + Send + 'static>;
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
                    let job = Box::new(f);
                    self.sender.send(job).unwrap();
                }
}
struct Worker {
    id : usize,
    thread : thread::JoinHandle<()>
}

impl Worker {
    fn new(id : usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop{
             let job = receiver.lock().unwrap().recv().unwrap();
             println!("Worker {} got a job ; executing ...",id);
        });
       

        Worker{id,thread}
    }
}