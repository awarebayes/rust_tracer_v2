use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crossbeam_channel::bounded;
use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rand;
use rand::SeedableRng;

type Pixel = [u8; 4];
type ResultMessage = Result<PlacedPixel, PlacedPixelErr>;

pub struct PlacedPixel {
    pub i: usize,
    pub j: usize,
    pub color: Pixel,
}

pub struct RTThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
    receiver: Option<Receiver<ResultMessage>>,
    collect_handle: Option<thread::JoinHandle<()>>,
    bar: Arc<ProgressBar>,
    pub end_image: Arc<Mutex<Vec<[u8; 4]>>>,
    height: usize,
    width: usize,
}

type JobFn = dyn FnMut(Arc<Mutex<rand::rngs::StdRng>>) -> ResultMessage + Send + 'static;
type Job = Box<JobFn>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct PlacedPixelErr {
    i: usize,
    j: usize,
}

impl RTThreadPool {
    pub fn new(size: usize, width: usize, height: usize) -> RTThreadPool {
        assert!(size > 0);
        let (thread_sender, my_receiver) = unbounded();
        let (my_sender, thread_receiver) = bounded(size);
        let thread_receiver = Arc::new(Mutex::new(thread_receiver));
        let thread_sender = Arc::new(thread_sender);
        let mut workers = Vec::with_capacity(size);
        let bar = Arc::new(ProgressBar::new(height as u64));
        bar.set_style(
            ProgressStyle::default_bar().template("[{elapsed}|{eta}] {bar:60} {pos}/{len}"),
        );

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&thread_receiver),
                Arc::clone(&thread_sender),
            ));
        }

        let end_image = Arc::new(Mutex::new(vec![[0, 0, 0, 255]; width * height]));

        RTThreadPool {
            workers,
            sender: my_sender,
            receiver: Some(my_receiver),
            end_image,
            width,
            height,
            collect_handle: None,
            bar,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnMut(Arc<Mutex<rand::rngs::StdRng>>) -> ResultMessage + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }

    pub fn start_collecting(&mut self) {
        let receiver_handle = self.receiver.take().unwrap();
        let image_ref = Arc::clone(&self.end_image);
        let height = self.height;
        let width = self.width;
        let total_pixels = width * height;
        let bar_ref = Arc::clone(&self.bar);

        self.collect_handle = Some(thread::spawn(move || {
            let mut pixels_placed = 0;
            while pixels_placed < total_pixels {
                let message = receiver_handle.recv().unwrap();
                match message {
                    Ok(pixel) => {
                        let offset = (height - 1 - pixel.j) * width + pixel.i;
                        image_ref.lock().unwrap()[offset] = pixel.color;
                    }
                    Err(e) => {
                        println!("Failed to shoot ray at {} {}", e.i, e.j);
                    }
                }
                pixels_placed += 1;
                if pixels_placed % width == 0 {
                    bar_ref.inc(1);
                }
            }
        }));
    }

    pub fn collect(&mut self) {
        self.collect_handle.take().unwrap().join().unwrap();
        self.bar.finish();
    }
}

impl Drop for RTThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down workers!");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<Message>>>,
        sender: Arc<Sender<ResultMessage>>,
    ) -> Worker {
        let rng = Arc::new(Mutex::new(rand::rngs::StdRng::from_entropy()));
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(mut job) => {
                    let result = job(Arc::clone(&rng));
                    sender.send(result).unwrap();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
