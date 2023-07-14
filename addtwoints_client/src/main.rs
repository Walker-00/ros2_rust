use std::{error::Error, time::Duration};

use futures::{executor::LocalPool, task::LocalSpawnExt};
use r2r::{example_interfaces::srv::AddTwoInts, Context, Node, log_warn, log_info};
use rand::random;

fn main() -> Result<(), Box<dyn Error>> {
    let ctx = Context::create()?;
    let mut node = Node::create(ctx, "node", "")?;
    let client = node.create_client::<AddTwoInts::Service>("/add_two_ints")?;

    let is_ava = node.is_available(&client)?;
    let mut pool = LocalPool::new();
    let spawn = pool.spawner();

    spawn.spawn_local(async move {
        log_warn!("AddTwoInts", "Waiting For AddTwoInts Service...");
        is_ava.await.unwrap();
        let mut msg = AddTwoInts::Request::default();
        loop {
            let [a, b]: [i32; 2] = random();
            [msg.a, msg.b] = [a as i64, b as i64];
            let resp = client.request(&msg).unwrap().await.unwrap();
            log_info!("AddTwoInts", "Result: {a} + {b} = {}", resp.sum);
        }
    })?;

    loop {
        node.spin_once(Duration::from_millis(100));
        pool.run_until_stalled();
    }
}
