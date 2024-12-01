use dotenv::dotenv;
use std::env;
use std::fs::create_dir_all;
use utils::get_session;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args
        .get(1)
        .expect("Day must be provided")
        .parse::<u8>()
        .expect("Day must be a number");
    dotenv().ok();
    let client = get_session(day);
    let dir = &format!("src/bin/{:0>2}", day);
    if !std::path::Path::new(dir).exists() {
        create_dir_all(dir).expect("Failed to create directory");
        let template = std::fs::read_to_string("src/bin/setup/main.rs.template")
            .expect("Failed to read template");
        std::fs::write(format!("{dir}/main.rs"), template).expect("Failed to write main file");
    }
    if std::path::Path::new(&format!("{dir}/main_input")).exists() {
        eprintln!("Input file already exists");
        return;
    }

    match client.get_input_text().await {
        Ok(input) => {
            std::fs::write(format!("{dir}/main_input"), input).expect("Failed to write input file");
        }
        Err(e) => {
            eprintln!("Failed to get input: {}", e);
        }
    }
}
