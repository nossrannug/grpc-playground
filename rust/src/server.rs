pub mod events {
    tonic::include_proto!("events");
}

use events::event_service_server::{EventService,EventServiceServer};
use events::{EventSubscriptionRequest, Event};

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tonic::transport::Server;

#[derive(Debug, Default)]
pub struct Events {}

#[tonic::async_trait]
impl EventService for Events {
    type SubscribeToEventsStream = ReceiverStream<Result<Event, Status>>;

    async fn subscribe_to_events(
        &self,
        request: Request<EventSubscriptionRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<Self::SubscribeToEventsStream>, Status> {
        println!("Got a request: {:?}", request);

        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                tx.send(Ok(Event {
                    id: "123".to_string(),
                    name: "EVENT_SOME".to_string(),
                    timestamp: 321,
                    event_data: "{}".to_string(),
                    offset: 1,
                })).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx))) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let events = Events::default();

    Server::builder()
        .add_service(EventServiceServer::new(events))
        .serve(addr)
        .await?;

    Ok(())
}
