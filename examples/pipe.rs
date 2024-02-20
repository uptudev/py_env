use py_env::*;

fn main() -> PyResult<()> {
    let out = todo!();
    let err = todo!();
    PyEnv::new("./py_test/run", out, err)
        .execute("print('hello world')")?;
    Ok(())
}