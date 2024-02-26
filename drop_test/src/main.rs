use act_zero::runtimes::tokio::spawn_actor;
use act_zero::*;
use tokio::time::Instant;

struct ShortGreeter {
    start: Instant,
    number_of_greets: i32,
}

impl Actor for ShortGreeter {}

impl ShortGreeter {
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

impl Drop for ShortGreeter {
    fn drop(&mut self) {
        println!("Short Greeter Dropped");
    }
}

#[tokio::main]
async fn main() {
    let task = {
        let short_greeter = spawn_actor(ShortGreeter::new());

        let task = {
            let short_greeter = short_greeter.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    call!(short_greeter.short_greet());
                }
            })
        };

        let short_greeter = short_greeter.clone();
        send!(short_greeter.short_greet());
        task
    };

    println!("inside block ddone");
    task.abort();
    println!("task aborted");
    println!("ending");
}
