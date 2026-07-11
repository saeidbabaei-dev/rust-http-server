use production_http_server::server::Server;

fn main() {
    let server = Server::new("127.0.0.1:7878").expect("Failed to bind TCP listener");

    server.run().expect("Server stopped unexpectedly");
}
