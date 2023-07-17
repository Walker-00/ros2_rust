use futures::executor::LocalPool;
use futures::prelude::*;
use futures::task::LocalSpawnExt;
use r2r::{log_info, ROS_DISTRO, Context, Node};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();
    let ctx = Context::create()?;
    let mut node = Node::create(ctx, "node", "ns")?;
    let (param_handler, param_events) = node.make_parameter_handler()?;
    let mut timer = node.create_wall_timer(std::time::Duration::from_secs(5))?;
    let params = node.params.clone();

    log_info!("Ros", "Your current Ros Version: {}", ROS_DISTRO);
    log_info!("Node", "Your current Node Name: {}", node.name()?);
    log_info!("Node", "Your current Node Fully Qualified Name: {}", node.fully_qualified_name()?);
    log_info!("Node", "Your current Node Namespace: {}", node.namespace()?);

    spawner.spawn_local(param_handler)?;

    spawner.spawn_local(async move {
        param_events
            .for_each(|(param_name, param_val)| {
                log_info!("parameter_rs", "Your parameter event: {} is now {:?}", param_name, param_val);
                future::ready(())
            })
            .await
    })?;


    spawner.spawn_local(async move {
        loop {
            log_info!("parameter_rs", "Node Parameters:");
            params.lock().unwrap().iter().for_each(|(k, v)| {
                println!("{} - {:?}", k, v);
            });
            let _ = timer.tick().await.expect("could not tick");
        }
    })?;

    loop {
        node.spin_once(std::time::Duration::from_millis(100));
        pool.run_until_stalled();
    }
}
