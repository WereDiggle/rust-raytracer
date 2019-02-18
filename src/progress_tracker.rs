use std::sync::{mpsc};
use super::ImageDimension;
use std::thread;

pub struct ProgressTracker {
    image_dimension: ImageDimension,
    sender: mpsc::Sender<ProgressMessage>,
    thread: Option<thread::JoinHandle<()>>,
}

// TODO: separate struct for storing progress info

pub enum ProgressMessage {
    Progress(u32),
    StartAA(u32),
    AAProgress(u32),
    Terminate,    
}

impl ProgressTracker {
    pub fn new(image_dimension: ImageDimension) -> ProgressTracker {
        let (sender, receiver) = mpsc::channel::<ProgressMessage>();

        let thread = thread::spawn(move || {
            let mut pixels_completed = 0;
            let mut total_aa_pixels = 0;
            let mut aa_pixels_completed = 0;
            loop {
                let message = receiver.recv().unwrap();
                match message {
                    ProgressMessage::Progress(pixels) => {
                        pixels_completed += pixels;
                        println!("{}/{}", pixels_completed, image_dimension.area());
                    },
                    ProgressMessage::StartAA(total) => {
                        total_aa_pixels = total;
                    },
                    ProgressMessage::AAProgress(pixels) => {
                        aa_pixels_completed += pixels;
                        println!("{}/{}", aa_pixels_completed, total_aa_pixels);
                    },
                    ProgressMessage::Terminate => {
                        break;
                    },
                }
            }
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