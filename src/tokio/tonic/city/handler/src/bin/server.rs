use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(handler::proto::reflection::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let server = Server::builder()
        .add_service(reflection)
        .add_service(handler::proto::seoul::city_server::CityServer::new(
            handler::city::seoul::CityService {},
        ))
        .add_service(handler::proto::tokyo::city_server::CityServer::new(
            handler::city::tokyo::CityService {},
        ));

    server.serve(addr).await?;

    Ok(())
}
