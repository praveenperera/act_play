use std::collections::VecDeque;
use std::io;
use std::time::Duration;

use act_zero::runtimes::tokio::spawn_actor;
use act_zero::*;
use tokio::net::TcpListener;
use tokio::time::Instant;

struct GreeterManager {
    start: Instant,
    number_of_greets: i32,
    greeters: VecDeque<Addr<LongGreeter>>,
}

impl Actor for GreeterManager {}

impl GreeterManager {
    async fn greet(&mut self) -> ActorResult<()> {
        self.number_of_greets += 1;
        let greeter = self.greeter();

        send!(greeter.long_greet(self.number_of_greets, self.start.clone()));

        Produces::ok(())
    }

    fn new() -> Self {
        let mut greeters = VecDeque::with_capacity(5);

        for _i in 0..5 {
            greeters.push_front(spawn_actor(LongGreeter::new()));
        }

        Self {
            start: Instant::now(),
            number_of_greets: 0,
            greeters,
        }
    }

    fn greeter(&mut self) -> WeakAddr<LongGreeter> {
        let greeter_pid = self
            .greeters
            .pop_front()
            .expect("Should always be available");

        self.greeters.push_back(greeter_pid.clone());

        greeter_pid.downgrade()
    }
}

struct LongGreeter {}

impl Actor for LongGreeter {}

impl LongGreeter {
    async fn long_greet(&self, number_of_greets: i32, start: Instant) -> ActorResult<()> {
        println!(
            "GreeterPool/Long Greeter: Number {}, Since Start: {}ms",
            number_of_greets,
            start.elapsed().as_millis()
        );

        // sleep here represents a longer await, for example a HTTP call to a slow service
        tokio::time::sleep(Duration::from_millis(1000)).await;

        Produces::ok(())
    }

    fn new() -> Self {
        Self {}
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8082").await.unwrap();
    let greeter = spawn_actor(GreeterManager::new());

    loop {
        let _ = listener.accept().await;
        call!(greeter.greet());
    }
}
