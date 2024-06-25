pub struct NodeFs;

impl rquickjs::module::ModuleDef for NodeFs {
    fn declare(declare: &rquickjs::module::Declarations) -> rquickjs::Result<()> {
        declare.declare("n")?;
        declare.declare("s")?;
        declare.declare("f")?;
        Ok(())
    }

    fn evaluate<'js>(
        ctx: &rquickjs::Ctx<'js>,
        exports: &rquickjs::module::Exports<'js>,
    ) -> rquickjs::Result<()> {
        exports.export("n", 123)?;
        exports.export("s", "abc")?;
        exports.export(
            "f",
            rquickjs::Function::new(ctx.clone(), |a: f64, b: f64| (a + b) * 0.5)?.with_name("f")?,
        )?;
        Ok(())
    }
}
