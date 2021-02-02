use std::io;
use std::time::Duration;

use act_zero::runtimes::tokio::spawn_actor;
use act_zero::*;
use tokio::net::TcpListener;
use tokio::time::Instant;

struct SimpleGreeter {
    start: Instant,
    number_of_greets: i32,
}

impl Actor for SimpleGreeter {}

impl SimpleGreeter {
    async fn short_greet(&mut self) -> ActorResult<()> {
        self.number_of_greets += 1;

        println!(
            "Short Greeter: Number {}, Since Start: {}ms",
            self.number_of_greets,
            self.start.elapsed().as_millis()
        );

        Produces::ok(())
    }

    fn new() -> Self {
        Self {
            start: Instant::now(),
            number_of_greets: 0,
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let short_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let short_greeter = spawn_actor(SimpleGreeter::new());

    loop {
        let _ = short_listener.accept().await;
        call!(short_greeter.short_greet());
    }
}
