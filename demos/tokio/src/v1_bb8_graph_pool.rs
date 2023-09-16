/*
cargo run -p nebula-demo-tokio --bin nebula_demo_tokio_v1_bb8_graph_pool 127.0.0.1 3699 user 'password'
*/

use std::env;

use bb8_nebula::{
    graph::GraphClientConfiguration, impl_tokio::v1::graph::new_graph_connection_manager,
};
use fbthrift_transport::AsyncTransportConfiguration;
use nebula_client::v1::{GraphQuery as _, GraphTransportResponseHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or_else(|_| "127.0.0.1".to_owned()));
    let port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PORT").unwrap_or_else(|_| "3699".to_owned()))
        .parse()
        .unwrap();
    let username = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("USERNAME").unwrap_or_else(|_| "user".to_owned()));
    let password = env::args()
        .nth(4)
        .unwrap_or_else(|| env::var("PASSWORD").unwrap_or_else(|_| "password".to_owned()));
    let space = env::args().nth(5).or_else(|| env::var("SPACE").ok());

    println!("v1_bb8_graph_pool {domain} {port} {username} {password} {space:?}",);

    //
    let client_configuration =
        GraphClientConfiguration::new(domain, port, username, password, space);
    let transport_configuration = AsyncTransportConfiguration::new(GraphTransportResponseHandler);
    let manager = new_graph_connection_manager(client_configuration, transport_configuration);
    let pool = bb8::Pool::builder().max_size(1).build(manager).await?;

    //
    {
        let mut session = pool.get().await?;
        let out = session.show_hosts().await?;
        println!("{out:?}");
    }

    //
    {
        let mut session = pool.get().await?;
        let out = session.show_spaces().await?;
        println!("{out:?}");
    }

    Ok(())
}
