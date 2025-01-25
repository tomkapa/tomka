use crate::helpers::TestApp;
use futures::StreamExt;
use tomka::protobuf::queue::publishing_queue_client::PublishingQueueClient;
use tomka::protobuf::queue::{ConsumeChunkRequest, Message, PublishChunkRequest};
use tonic::transport::Channel;

mod helpers;

#[tokio::test]
async fn test_integration_queue() {
    let app = TestApp::new().await;

    // 4) Now we know the actual port
    let channel = Channel::from_shared(format!("http://{}", app.address.to_string()))
        .expect("Invalid address")
        .connect()
        .await
        .expect("Failed to connect");

    let mut client = PublishingQueueClient::new(channel.clone());

    // 5) Publish some chunks
    let total_messages = 5;
    for i in 0..total_messages {
        let message = Message {
            message_id: format!("chunk-{}", i),
            data: format!("payload-{}", i),
        };
        let req = PublishChunkRequest {
            message: Some(message),
        };
        client.publish_chunk(req).await.expect("Failed to publish");
    }

    // 6) Subscribe in a separate task, read messages
    let mut client2 = PublishingQueueClient::new(channel);
    let inbound = client2
        .consume_chunk(ConsumeChunkRequest {})
        .await
        .expect("Failed to consume")
        .into_inner();

    let received = inbound.take(total_messages).collect::<Vec<_>>().await;

    // 7) Verify we got them all
    assert_eq!(received.len(), total_messages);
    for msg in received {
        let res = msg.unwrap();
        assert!(res
            .clone()
            .message
            .unwrap()
            .message_id
            .starts_with("chunk-"));
        println!("Got: {:?}", res);
    }
}
