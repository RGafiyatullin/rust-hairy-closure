use std::error;

use futures::future;
use futures::prelude::*;
use tokio::runtime::Runtime;

fn fold_it(xs: Vec<()>) -> impl Future<Item = (), Error = ()> {
    let s = "[one-two-three]".to_owned();
    xs.into_iter()
        .fold::<Box<dyn Future<Item = (), Error = ()> + Send>, _>(
            Box::new(future::ok(())),
            |prev, ()| {
                let fut = prev.and_then(|()| iteration(&s).into_future());
                Box::new(fut)
            },
        )
}
fn iteration(s: &str) -> Result<(), ()> {
    Ok(println!("iteration: {}", s))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let xs = vec![()];
    let runnable = fold_it(xs);
    Runtime::new()
        .unwrap()
        .block_on_all(runnable)
        .expect("Runtime failure");
    Ok(())
}
