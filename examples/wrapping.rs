#![deny(warnings)]
use warp::Filter;

fn wrapper<F, T>(filter: F) -> impl Filter<Extract = (&'static str,)> + Clone + Send + Sync + 'static
where
    F: Filter<Extract = (T,), Error = std::convert::Infallible> + Clone + Send + Sync + 'static,
    F::Extract: warp::Reply,
{
    warp::any()
        .map(|| {
            println!("before filter");
        })
        .untuple_one()
        .and(filter)
        .map(|_arg| {
            "wrapped hello world"
        })
}

#[tokio::main]
async fn main() {

    // Match any request and return hello world!
    let routes = warp::any()
        .and_then(|| async { Ok::<&'static str, warp::Rejection>("Hello, World!") })
        .boxed()
        .recover(|_err| async { Ok("recovered") })
        .with(warp::wrap_fn(wrapper));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
