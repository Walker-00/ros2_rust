use std::{error::Error, time::Duration};

use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use r2r::{self, geometry_msgs::msg::Twist, Context, Node, QosProfile};

fn main() -> Result<(), Box<dyn Error>> {
    let ctx = Context::create()?;
    let mut node = Node::create(ctx, "name", "")?;
    let publisher = node.create_publisher::<Twist>("/turtle1/cmd_vel", QosProfile::default())?;
    let mut timer = node.create_wall_timer(Duration::from_millis(500))?;

    let mut pool = LocalPool::new();
    let spawn = pool.spawner();

    spawn.spawn_local(async move {
        loop {
            let _ = timer.tick().await;
            let mut msg = Twist::default();
            msg.linear.x = 2.;
            msg.angular.z = 1.;
            publisher.publish(&msg).unwrap();
        }
    })?;

    loop {
        node.spin_once(Duration::from_millis(100));
        pool.run_until_stalled();
    }
}
