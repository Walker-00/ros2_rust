use std::error::Error;
use std::time::Duration;

use futures::{select, FutureExt};
use futures::{stream::StreamExt, executor::LocalPool};
use futures::task::LocalSpawnExt;
use r2r::{log_warn, log_info};
use r2r::{example_interfaces::srv::AddTwoInts, Context, Node};

fn main() -> Result<(), Box<dyn Error>> {
    let ctx = Context::create()?;
    let mut node = Node::create(ctx, "node", "")?;
    let mut service = node.create_service::<AddTwoInts::Service>("/add_two_ints")?;
    let service_delayed = node.create_service::<AddTwoInts::Service>("/add_two_ints_delayed")?;
    let client = node.create_client::<AddTwoInts::Service>("/add_two_ints_delayed")?;
    let mut timer = node.create_wall_timer(Duration::from_millis(250))?;
    let mut timer2 = node.create_wall_timer(Duration::from_millis(2000))?;

    let is_ava = node.is_available(&client)?;
    let mut pool = LocalPool::new();
    let spawn = pool.spawner();

    spawn.spawn_local(async move {
        log_warn!("AddTwoInts", "Waiting for Service Available...");
        is_ava.await.unwrap();
        log_info!("AddTwoInts", "Service Available!!");

        while let Some(req) = service.next().await {
                log_info!("AddTwoInts", "Requesting to the Service...");
                let resp = client.request(&req.message).unwrap().await.unwrap();
                log_info!("AddTwoInts", "Result: {} + {} = {}", req.message.a, req.message.b, resp.sum);
                req.respond(resp).unwrap();
        }
    })?;

    spawn.spawn_local(async move {
        let mut service_delayed = service_delayed.fuse();
        loop {
            select! {
                req = service_delayed.next() => {
                    if let Some(req) = req {
                        let resp = AddTwoInts::Response {
                            sum: req.message.a + req.message.b,
                        };
                        let _ = timer2.tick().await;
                        req.respond(resp).unwrap();
                    }
                },
                elapsed = timer2.tick().fuse() => {
                    if let Ok(elapsed) = elapsed {
                        log_info!("AddTwoInts", "No request made in {}ms", elapsed.as_millis());
                    }
                }
            }
        }
    })?;

    spawn.spawn_local(async move {
        loop {
            let elapsed = timer.tick().await.unwrap();
            log_info!("AddTwoInts", "Doing Other Async Works, {}ms since last call", elapsed.as_millis());
        }
    })?;

    loop {
        node.spin_once(Duration::from_millis(100));
        pool.run_until_stalled();
    }

}
