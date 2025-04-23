use tonic::{transport::Server, Request, Response, Status};

use handler::proto::seoul::city_server::{City, CityServer};
use handler::proto::seoul::{SeoulRequest, SeoulResponse};

#[derive(Debug)]
pub struct CityService {}

#[tonic::async_trait]
impl City for CityService {
    async fn process(
        &self,
        request: Request<SeoulRequest>,
    ) -> Result<Response<SeoulResponse>, Status> {
        println!("fn process()");

        let request = request.into_inner();
        println!("Received request: {:?}", request);

        let response = SeoulResponse {
            granted: true,
            message: "Hello!".to_string(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let city_service = CityService {};

    let service = CityServer::new(city_service);

    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}
