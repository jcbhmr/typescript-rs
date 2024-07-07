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

This project uses `cargo run --example task-<task_name>` for ad hoc tasks. For example, to regenerate the `typescript.tgz` tarball, run `cargo run --example task-generate` from the root of the repository. All `.../examples/task-<task_name>.rs` files should be marked with `#![doc(hidden)]` to indicate that they aren't actually user-facing examples.

### Binaries

There are two binaries offerred through this package: `tsc` and `tsserver`. These binaries use a pattern similar to PyInstaller: extract a bundled tarball out to an `./_internal/` folder and then use that as the project workspace.

ℹ These _could_ be busybox-ed into a single binary but I don't know how that would work with [`cargo binstall`](https://github.com/cargo-bins/cargo-binstall).