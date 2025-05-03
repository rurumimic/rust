use tonic::{Request, Response, Status};

use crate::proto::tokyo::city_server::City;
use crate::proto::tokyo::{TokyoRequest, TokyoResponse};

#[derive(Debug)]
pub struct CityService {}

#[tonic::async_trait]
impl City for CityService {
    async fn process(
        &self,
        request: Request<TokyoRequest>,
    ) -> Result<Response<TokyoResponse>, Status> {
        println!("[Tokyo] fn process()");

        let request = request.into_inner();
        println!("Received request: {:?}", request);

        let response = TokyoResponse {
            ack: true,
            duration: 1,
        };

        Ok(Response::new(response))
    }
}
