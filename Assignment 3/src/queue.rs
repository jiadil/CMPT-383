use std::sync::mpsc;
use std::thread;

pub trait Task {
    type Output: Send;
    fn run(&self) -> Option<Self::Output>;
}

pub struct WorkQueue<TaskType: 'static + Task + Send> {
    send_tasks: Option<spmc::Sender<TaskType>>, // Option because it will be set to None to close the queue
    recv_tasks: spmc::Receiver<TaskType>,
    //send_output: mpsc::Sender<TaskType::Output>, // not need in the struct: each worker will have its own clone.
    recv_output: mpsc::Receiver<TaskType::Output>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl<TaskType: 'static + Task + Send> WorkQueue<TaskType> {
    pub fn new(n_workers: usize) -> WorkQueue<TaskType> {
        // TODO: create the channels; start the worker threads; record their JoinHandles
        let (sender_tasks, receiver_tasks) = spmc::channel();
        let (sender_output, receiver_output) = mpsc::channel();

        let mut workers = Vec::new();

        for _ in 0..n_workers {
            let snd = sender_output.clone();
            let rcv = receiver_tasks.clone();
            // create a new thread and push it to the vector
            workers.push(thread::spawn(move || {
                WorkQueue::<TaskType>::run(rcv, snd);
            }));
        }

        WorkQueue {
            send_tasks: Some(sender_tasks),
            recv_tasks: receiver_tasks,
            recv_output: receiver_output,
            workers: workers,
        }
    }

    fn run(recv_tasks: spmc::Receiver<TaskType>, send_output: mpsc::Sender<TaskType::Output>) {
        // TODO: the main logic for a worker thread
        loop {
            let task_result = recv_tasks.recv();
            // task_result will be Err() if the spmc::Sender has been destroyed and no more messages can be received here
            match task_result {
                Ok(task) => {
                    let result = task.run();
                    match result {
                        Some(output) => {
                            send_output.send(output).unwrap();
                        }
                        None => {}
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    pub fn enqueue(&mut self, t: TaskType) -> Result<(), spmc::SendError<TaskType>> {
        // TODO: send this task to a worker
        match self.send_tasks {
            Some(ref mut sender) => {
                return sender.send(t);
            }
            None => {
                return Err(spmc::SendError(t));
            }
        }
    }

    // Helper methods that let you receive results in various ways
    pub fn iter(&mut self) -> mpsc::Iter<TaskType::Output> {
        self.recv_output.iter()
    }
    pub fn recv(&mut self) -> TaskType::Output {
        self.recv_output
            .recv()
            .expect("I have been shutdown incorrectly")
    }
    pub fn try_recv(&mut self) -> Result<TaskType::Output, mpsc::TryRecvError> {
        self.recv_output.try_recv()
    }
    pub fn recv_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> Result<TaskType::Output, mpsc::RecvTimeoutError> {
        self.recv_output.recv_timeout(timeout)
    }

    pub fn shutdown(&mut self) {
        // TODO: Destroy the spmc::Sender so everybody knows no more tasks are incoming;
        // drain any pending tasks in the queue; wait for each worker thread to finish.
        // HINT: Vec.drain(..)
        self.send_tasks = None;
        loop {
            match self.recv_tasks.recv() {
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }
        }

        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

impl<TaskType: 'static + Task + Send> Drop for WorkQueue<TaskType> {
    fn drop(&mut self) {
        // "Finalisation in destructors" pattern: https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html
        match self.send_tasks {
            None => {} // already shut down
            Some(_) => self.shutdown(),
        }
    }
}
