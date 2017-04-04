pub struct Network {
    pub x_ratio: f32,
    pub y_ratio: f32
}

impl Network {
    pub fn new() -> Network {
        Network {
            x_ratio: 0.0,
            y_ratio: 0.0
        }
    }

    pub fn server(&mut self) {
        use std::thread;

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

                            println!("Received: {}, {}", f1, f2);
                        }
                    },
                    Err(_) => println!("Error while connecting!"),
                }
            }
        });
    }
}