wit_bindgen::generate!({ generate_all });

use crate::exports::wasmcloud::calculation::process_data::{Data, Guest as ProcessDataGuest};
use crate::exports::wasmcloud::messaging::handler::{BrokerMessage, Guest as MessagingGuest};
use crate::wasi::logging::logging::*;
use crate::wasmcloud::calculation::calculator::{calculate, Numbers};
use crate::wasmcloud::messaging::consumer::publish;

struct CustomTemplateComponent;

// Custom interface implementation to call provider
impl ProcessDataGuest for CustomTemplateComponent {
    fn process(data: Data) -> String {
        log(Level::Info, "", &format!("Data received: {:?}", data));
        let result = calculate(Numbers {
            number_one: data.first_number,
            number_two: data.second_number,
        });
        format!("Provider is calculating {result:#?}").to_string()
    }
}

// NATS messaging entry point
impl MessagingGuest for CustomTemplateComponent {
    fn handle_message(msg: BrokerMessage) -> Result<(), String> {
        log(Level::Info, "", &format!("Broker message: {msg:?}"));
        if let Some(reply_to) = msg.reply_to {
            publish(&BrokerMessage {
                subject: reply_to,
                reply_to: None,
                body: msg.body,
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
