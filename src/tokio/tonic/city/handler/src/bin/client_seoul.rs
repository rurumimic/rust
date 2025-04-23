use handler::proto::seoul::city_client::CityClient;
use handler::proto::seoul::SeoulRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CityClient::connect("http://[::1]:50051").await?;

    // simple RPC
    let response = client
        .process(SeoulRequest {
            resident_id: 1,
            district: "Gangnam".to_string(),
        })
        .await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
