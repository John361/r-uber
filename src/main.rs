use std::fs;

fn main() {
    let configuration_path: &str = "./tests/configurations.json";
    let configuration_content: String = read_configuration_file(configuration_path);

    println!("{}", &configuration_content);
}

fn read_configuration_file(configuration_file_path: &str) -> String {
    let error_message: String = format!("Cannot read configuration file from path {0}",
                                configuration_file_path);

    fs::read_to_string(configuration_file_path)
        .expect(&error_message)
}
