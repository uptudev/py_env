use py_env::*;

fn main() -> PyResult<()> {
    let out = |line: &str| println!("{}", line);
    let err = |line: &str| eprintln!("{}", line);
    PyEnv::new("./py_test/run", out, err)
        .execute("print('hello world')")?;
    Ok(())
}