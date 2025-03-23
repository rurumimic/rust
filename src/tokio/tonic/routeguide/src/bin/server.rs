use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

use routeguide::data;
use routeguide::helper::{calc_distance, in_range};
use routeguide::routeguide::route_guide_server::{RouteGuide, RouteGuideServer};
use routeguide::routeguide::{Feature, Point, Rectangle, RouteNote, RouteSummary};

#[derive(Debug)]
pub struct RouteGuideService {
    features: Arc<Vec<Feature>>,
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        println!("fn get_feature()");

        for feature in self.features.iter() {
            if feature.location.as_ref() == Some(request.get_ref()) {
                println!("feature={:?}", feature);
                return Ok(Response::new(feature.clone()));
            }
        }

        println!("feature={:?}", Feature::default());
        Ok(Response::new(Feature::default()))
    }

    type ListFeaturesStream = ReceiverStream<Result<Feature, Status>>;

    async fn list_features(
        &self,
        request: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        println!("fn list_features()");

        let (tx, rx) = mpsc::channel(4);
        let features = self.features.clone();

        tokio::spawn(async move {
            for feature in features.iter() {
                if in_range(feature.location.as_ref().unwrap(), request.get_ref()) {
                    println!("feature={:?}", feature);
                    tx.send(Ok(feature.clone())).await.unwrap();
                }
            }
            println!("End of list_features()");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn record_route(
        &self,
        request: Request<tonic::Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        println!("fn record_route()");

        let mut stream = request.into_inner();

        let mut summary = RouteSummary::default();
        let mut last_point = None;
        let now = Instant::now();

        while let Some(point) = stream.next().await {
            let point = point?;
            summary.point_count += 1;

            for feature in self.features.iter() {
                if feature.location.as_ref() == Some(&point) {
                    summary.feature_count += 1;
                }
            }

            if let Some(last_point) = &last_point {
                println!("Add {:?}", calc_distance(last_point, &point));
                summary.distance += calc_distance(last_point, &point);
            }

            last_point = Some(point);
        }

        summary.elapsed_time = now.elapsed().as_secs() as i32;

        println!("End of record_route()");
        Ok(Response::new(summary))
    }

    type RouteChatStream = Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send + 'static>>;

    async fn route_chat(
        &self,
        request: Request<tonic::Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        println!("fn route_chat()");

        let mut notes = HashMap::new();
        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(note) = stream.next().await {
                let note = note?;

                let location = note.location.clone().unwrap();
                println!("note={:?}", note);

                let location_notes = notes.entry(location).or_insert(vec![]);
                location_notes.push(note);

                for note in location_notes {
                    yield note.clone();
                }
            }
            println!("End of route_chat()");
        };

        println!("Response::new(Box::pin(output))");
        Ok(Response::new(Box::pin(output)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let route_guide = RouteGuideService {
        features: Arc::new(data::load()),
    };

    let service = RouteGuideServer::new(route_guide);

    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}
