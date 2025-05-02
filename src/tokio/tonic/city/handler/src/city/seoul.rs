use tonic::{Request, Response, Status};

use crate::proto::seoul::city_server::City;
use crate::proto::seoul::{SeoulRequest, SeoulResponse};

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
