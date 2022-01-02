pub mod events {
  tonic::include_proto!("events");
}

use events::event_service_client::{EventServiceClient};
use events::{EventSubscriptionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EventServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(EventSubscriptionRequest {
        from_offset: 0,
    });

    let mut stream = client.subscribe_to_events(request).await?.into_inner();

    while let Some(feature) = stream.message().await? {
        println!("NOTE = {:?}", feature);
    }

    Ok(())
}
