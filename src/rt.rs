// thread_local! {
//     pub static RUNTIME: rquickjs::Runtime = rquickjs::Runtime::new().unwrap();
//     pub static CONTEXT: rquickjs::Context = RUNTIME.with(|rt| {
//         let context = rquickjs::Context::full(&rt).unwrap();
//         context.with(|ctx| init_ctx(ctx));
//         context
//     });
// }

// fn init_ctx(ctx: rquickjs::Ctx<'_>) {
//     let require = rquickjs::Function::new(ctx.clone(), |specifier: String| {
//         match specifier.as_str() {
//             "fs" | "node:fs" => rquickjs::Object::new(require.ctx()),
//         }
//     }).unwrap();
//     let module = rquickjs::Object::new(ctx);
// }

// pub fn with_ctx<R>(f: impl FnOnce(rquickjs::Ctx<'_>) -> R) -> R {
//     CONTEXT.with(|context| context.with(|ctx| f(ctx)))
// }

// pub fn with_exports<R>(f: impl FnOnce(rquickjs::Object) -> R) -> R {
//     with_ctx(|ctx| {
//         // `ts` is like `module.exports` in this case.
//         let exports: rquickjs::Object = ctx.globals().get("ts").unwrap();
//         f(exports)
//     })
// }
