use handler::proto::tokyo::city_client::CityClient;
use handler::proto::tokyo::TokyoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CityClient::connect("http://[::1]:50051").await?;

    // simple RPC
    let response = client
        .process(TokyoRequest {
            signal: 3,
            payload: vec![2, 3, 7],
        })
        .await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
