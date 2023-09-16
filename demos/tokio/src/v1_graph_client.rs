/*
cargo run -p nebula-demo-tokio --bin nebula_demo_tokio_v1_graph_client 127.0.0.1 3699 user 'password'
*/

use std::env;

use fbthrift_transport::{AsyncTransport, AsyncTransportConfiguration};
use nebula_client::v1::{GraphClient, GraphQuery as _, GraphTransportResponseHandler};

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

    println!("v1_graph_client {domain} {port} {username} {password}");

    //
    let addr = format!("{domain}:{port}");

    //
    //
    let transport = AsyncTransport::with_tokio_tcp_connect(
        addr,
        AsyncTransportConfiguration::new(GraphTransportResponseHandler),
    )
    .await?;
    let client = GraphClient::new(transport);

    let mut session = client
        .authenticate(username.as_str(), password.as_str())
        .await?;

    let out = session.show_hosts().await?;
    println!("{out:?}");

    Ok(())
}
