use std::io::Write;

fn rip_deps(input: &str) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    let mut input_words = input.split_whitespace();
    while let Some(word) = input_words.next() {
        match word {
            "import" => {
                match input_words.next() {
                    Some(package) => {
                        out.push(trim_syntax(package));
                    },
                    _ => {}
                };
            },
            "from" => {
                match (input_words.next(), input_words.next(), input_words.next()) {
                    (Some(package), Some("import"), Some(_)) => {
                        out.push(trim_syntax(package));
                    },
                    _ => {}
                }
            }
            _ => {}
        };
    }
    out
}

fn trim_syntax(input: &str) -> &str {
    input.trim_matches(|c| c == ';' || c == '\'' || c == '"' || c == ',' || c == '.')
}

pub fn get_dependencies(input: &str, path: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut deps = rip_deps(input);
    if deps.is_empty() {
        return Ok(());
    } 

    let mut stdout = std::io::stdout().lock();
    writeln!(stdout, "\x1b[0;1;31mWARNING\x1b[0m: dependencies not installed:")?;

    while let Some(dep) = deps.pop() {
        let package_path = path.join(dep);
        if package_path.exists() {
            continue;
        }
        writeln!(stdout, "\t{dep}")?;
    }

    writeln!(stdout, "")?;
    write!(stdout, "\x1b[0;35mPlease list all packages to install, delimited with spaces\x1b[0m: ")?;
    stdout.flush()?;

    let mut response_str = String::new();
    std::io::stdin()
        .read_line(&mut response_str)?;

    if !response_str.trim().is_empty() {
        let mut handle = std::process::Command::new("pip")
            .args([
                "install",
                &response_str,
                "--target",
                path.join("site-packages")
                    .as_os_str()
                    .to_str()
                    .ok_or("Invalid path")?
            ])
            .spawn()?;
        handle.wait()?;
    }
    Ok(())
}
