use std::os::raw::c_char;
use std::path::PathBuf;

const USAGE: &str = r#"
dister 0.1.0
Builds and bundles your wasm web app.

USAGE:
    dister <SUBCOMMAND>

SUBCOMMANDS:
    build     Build your wasm web app
    clean     Clean output artifacts
    serve     Serve the generated index.html
    --help    Prints this message or the help of the given subcommand(s)
"#;

const HTML: &str = r#"
<html>
  <head>
  <meta charset="utf-8">
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
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

struct Command {
    name: String,
    args: Vec<String>,
}

impl Command {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            args: vec![],
        }
    }

    pub fn args(&mut self, a: &[&str]) {
        self.args = a.iter().map(|s| s.to_string()).collect();
    }

    pub fn exec(self) -> bool {
        extern "C" {
            pub fn system(s: *const c_char) -> i32;
        }
        let mut cmd = self.name;
        for arg in self.args {
            cmd.push(' ');
            cmd.push_str(&arg);
        }
        cmd.push('\0');
        unsafe { system(cmd.as_ptr() as _) == 0 }
    }
}

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
                cargo.args(&["install", "wasm-bindgen-cli"]);
                if !cargo.exec() {
                    panic!("Failed to install wasm-bindgen!");
                }
            }
            let mut cargo = Command::new("cargo");
            let mut cargo_args = vec!["build", "--target", "wasm32-unknown-unknown"];
            if release {
                cargo_args.push("--release");
            }
            cargo.args(&cargo_args);
            if !cargo.exec() {
                panic!("Failed to build for target wasm32-unknown-unknown!");
            }
            let mut wb = Command::new("wasm-bindgen");
            wb.args(&[&path, "--out-dir", "dist", "--target", "web", "--weak-refs"]);
            if !wb.exec() {
                panic!("Failed to run wasm-bindgen on the generated wasm binary");
            }
            let html = HTML.to_string().replace("{{crate}}", &crate_name);
            let dist = PathBuf::from("dist");
            if dist.exists() {
                std::fs::write(dist.join("index.html"), html).unwrap();
            }
        }
        "serve" => {
            if !check_miniserve() {
                eprintln!("miniserve was not found, running a first-time install...");
                let mut cargo = Command::new("cargo");
                cargo.args(&["install", "miniserve"]);
                if !cargo.exec() {
                    panic!("Failed to install miniserve!");
                }
            }
            let mut serve = Command::new("miniserve");
            serve.args(&["dist", "--index", "index.html"]);
            if !serve.exec() {
                panic!("Failed to exec miniserve!");
            }
        }
        "clean" => {
            let mut cargo = Command::new("cargo");
            cargo.args(&["clean"]);
            if !cargo.exec() {
                panic!("Failed to run cargo clean!");
            }

            let dist = PathBuf::from("dist");
            if dist.exists() {
                std::fs::remove_dir_all(dist).unwrap();
            }
        }
        _ => {
            println!("{}", USAGE);
        }
    }
}

fn check_wasm_bindgen() -> bool {
    let mut wb = std::process::Command::new("wasm-bindgen");
    wb.args(&["--help"]);
    wb.output().is_ok()
}

fn check_miniserve() -> bool {
    let mut wb = std::process::Command::new("miniserve");
    wb.args(&["--help"]);
    wb.output().is_ok()
}
