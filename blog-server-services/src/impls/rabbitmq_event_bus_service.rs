use std::fmt::Debug;
use std::sync::Arc;

use amqprs::{
    BasicProperties, FieldTable,
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, Channel, QueueBindArguments},
    connection::{Connection, OpenConnectionArguments},
    error::Error,
};
use blog_generic::events::{NewPostPublished, SubscriptionStateChanged};
use screw_components::dyn_result::DResult;
use serde::Serialize;

use crate::traits::Publish;

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

const ROUTING_KEY: &str = "blog.events";
const EXCHANGE_NAME: &str = "blog.events";
const QUEUE_NAME: &str = "blog.events";
const ROUTING_HEADER_KEY: &str = "blog.events.type";

trait BusEvent: Serialize + Debug {
    const ROUTING_TYPE: &'static str;
}

impl BusEvent for SubscriptionStateChanged {
    const ROUTING_TYPE: &'static str = "subscriptionstatechanged";
}

impl BusEvent for NewPostPublished {
    const ROUTING_TYPE: &'static str = "newpostpublished";
}

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

        let channel = new_connection.open_channel(None).await?;
        channel.register_callback(DefaultChannelCallback).await?;

        channel
            .queue_bind(QueueBindArguments::new(
                QUEUE_NAME,
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
impl<E> Publish<E> for RabbitEventBusService
where
    E: BusEvent + Send + Sync + 'static,
{
    async fn publish(&self, event: E) {
        match self.send(&event).await {
            Ok(()) => println!("event published: {} {event:?}", E::ROUTING_TYPE),
            Err(error) => println!(
                "event not published: {} {event:?}: {error}",
                E::ROUTING_TYPE
            ),
        }
    }
}

impl RabbitEventBusService {
    //TODO add publisher confirms
    async fn send<E: BusEvent>(&self, event: &E) -> DResult<()> {
        let channel = self.channel.as_ref().ok_or("event bus is not connected")?;

        let mut field_table = FieldTable::new();
        let header_key = ROUTING_HEADER_KEY
            .try_into()
            .map_err(|_| format!("{ROUTING_HEADER_KEY} is not a header name"))?;
        field_table.insert(header_key, E::ROUTING_TYPE.to_owned().into());

        let mut props = BasicProperties::default();
        props.with_headers(field_table);

        channel
            .basic_publish(
                props,
                serde_json::to_vec(event)?,
                BasicPublishArguments::new(EXCHANGE_NAME, ROUTING_KEY),
            )
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_routing_types_are_the_ones_consumers_match_on() {
        assert_eq!(
            SubscriptionStateChanged::ROUTING_TYPE,
            "subscriptionstatechanged"
        );
        assert_eq!(NewPostPublished::ROUTING_TYPE, "newpostpublished");
    }
}
