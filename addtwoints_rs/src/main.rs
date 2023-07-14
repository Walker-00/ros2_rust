use std::error::Error;

use futures::stream::StreamExt;
use futures::task::SpawnExt;
use r2r::{Context, Node};

fn main() -> Result<(), Box<dyn Error>> {
    let ctx = Context::create()?;
    let mut node = Node::create(ctx, "node", "")?;
    let service = node.create_service("/add_two_ints")?;
    Ok(())
}
