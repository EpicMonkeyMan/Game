use std::sync::{Arc, Mutex};

pub struct Network {
    pub x_ratio: Arc<Mutex<f32>>,
    pub y_ratio: Arc<Mutex<f32>>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            x_ratio: Arc::new(Mutex::new(0.0)),
            y_ratio: Arc::new(Mutex::new(0.0)),
        }
    }

    pub fn server(&mut self) {
        use std::thread;

        let x = self.x_ratio.clone();
        let y = self.y_ratio.clone();
        thread::spawn(move || {
            use std::net::TcpListener;
            use std::io::Read;

            let server = TcpListener::bind("127.0.0.1:8080").unwrap();

            println!("Waiting for clients...");

            'cloop: for client in server.incoming() {
                match client {
                    Ok(mut c) => {
                        use std::mem;

                        println!("New client: {:?}", c);

                        loop {
                            let mut buffer1: [u8; 4] = [0; 4];
                            let mut buffer2: [u8; 4] = [0; 4];

                            if c.read(&mut buffer1).is_err() || c.read(&mut buffer2).is_err() {
                                println!("Client disconnected!");
                                continue 'cloop;
                            }

                            let f1: f32 = unsafe {mem::transmute(buffer1)};
                            let f2: f32 = unsafe {mem::transmute(buffer2)};

                            {
                                let mut x = x.lock().unwrap();
                                let mut y = y.lock().unwrap();
                                *x = f1;
                                *y = f2;
                            }
                            println!("Received: {}, {}", f1, f2);
                        }
                    },
                    Err(_) => println!("Error while connecting!"),
                }
            }
        });
    }
}