pub mod amqp {
    use std::clone::Clone;
    use amiquip::{AmqpProperties, Connection, Exchange, Publish, QueueDeclareOptions};

    pub struct AmqpCollector {
        connection: Connection,
        queue_name: String
    }

    impl AmqpCollector {
        pub fn new(url: String, api_key: String, queue_name: String) -> AmqpCollector {
            println!("Opening connection...{}", url);
            AmqpCollector {
                connection: match Connection::insecure_open(&url) {
                    Ok(conn) => conn,
                    Err(err) => { 
                        println!("Some error {}", err);
                        Connection::insecure_open(&url).unwrap()
                    }
                },
                queue_name: queue_name
            }
        }

        pub fn send_message (mut self) {
            let channel = self.connection.open_channel(None).unwrap();
            let _ = channel.queue_declare(
                self.queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..QueueDeclareOptions::default()
                },
            );

            let exchange = Exchange::direct(&channel);
            exchange.publish(Publish::with_properties(
                String::from("Hello World!").as_bytes(),
                "SomeQueue",
                AmqpProperties::default().with_delivery_mode(2),
            )).expect("Something went wrong while sending message");
            println!("Sent message");
        }

        fn drop(self) {
            println!("CLOSING CONNECTION FUCK!!!");
            self.connection.close().unwrap();
        }
    }
}