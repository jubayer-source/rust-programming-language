fn main(){
    let server_port = 8080;

    let mut active_connections = 0;
    active_connections += 1;

    println!("Server running on port: {}",server_port);
    println!("Active connections: {}",active_connections);
}