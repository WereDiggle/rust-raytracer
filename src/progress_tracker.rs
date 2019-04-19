use std::sync::{mpsc};
use super::ImageDimension;
use std::thread;
use std::time::{Instant};

pub struct ProgressTracker {
    image_dimension: ImageDimension,
    sender: mpsc::Sender<ProgressMessage>,
    thread: Option<thread::JoinHandle<()>>,
}

// TODO: separate struct for storing progress info
struct ProgressInfo {
    total: u32,
    progress: u32,
}

impl ProgressInfo {
    pub fn new(total: u32) -> ProgressInfo {
        ProgressInfo { total, progress: 0 }
    }

    pub fn add_progress(&mut self, progress: u32) {
        self.progress += progress;
    }

    pub fn reset(&mut self, total: u32) {
        self.progress = 0;
        self.total = total;
    }

    pub fn get_percentage(&self) -> String {
        format!("{:.*}%", 6, 100.0 * self.progress as f64 / self.total as f64)
    }
}

pub enum ProgressMessage {
    Progress(u32),
    StartAA(u32),
    AAProgress(u32),
    Terminate,    
}

fn printr(string: &str) {
    print!("{}\r", string);
}

impl ProgressTracker {
    pub fn new(image_dimension: ImageDimension) -> ProgressTracker {
        let (sender, receiver) = mpsc::channel::<ProgressMessage>();

        let thread = thread::spawn(move || {
            let mut info = ProgressInfo::new(image_dimension.area());
            let start_time = Instant::now();
            println!("START RENDERING");
            loop {
                let message = receiver.recv().unwrap();
                match message {
                    ProgressMessage::Progress(pixels) => {
                        info.add_progress(pixels);
                        printr(&info.get_percentage());
                    },
                    ProgressMessage::StartAA(total) => {
                        info.reset(total);
                        println!("START ANTIALIASING");
                    },
                    ProgressMessage::AAProgress(pixels) => {
                        info.add_progress(pixels);
                        printr(&info.get_percentage());
                    },
                    ProgressMessage::Terminate => {
                        break;
                    },
                }
            }
            println!("Elapsed Time: {} seconds", start_time.elapsed().as_secs());
        });

        ProgressTracker {
            image_dimension,
            sender,
            thread: Some(thread),
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<ProgressMessage> {
        self.sender.clone()
    }
}

impl Drop for ProgressTracker {
    fn drop(&mut self) {
        self.sender.send(ProgressMessage::Terminate).unwrap();

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}