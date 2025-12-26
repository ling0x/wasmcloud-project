wit_bindgen::generate!({ generate_all });

use crate::data::parse_request_data;
use crate::exports::wasmcloud::calculation::process_data::{Data, Guest as ProcessDataGuest};
use crate::exports::wasmcloud::messaging::handler::{BrokerMessage, Guest as MessagingGuest};
use crate::wasi::logging::logging::*;
use crate::wasmcloud::calculation::calculator::{multiply, Numbers};
use crate::wasmcloud::messaging::consumer::publish;

mod data;

struct CustomTemplateComponent;

// Custom interface implementation to call provider
impl ProcessDataGuest for CustomTemplateComponent {
    fn process(data: Data) -> String {
        log(Level::Info, "", &format!("Data received: {:?}", data));
        let result = multiply(Numbers {
            number_one: data.first_number,
            number_two: data.second_number,
        });
        format!("Provider is calculating {result:#?}").to_string()
    }
}

// NATS messaging entry point
impl MessagingGuest for CustomTemplateComponent {
    fn handle_message(msg: BrokerMessage) -> Result<(), String> {
        log(
            Level::Info,
            "",
            &format!("Broker message received: {msg:?}"),
        );
        if let Some(reply_to) = msg.reply_to {
            let data = parse_request_data(&msg.body)?;
            let result = match msg.subject.as_str() {
                "wasmcloud.echo" => 0,
                "calculator.add" => data.first_number + data.second_number,

                "calculator.substract" => data.first_number - data.second_number,
                "calculator.multiply" => multiply(Numbers {
                    number_one: data.first_number,
                    number_two: data.second_number,
                }),
                "calculator.divide" => data.first_number / data.second_number,
                "calculator.modulus" => data.first_number % data.second_number,
                _ => 0,
            };
            let reply = format!("Result: {result:?}").into_bytes();
            publish(&BrokerMessage {
                subject: reply_to,
                reply_to: None,
                body: reply,
            })
        } else {
            log(
                Level::Warn,
                "",
                "No reply_to field in message, ignoring message",
            );
            Ok(())
        }
    }
}

export!(CustomTemplateComponent);
