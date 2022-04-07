use std::path::PathBuf;
use std::process::Command;

const USAGE: &str = r#"
dister 0.1.0
Builds your wasm web app.

USAGE:
    dister <SUBCOMMAND>

SUBCOMMANDS:
    build     Build the Rust WASM app and all of its assets
    clean     Clean output artifacts
    serve     Serve the output
    --help      Prints this message or the help of the given subcommand(s)
"#;

const HTML: &str = r#"
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type" />
  </head>
  <body>
    <script src="./{{crate}}.js"></script>
    <script type="module">
      import init from "./{{crate}}.js";

      init();
    </script>
  </body>
</html>
"#;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    handle_args(&args);
}

fn handle_args(args: &[String]) {
    if args.len() == 1 {
        println!("{}", USAGE);
        return;
    }
    match args[1].as_str() {
        "build" => {
            let mut release = false;
            if let Some(val) = args.get(2) {
                if val == "--release" {
                    release = true;
                }
            }
            let cargo_toml =
                std::fs::read_to_string("Cargo.toml").expect("Failed to find a Cargo.toml!");
            let pkg: toml::Value = cargo_toml.parse().unwrap();
            let crate_name = format!("{}", pkg["package"]["name"]).replace('"', "");
            let mut path = String::from("./target/wasm32-unknown-unknown/");
            if release {
                path.push_str("release/");
            } else {
                path.push_str("debug/");
            }
            path.push_str(&crate_name);
            path.push_str(".wasm");
            if !check_wasm_bindgen() {
                eprintln!("wasm-bindgen-cli was not found, running a first-time install...");
                let mut cargo = Command::new("cargo");
                cargo.args(["install", "wasm-bindgen-cli"]);
                cargo.output().expect("Failed to install wasm-bindgen!");
            }
            let mut cargo = Command::new("cargo");
            cargo.args([
                "build",
                if release { "--release" } else { "" },
                "--target",
                "wasm32-unknown-unknown",
            ]);
            cargo
                .output()
                .expect("Failed to build for target wasm32-unknown-unknown!");
            let mut wb = Command::new("wasm-bindgen");
            wb.args([&path, "--out-dir", "dist", "--target", "web", "--weak-refs"]);
            wb.output()
                .expect("Failed to run wasm-bindgen on the generated wasm binary");
            let html = HTML.to_string().replace("{{crate}}", &crate_name);
            let dist = PathBuf::from("dist");
            if dist.exists() {
                std::fs::write(dist.join("index.html"), html).unwrap();
            }
        }
        "serve" => {
            let cmd = Command::new("python3")
                .args(["-m", "http.server", "--dir", "dist"])
                .output()
                .expect("Failed to run python server!");
            println!("{}", cmd.status);
        }
        "clean" => {
            let mut cargo = Command::new("cargo");
            cargo
                .args(["clean"])
                .output()
                .expect("Failed to run cargo clean!");
        }
        _ => {
            println!("{}", USAGE);
        }
    }
}

fn check_wasm_bindgen() -> bool {
    let mut wb = Command::new("wasm-bindgen");
    wb.args(["--help"]);
    if let Ok(_) = wb.output() {
        true
    } else {
        false
    }
}
