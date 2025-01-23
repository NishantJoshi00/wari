# Wari

A proof of concept implementation for exploring WebAssembly's component model.

## What is the Component Model?

The WebAssembly Component Model is a proposal that enables WebAssembly modules to be more easily composed together and integrated with their host environment. It introduces:

- Interface types for better type safety across language boundaries
- WIT (WebAssembly Interface Types) format for describing interfaces
- Component-level linking and composition

## This Implementation

This project demonstrates key concepts of the component model:

1. **Interface Definition**: Uses WIT to define clear contracts between host and components
2. **Host Bindings**: Shows how host functions are exposed to components
3. **Component Implementation**: Demonstrates writing a component that consumes host functions
4. **Runtime Loading**: Shows how to load and instantiate components using Wasmtime

## Project Structure

```
.
├── src/                 # Host runtime implementation
├── module/              # Sample component
└── wit/                 # Interface definitions
```

## Key Files

- `wit/module.wit`: Defines the interface between host and component
- `src/types.rs`: Implements host functions
- `module/sample-module/`: Example component implementation

## Running the POC

```bash
# Build the runtime
cargo build

# Build the sample component
cd module/sample-module
cargo build --target wasm32-wasi

# Run
cargo run -- path/to/component.wasm < input.txt
```

## Learning Points

1. **Interface-First Design**: WIT files define the contract before implementation
2. **Type Safety**: The component model provides type safety across the host-component boundary
3. **Host Integration**: Components can seamlessly call host functions
4. **Tooling**: Demonstrates wit-bindgen for generating bindings

## References

- [Component Model Specification](https://github.com/WebAssembly/component-model)
- [WIT Documentation](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md)
- [Wasmtime Component Model Guide](https://docs.wasmtime.dev/)
