use rand::rngs::ThreadRng;
use rand::Rng;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use tonic::{transport::Channel, Request};

use routeguide::routeguide::route_guide_client::RouteGuideClient;
use routeguide::routeguide::{Point, Rectangle, RouteNote};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RouteGuideClient::connect("http://[::1]:50051").await?;

    // simple RPC
    let response = client
        .get_feature(Request::new(Point {
            latitude: 409146138,
            longitude: -746188906,
        }))
        .await?;
    println!("RESPONSE={:?}", response);

    // server-side streaming
    print_features(&mut client).await?;

    // client-side streaming
    run_record_route(&mut client).await?;

    // bidirectional streaming
    run_route_chat(&mut client).await?;

    Ok(())
}

async fn print_features(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let rectangle = Rectangle {
        lo: Some(Point {
            latitude: 400000000,
            longitude: -750000000,
        }),
        hi: Some(Point {
            latitude: 420000000,
            longitude: -730000000,
        }),
    };

    let mut stream = client
        .list_features(Request::new(rectangle))
        .await?
        .into_inner();

    while let Some(feature) = stream.message().await? {
        println!("FEATURE={:?}", feature);
    }

    Ok(())
}

async fn run_record_route(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let point_count: i32 = rng.random_range(2..100);

    let mut points = vec![];
    for _ in 0..=point_count {
        points.push(random_point(&mut rng))
    }

    println!("Traversing {} points", points.len());
    let request = Request::new(tokio_stream::iter(points));

    match client.record_route(request).await {
        Ok(response) => println!("SUMMARY={:?}", response.into_inner()),
        Err(e) => println!("something went wrong: {:?}", e),
    }

    Ok(())
}

fn random_point(rng: &mut ThreadRng) -> Point {
    let latitude = (rng.random_range(0..180) - 90) * 10_000_000;
    let longitude = (rng.random_range(0..360) - 180) * 10_000_000;

    Point {
        latitude,
        longitude,
    }
}

async fn run_route_chat(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let start = time::Instant::now();

    let outbound = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(1));

        for _ in 0..4 {
            let time = interval.tick().await;
            let elapsed = time.duration_since(start);
            let note  = RouteNote {
                location: Some(Point {
                    latitude: 409146138 + elapsed.as_secs() as i32,
                    longitude: -746188906,
                }),
                message: format!("at {:?}", elapsed),
            };

            yield note;
        }
    };

    let response = client.route_chat(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(note) = inbound.message().await? {
        println!("NOTE={:?}", note);
    }

    Ok(())
}
