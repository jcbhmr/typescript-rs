# TypeScript for Rust

🟦 TypeScript compiler API bindings & binaries for Rust

## Installation

```sh
cargo add typescript2
```

<sup>The crates.io package is called `typescript2` because `typescript` was taken.</sup>

## Usage

```rs
use typescript as ts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const SOURCE: &str = "let x: string  = 'string'";
    let result = ts::transpile_module(SOURCE, {
        compiler_options: {
            module: ts::ModuleKind::CommonJS
        }
    });
    println!("{}", serde_json::to_string(&result)?);
}
```

<details><summary>Equivalent JavaScript code</summary>

```js
import * as ts from "typescript";

const source = "let x: string  = 'string'";

let result = ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.CommonJS }});

console.log(JSON.stringify(result));
```

</details>

## Development

**TODO: Get the "A simple transform function" example working.**

This project uses `cargo run --example _<task_name>` for ad hoc tasks. For example, to regenerate the `typescript.tgz` tarball, run `cargo run --example _generate` from the root of the repository. All `.../examples/_<task_name>.rs` files should be marked with `#![doc(hidden)]` to indicate that they aren't actually user-facing examples.

### Binaries

TODO: Want to use `jsinstaller` to generate self-contained PyInstaller-like binaries with a `the-app-name.exe` and a `./_internal/...` project workspace with the actual files. That way `fs.readFile()` and friends would work without any extra effort! https://github.com/jcbhmr/jsinstaller