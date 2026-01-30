use std::sync::Arc;

use amqprs::{
    BasicProperties, FieldTable,
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, Channel, QueueBindArguments},
    connection::{Connection, OpenConnectionArguments},
    error::Error,
};
use blog_generic::events::{NewPostPublished, SubscriptionStateChanged};
use serde::Serialize;

use crate::traits::Publish;

enum EventBusError {
    SerializationError,
    PublishingError,
}

struct SendParametersDto {
    bytes_payload: Result<Vec<u8>, EventBusError>,
    routing_header_value: String,
    channel: Option<Channel>,
}

impl SendParametersDto {
    fn new(
        bytes_payload: Result<Vec<u8>, EventBusError>,
        routing_header_value: String,
        channel: Option<Channel>,
    ) -> SendParametersDto {
        SendParametersDto {
            bytes_payload,
            routing_header_value,
            channel,
        }
    }
}

pub async fn create_rabbit_event_bus_service(
    connection_string: &str,
) -> Result<
    Arc<impl Publish<SubscriptionStateChanged> + Publish<NewPostPublished>>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    if connection_string.is_empty() {
        return Err("connection string is empty".into());
    }
    let mut connection_configuration: OpenConnectionArguments = connection_string.try_into()?;
    connection_configuration.connection_name("blog_producer");
    let mut service = RabbitEventBusService::new(connection_configuration);
    service.connect().await?;
    Ok(Arc::new(service))
}

const ROUTING_KEY: &'static str = "blog.events";
const EXCHANGE_NAME: &'static str = "blog.events";
const QUEUE_NAME: &'static str = "blog.events";
const ROUTING_HEADER_KEY: &'static str = "blog.events.type";

struct RabbitEventBusService {
    connection_configuration: OpenConnectionArguments,
    connection: Option<Connection>,
    channel: Option<Channel>,
}

impl RabbitEventBusService {
    fn new(connection_configuration: OpenConnectionArguments) -> RabbitEventBusService {
        println!("RabbitEventBusService created");
        RabbitEventBusService {
            connection_configuration,
            connection: None,
            channel: None,
        }
    }
}

#[async_trait]
trait Connect {
    async fn connect(&mut self) -> Result<(), Error>;
}

//TODO setup correct callback (defaults are "for demo and debugging purposes only")
#[async_trait]
impl Connect for RabbitEventBusService {
    async fn connect(&mut self) -> Result<(), Error> {
        if self.connection.is_some() {
            return Ok(());
        }

        let new_connection = Connection::open(&self.connection_configuration).await?;
        new_connection
            .register_callback(DefaultConnectionCallback)
            .await?;

        let channel = new_connection.open_channel(None).await.unwrap();
        channel.register_callback(DefaultChannelCallback).await?;

        channel
            .queue_bind(QueueBindArguments::new(
                &QUEUE_NAME,
                EXCHANGE_NAME,
                ROUTING_KEY,
            ))
            .await?;

        self.connection = Some(new_connection);
        self.channel = Some(channel);

        Ok(())
    }
}

#[async_trait]
impl Publish<SubscriptionStateChanged> for RabbitEventBusService {
    async fn publish(&self, event: SubscriptionStateChanged) -> () {
        println!(
            "event published: {}, {}",
            event.blog_user_id, event.user_telegram_id
        );
        let send_parameters = SendParametersDto::new(
            to_bytes_payload(event),
            String::from("subscriptionstatechanged"),
            self.channel.clone(),
        );
        publish(send_parameters).await;
    }
}

#[async_trait]
impl Publish<NewPostPublished> for RabbitEventBusService {
    async fn publish(&self, event: NewPostPublished) -> () {
        println!("event published: {}", event.blog_user_id);

        let send_parameters = SendParametersDto::new(
            to_bytes_payload(event),
            String::from("newpostpublished"),
            self.channel.clone(),
        );
        publish(send_parameters).await;
    }
}

async fn publish(parameters: SendParametersDto) -> () {
    if let (Ok(payload), Some(channel)) = (parameters.bytes_payload, parameters.channel) {
        let res = internal_publish(payload, &channel, parameters.routing_header_value).await;
        if res.is_err() {
            println!("Error while publishing message");
        }
    } else {
        println!("Error while parsing event");
    }
}

fn to_bytes_payload<T: Serialize>(event: T) -> Result<Vec<u8>, EventBusError> {
    match serde_json::to_string(&event) {
        Ok(json_string) => Ok(json_string.into_bytes()),
        Err(_) => Err(EventBusError::SerializationError),
    }
}

//TODO add publisher confirms
async fn internal_publish(
    payload: Vec<u8>,
    channel: &Channel,
    routing_value: String,
) -> Result<(), EventBusError> {
    let args = BasicPublishArguments::new(EXCHANGE_NAME, ROUTING_KEY);

    let mut props = BasicProperties::default();
    let mut field_table = FieldTable::new();
    let header_key = ROUTING_HEADER_KEY.try_into().unwrap();
    field_table.insert(header_key, routing_value.into());
    props.with_headers(field_table);

    match channel.basic_publish(props, payload, args).await {
        Ok(_) => Ok(()),
        Err(_) => Err(EventBusError::PublishingError),
    }
}
