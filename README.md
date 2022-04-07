# dister

Dister builds and bundles your wasm web app.

## Requirements
- wasm32-unknown-unknown target
`rustup target add wasm32-unknown-unknown`
- Wasm-bindgen
`cargo install wasm-bindgen-cli`
- Miniserve (to serve the outputted html file)
`cargo install miniserve`

## Usage
```
USAGE:
    dister <SUBCOMMAND>

SUBCOMMANDS:
    build     Build your wasm web app
    clean     Clean output artifacts
    serve     Serve the generated index.html
    --help    Prints this message or the help of the given subcommand(s)
```

From the root of your wasm rust applications directory, run:
`dister build` or `dister build --release`

It should generate a dist folder in the root. The generated html index file can then be served by any server or by running `dister serve`.