wit_bindgen::generate!({ generate_all });

use crate::exports::wasmcloud::example::process_data::Data;
use crate::exports::wasmcloud::example::process_data::Guest;
use crate::wasi::logging::logging::*;
use crate::wasmcloud::example::calculator::Numbers;

struct CustomTemplateComponent;

impl Guest for CustomTemplateComponent {
    fn process(data: Data) -> String {
        log(Level::Info, "", &format!("Data received: {:?}", data));
        let result = crate::wasmcloud::example::calculator::calculate(Numbers {
            number_one: data.first_number,
            number_two: data.second_number,
        });
        format!("Provider is calculating {result:#?}").to_string()
    }
}

export!(CustomTemplateComponent);
