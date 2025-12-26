wit_bindgen::generate!({ generate_all });

use crate::exports::wasmcloud::calculation::process_data::{Data, Guest as ProcessDataGuest};
use crate::exports::wasmcloud::messaging::handler::{BrokerMessage, Guest as MessagingGuest};
use crate::wasi::logging::logging::*;
use crate::wasmcloud::calculation::calculator::{calculate, Numbers};

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
        log(
            Level::Info,
            "",
            &format!("Received NATS message on subject: {}", msg.subject),
        );

        // Parse JSON manually into the WIT-generated Data struct
        let body_str = std::str::from_utf8(&msg.body)
            .map_err(|e| format!("Invalid UTF-8 in message body: {}", e))?;

        let data = parse_data_from_json(body_str)?;

        // Process using your existing logic
        let result = Self::process(data);

        log(Level::Info, "", &format!("Processing complete: {}", result));

        // Send reply if requested
        if let Some(reply_to) = msg.reply_to {
            use crate::wasmcloud::messaging::consumer::publish;
            publish(&BrokerMessage {
                subject: reply_to,
                body: result.into_bytes(),
                reply_to: None,
            })
            .map_err(|e| format!("Failed to publish reply: {}", e))?;
        }

        Ok(())
    }
}

// Helper function to parse JSON into WIT-generated Data struct
fn parse_data_from_json(json_str: &str) -> Result<Data, String> {
    use serde_json::Value;

    let json: Value =
        serde_json::from_str(json_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let first_number = json
        .get("first_number")
        .and_then(|v| v.as_i64())
        .ok_or("Missing or invalid 'first_number' field")? as i32;

    let second_number = json
        .get("second_number")
        .and_then(|v| v.as_i64())
        .ok_or("Missing or invalid 'second_number' field")? as i32;

    Ok(Data {
        first_number,
        second_number,
    })
}

export!(CustomTemplateComponent);
