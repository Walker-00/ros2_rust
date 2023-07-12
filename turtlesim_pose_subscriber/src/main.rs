use std::{error::Error, time::Duration};

use futures::{executor::LocalPool, stream::StreamExt, task::LocalSpawnExt, future};
use r2r::{Context, Node, QosProfile, turtlesim::msg::Pose, log_info};

fn main() -> Result<(), Box<dyn Error>> {
    let ctx = Context::create()?;
    let mut node = Node::create(ctx, "name", "")?;
    let subscriber = node.subscribe::<Pose>("/turtle1/pose", QosProfile::default())?;

    let mut pool = LocalPool::new();
    let spawn = pool.spawner();

    spawn.spawn_local(async move {
        subscriber.for_each(|msg| {
            log_info!("Turtlesim Pose Subscriber.", "Turtle is in x: {}, y: {}", msg.x, msg.y);
            future::ready(())
        }).await
    })?;

    loop {
        node.spin_once(Duration::from_millis(100));
        pool.run_until_stalled();
    }
}
