# dister

dister builds and bundles your wasm web app.

## Installation
`cargo install dister`

## Requirements
- wasm32-unknown-unknown target:

`rustup target add wasm32-unknown-unknown`

- Wasm-bindgen:

`cargo install wasm-bindgen-cli`

- Wasm-opt (optional, wasm-opt is called if found in PATH and the --release flag is passed to `dister build`).

It can be downloaded from the [binaryen repo's releases](https://github.com/WebAssembly/binaryen/releases).

## Usage
```
USAGE:
    dister <SUBCOMMAND>

SUBCOMMANDS:
    build     Build your wasm web app
    clean     Clean output artifacts
    serve     Serve the generated index.html
    deploy    Creates a desktop app using the wasm web app for frontend
    --help    Prints this message or the help of the given subcommand(s)
```

From the manifest directory of your wasm rust applications directory, run:
`dister build` or `dister build --release`

It should generate a dist folder in the manifest directory. The generated html index file can then be served by any server or by running `dister serve`. If you prefer to use another server for development, pass the dist directory to the server:
`python3 -m http.server --dir dist`

If dister finds an index.html shell in the manifest directory, it will use it and replace the `{{SCRIPT}}`  placeholder (à la emscripten shell html) with a script to load the generated js glue code.
```html
<html>
  <head>
  <meta charset="utf-8">
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  </head>
  <body>
    {{SCRIPT}}
  </body>
</html>
```

To create a desktop app:
```
dister deploy --width=800 --height=600 --title="app name"
```