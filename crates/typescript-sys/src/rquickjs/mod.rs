/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/

mod node_fs;

struct State {
    pub rt: rquickjs::Runtime,
    pub ctx: rquickjs::Context,
}

const TYPESCRIPT_JS: &str = include_str!("../../node_modules/typescript/lib/typescript.js");

impl State {
    pub fn new() -> Self {
        let rt = rquickjs::Runtime::new().unwrap();
        let ctx = rquickjs::Context::full(&rt).unwrap();

        ctx.with(|ctx| {
            let global = ctx.globals();

            ctx.eval::<(), _>(TYPESCRIPT_JS).unwrap();

            let ts: rquickjs::Object = global.get("ts").unwrap();
            _ = ts;
        });

        Self { rt, ctx }
    }

    pub fn with_exports<R>(&self, f: impl FnOnce(rquickjs::Object) -> R) -> R {
        self.ctx.with(|ctx| {
            let exports: rquickjs::Object = ctx.globals().get("ts").unwrap();
            f(exports)
        })
    }
}

thread_local! {
    static STATE: State = State::new();
}
