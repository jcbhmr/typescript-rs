//! This example is a barebones compiler which takes a list of TypeScript files
//! and compiles them to their corresponding JavaScript.
//!
//! We will need to create a Program, via createProgram - this will create a
//! default CompilerHost which uses the file system to get files.
//!
//! <details><summary>Equivalent JavaScript code</summary>
//!
//! ```js
//! import * as ts from "typescript";
//!
//! function compile(fileNames: string[], options: ts.CompilerOptions): void {
//!   let program = ts.createProgram(fileNames, options);
//!   let emitResult = program.emit();
//!
//!   let allDiagnostics = ts
//!     .getPreEmitDiagnostics(program)
//!     .concat(emitResult.diagnostics);
//!
//!   allDiagnostics.forEach(diagnostic => {
//!     if (diagnostic.file) {
//!       let { line, character } = ts.getLineAndCharacterOfPosition(diagnostic.file, diagnostic.start!);
//!       let message = ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n");
//!       console.log(`${diagnostic.file.fileName} (${line + 1},${character + 1}): ${message}`);
//!     } else {
//!       console.log(ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"));
//!     }
//!   });
//!
//!   let exitCode = emitResult.emitSkipped ? 1 : 0;
//!   console.log(`Process exiting with code '${exitCode}'.`);
//!   process.exit(exitCode);
//! }
//!
//! compile(process.argv.slice(2), {
//!   noEmitOnError: true,
//!   noImplicitAny: true,
//!   target: ts.ScriptTarget.ES5,
//!   module: ts.ModuleKind.CommonJS
//! });
//! ```
//!
//! </details>

// use typescript as ts;

// fn compile(file_names: &[String], options: ts::CompilerOptions) -> Result<(), Box<dyn std::error::Error>> {
//     let program = ts::create_program(file_names, options);
//     let emit_result = program.emit();

//     // TODO

//     let exit_code = if emit_result.emit_skipped { 1 } else { 0 };
//     println!("Process exiting with code '{exit_code}'.");
//     std::process::exit(exit_code);
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // compile(
    //     std::env::args().skip(1).collect::<Vec<_>>().as_slice(),
    //     ts::CompilerOptions {
    //         no_emit_on_error: true,
    //         no_implicit_any: true,
    //         target: ts::ScriptTarget::ES5,
    //         module: ts::ModuleKind::CommonJS,
    //         ..Default::default()
    //     },
    // )?;
    Ok(())
}
