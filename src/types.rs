use std::collections::HashMap;

wasmtime::component::bindgen!("module" in "wit/module.wit");

pub struct State {
    pub injections: HashMap<String, Injection>,
}

pub struct Injection {
    pub value: usize,
    pub name: String,
}

impl Injection {
    pub fn new(value: usize, name: String) -> Injection {
        Injection { value, name }
    }

    pub fn space_shift(&self, input: String) -> String {
        format!("{}{} - {}", "#".repeat(self.value), self.name, input)
    }
}

impl units::module::driver::Host for State {
    fn intend(&mut self, input: String) -> String {
        tracing::info!(action = "intend", input = input.as_str());
        let output = self.injections.get("intend").unwrap().space_shift(input);
        tracing::info!(action = "intend", output = output.as_str());
        output
    }

    fn done(&mut self, input: String) {
        tracing::info!(action = "done", input = input.as_str());
        self.injections.get("done").unwrap().space_shift(input);
    }

    fn transfer(&mut self, fro: String, to: String, value: String) {
        tracing::info!(
            action = "transfer",
            fro = fro.as_str(),
            to = to.as_str(),
            value = value.as_str()
        );
        let output = self
            .injections
            .get("transfer")
            .unwrap()
            .space_shift(format!("{} from {} to {}", value, fro, to));
        tracing::info!(action = "transfer", output = output.as_str());
    }

    fn view(&mut self, input: String) -> String {
        tracing::info!(action = "view", input = input.as_str());
        let output = self.injections.get("view").unwrap().space_shift(input);
        tracing::info!(action = "view", output = output.as_str());
        output
    }
}
