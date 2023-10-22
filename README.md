# nebula-rust

Nebula Rust is a Rust client for developers to connect their projects to Nebula Graph.

## Before you start

### Prerequisites

To use this rust client, do a check of these:

- Rust development environment is installed.

- Nebula Graph is deployed. For more information, see [Deployment and installation of Nebula Graph](https://docs.nebula-graph.io/master/4.deployment-and-installation/1.resource-preparations/).

- [Installed Nebula Console]([NebulaGraph Console - NebulaGraph Database Manual (nebula-graph.io)](https://docs.nebula-graph.io/3.6.0/nebula-console/)),then load the test dataset using the following command(If you want to run the demo).

  ```
  nebula> :play basketballplayer;
  ```

### What we can achieve

NebulaGraph is composed of three services: Graph service, Meta service, and Storage service, which is an architecture that separates storage and computing. We have implemented some Graph services and some Storage services.

* graph service : graph query, such as `SHOW HOSTS`
* storage service : scan vertex and edge

Welcome everyone to actively participate in improving the rust client and achieving more functions!

## How to use in your code

### Use graph-client to execute statements

**Step 1:** Navigate to the directory containing your Rust code project.

**Step 2:** Make sure you have installed all the necessary dependencies in your Rust project, and your Nebula Graph cluster is running smoothly.

**Step 3:** Open your terminal or command prompt.

**Step 4:** Replace the command parameters with the values relevant to the cluster you want to query.

**Step 5:** Run the program using the following command:

```
cargo run -p nebula-demo-tokio --bin nebula_demo_tokio_v3_graph_client 127.0.0.1 9669 root 'password'
```

**The demo file is located in demos/tokio/src/v3_graph_client.rs**

The meaning of this command is as follows:

- `-p nebula-demo-tokio`: Specifies the subproject to run (defined in Cargo.toml).
- `--bin nebula_demo_tokio_v3_graph_client`: Specifies the binary file to run.
- `127.0.0.1`: IP address of the Nebula Graph server.
- `9669`: Port number of the Nebula Graph server.
- `root`: Username used for authentication when connecting to Nebula Graph.
- `'password'`: Password used for authentication when connecting to Nebula Graph.

After running the program, it will connect to the Nebula Graph server and return the results of your query based on the query statement you provided. The query results will be printed on the command line.

Make sure the Nebula Graph server is running properly and accessible on the provided IP address and port. Also, ensure that your Rust project contains the necessary dependencies and configuration to communicate with Nebula Graph. If there are any issues in your code, you may need to check error messages for troubleshooting.

If you want to run your own query statement, you need to modify STMT_ SHOW_ HOSTS variables and Host structures.**This part of the code is located in nebula-client/src/v3/graph/query.rs.**

```
const STMT_SHOW_HOSTS: &[u8] = b"SHOW HOSTS;";
#[derive(Deserialize, Debug)]
pub struct Host {
    #[serde(rename(deserialize = "Host"))]
    pub host: String,
    #[serde(rename(deserialize = "Port"))]
    pub port: u16,
   // #[serde(rename(deserialize = "HTTP port"))]
   // pub http_port: u16,
    #[serde(rename(deserialize = "Status"))]
    pub status: String,
    #[serde(rename(deserialize = "Leader count"))]
    pub leader_count: u64,
    #[serde(rename(deserialize = "Leader distribution"))]
    pub leader_distribution: String,
    #[serde(rename(deserialize = "Partition distribution"))]
    pub partition_distribution: String,
    #[serde(rename(deserialize = "Version"))]
    pub version: String,
}
```

### Use storage-client to scan vertex and edge (recommend)

**Step 1:** Navigate to the directory containing your Rust code project.

**Step 2:** Make sure you have installed all the necessary dependencies in your Rust project, and your Nebula Graph cluster is running smoothly.

**Step 3:** Open your terminal or command prompt.

**Step 4:** Replace the command parameters with the values relevant to the cluster you want to query.

**Step 5:** Run the program using the following command:

```
cargo run -p nebula-demo-tokio --bin nebula_demo_scan 192.168.10.21:9559,192.168.10.22:9559,192.168.10.23:9559 basketballplayer player basketballplayer serve
```

**The demo file is located in demos/tokio/src/v3_ scan_ vertex_ edge.rs**

The meaning of this command is as follows:

- `-p nebula-demo-tokio`: Specifies the subproject to run (defined in Cargo.toml).
- `--bin nebula_demo_scan`: Specifies the binary file to run.
- `192.168.10.21:9559,192.168.10.22:9559,192.168.10.23:9559`: IP address and Port number of the Nebula Meta server.
- `basketballplayer`: Passed as the first argument to the program as vspace_name.
- `player`: Passed as the second argument to the program as tag_name.
- `basketballplayer`: Passed as the third argument to the program as espace_name.
- `serve`: Passed as the fourth argument to the program as edge_name.

After running the program, it will connect to the Nebula Graph server and query vertex and edge data based on the provided parameters. The query results will be printed on the command line.

Make sure the Nebula Graph server is running properly and accessible on the provided IP address and port. Also, ensure that your Rust project contains the necessary dependencies and configuration to communicate with Nebula Graph. If there are any issues in your code, you may need to check error messages for troubleshooting.

### Use storage-client to scan vertex and edge (output in the form of structs)

We offer an option to output scan  vertex and edge results in the form of structs, and this option is turned off by default. If you want to enable it, simply add the **'show_struct_result'** feature in nebula-client/Cargo.toml to the default features.

```
[features]
default = ["graph","storage", "meta",show_struct_result]
```



## Reference

Part of the code in this project refers to the [nebula-rs]([bk-rs/nebula-rs: Nebula Graph Client API in Rust. (github.com)](https://github.com/bk-rs/nebula-rs)) project.Thank you for the author's open source contribution.
