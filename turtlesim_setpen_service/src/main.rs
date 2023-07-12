use std::error::Error;
use std::time::Duration;

use futures::task::LocalSpawnExt;
use futures::future;
use futures::{executor::LocalPool, stream::StreamExt};
use r2r::{
    geometry_msgs::msg::Twist,
    turtlesim::{msg::Pose, srv::SetPen},
    Context, Node, QosProfile,
};
use r2r::{log_info, log_warn};
use rand::random;

fn main() -> Result<(), Box<dyn Error>> {
    let ctx = Context::create()?;
    let mut node = Node::create(ctx, "name", "")?;
    let subscriber = node.subscribe::<Pose>("/turtle1/pose", QosProfile::default())?;
    let publisher = node.create_publisher::<Twist>("/turtle1/cmd_vel", QosProfile::default())?;
    let client = node.create_client::<SetPen::Service>("/turtle1/set_pen")?;

    let is_ava = node.is_available(&client)?;
    let mut pool = LocalPool::new();
    let spawn = pool.spawner();

    spawn.spawn_local(async move {
        log_warn!("SetPen Service", "Waiting For SetPen Service...");
        is_ava.await.unwrap();
        let mut msg = Twist::default();
        subscriber.for_each(|pose| {
            if pose.x > 9. || pose.x < 2. || pose.y > 9. || pose.y <2. {
                msg.linear.x = 1.;
                msg.angular.z = 0.9;
            } else {
                msg.linear.x = 5.;
            }
            publisher.publish(&msg).unwrap();

            let [r, g, b, width]: [u8;4] = random();

            let msg = SetPen::Request {r,g,b,width, off:0};

            log_info!("SetPen", "Color changed to red: {r}, green: {g}, blue: {b} with width of {width}");
            client.request(&msg).unwrap();

            future::ready(())
        }).await
    })?;

    loop {
        node.spin_once(Duration::from_millis(100));
        pool.run_until_stalled();
    }
}
