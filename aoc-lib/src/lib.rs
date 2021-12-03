/// Executes some code and records the time it took to run
pub fn time<T, F>(fun: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> T,
{
    let now = std::time::Instant::now();
    let ret = fun();
    let elapsed = now.elapsed();
    (ret, elapsed)
}

/// Reads stdin to a String
pub fn read_stdin() -> String {
    use std::io::Read;
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).unwrap();
    string
}

pub fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(filename).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    string
}
