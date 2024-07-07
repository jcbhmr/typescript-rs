//! Creating a compiler is not too many lines of code, but you may want to just
//! get the corresponding JavaScript output given TypeScript sources. For this
//! you can use ts.transpileModule to get a string => string transformation in
//! two lines.
//! 
//! <details><summary>Equivalent JavaScript code</summary>
//! 
//! ```js
//! import * as ts from "typescript";
//! 
//! const source = "let x: string  = 'string'";
//! 
//! let result = ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.CommonJS }});
//! 
//! console.log(JSON.stringify(result));
//! ```
//! 
//! </details>

use typescript as ts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const SOURCE: &str = "let x: string  = 'string'";
    let result = ts::transpile_module(SOURCE, ts::TranspileOptions{
        compiler_options: ts::CompilerOptions{
            module: ts::ModuleKind::CommonJS,
            ..Default::default()
        },
        ..Default::default()
    });
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}
