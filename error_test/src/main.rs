
use act_zero::runtimes::tokio::spawn_actor;
use act_zero::*;

struct ErrorTest {
    addr: Addr<Self>
}

#[async_trait::async_trait]
impl Actor for ErrorTest {
    async fn started(&mut self, addr: Addr<Self>) -> ActorResult<()> {
        self.addr = addr;
        Produces::ok(())
    }

    async fn error(&mut self, error: ActorError) -> bool {
        println!("Got error: {:?}", error);

        send!(self.addr.while_dieing());

        // do not stop on actor error
        false
    }
}


impl ErrorTest {
    async fn while_dieing(&mut self) {
        println!("I am dying arghhhhhh")
    }


    async fn die(&mut self) -> ActorResult<()> {
        let number_str = "hfdksfh";
        let _number = number_str.parse::<i32>()?;

        Produces::ok(())
    }

    fn new() -> Self {
        Self { addr: Addr::detached()}
    }
}

#[tokio::main]
async fn main() {
    let error_test = spawn_actor(ErrorTest::new());
    send!(error_test.die());
}
