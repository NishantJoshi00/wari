#[allow(warnings)]
mod bindings;

use bindings::Guest;

use bindings::units::module::driver;

struct Component;

impl Guest for Component {
    fn main(input: String) -> String {
        let intent = driver::intend(&input);
        let result = driver::view(&intent);
        driver::done(&result);
        result
    }
}

bindings::export!(Component with_types_in bindings);
