use std::io::{Read, Write};
use std::path::PathBuf;

mod types;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mut state = types::State {
        injections: std::collections::HashMap::new(),
    };

    state.injections.insert(
        "intend".to_string(),
        types::Injection::new(4, "intend".to_string()),
    );
    state.injections.insert(
        "done".to_string(),
        types::Injection::new(4, "done".to_string()),
    );
    state.injections.insert(
        "transfer".to_string(),
        types::Injection::new(4, "transfer".to_string()),
    );
    state.injections.insert(
        "view".to_string(),
        types::Injection::new(4, "view".to_string()),
    );

    let engine = wasmtime::Engine::default();
    let mut linker = wasmtime::component::Linker::new(&engine);

    types::Module::add_to_linker(&mut linker, |state: &mut types::State| state)?;

    let mut store = wasmtime::Store::new(&engine, state);

    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("expected path to wasm file"))?;

    let mut input = String::new();
    _ = std::io::stdin().read_to_string(&mut input)?;
    std::io::stdout().flush()?;

    let input = input.trim();

    anyhow::ensure!(path.exists(), "file does not exist: {:?}", path);

    let component = wasmtime::component::Component::from_file(&engine, path)?;

    let bindings = types::Module::instantiate(&mut store, &component, &linker)?;

    let output = bindings.call_main(store, input)?;

    tracing::info!(source = "runtime", output = output.as_str());

    Ok(())
}
