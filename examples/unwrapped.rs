use py_env::*;

pub fn main() {
    // This code attempts to install faker and run a script dependent on it, panicing upon shell
    // command error (not Python code error).
    PyEnv::new("./py_test")
        .execute("import faker; print(faker.Faker().name())").unwrap();
}
