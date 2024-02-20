use py_env::*;

pub fn main() {
    // This code attempts to install faker and run a script dependent on it, panicing upon shell
    // command error (not Python code error).
    PyEnv::at("./py_test")
        .try_install("faker")
        .try_execute("import faker; print(faker.Faker().name())");
}
