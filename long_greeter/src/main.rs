use std::io;
use std::time::Duration;

use act_zero::runtimes::tokio::spawn_actor;
use act_zero::*;
use tokio::net::TcpListener;
use tokio::time::Instant;

struct LongGreeter {
    start: Instant,
    number_of_greets: i32,
}

impl Actor for LongGreeter {}

impl LongGreeter {
    async fn long_greet(&mut self) -> ActorResult<()> {
        self.number_of_greets += 1;

        println!(
            "Long Greeter: Number {}, Since Start: {}ms",
            self.number_of_greets,
            self.start.elapsed().as_millis()
        );

        // sleep here represents a longer await, for example a HTTP call to a slow service
        tokio::time::sleep(Duration::from_millis(1000)).await;

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
    let long_listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    let long_greeter = spawn_actor(LongGreeter::new());

    loop {
        let _ = long_listener.accept().await;
        call!(long_greeter.long_greet());
    }
}
