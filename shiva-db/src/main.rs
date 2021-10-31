const DB_FILE_PATH: &str = "shiva.db";
fn main() {
    let mut args =  std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    println!("The key is {}", key);
    println!("The value is {}", value);
    let content = format!("{}\t{}", key, value);
    std::fs::write(DB_FILE_PATH, content).ok();
}
