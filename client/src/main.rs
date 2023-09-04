use repl_rs::Command;
use repl_rs::{Repl, Value};
use std::collections::HashMap;
use std::string::ToString;
use server_sdk::apis::configuration::{ApiKey, Configuration};

 fn ping(_args: HashMap<String, Value>, context: &mut &Configuration) -> repl_rs::Result<Option<String>> {
    match server_sdk::apis::default_api::ping(context ) {
        Ok(resp) => Ok(Some(resp)),
        Err(e) => Ok(Some(format!("Failed to ping {}", e)))
    }
}

fn main() {
    let config =  &Configuration{
        base_path: "https://chatrs-server2.shuttleapp.rs".to_string(),
        api_key: Some(ApiKey{
            prefix: None,
            key: "509fdff0-4236-482b-b1a6-613e7666997d".to_string()
        }),
        ..Default::default()
    };

    let mut repl = Repl::new(config)
        .with_name("chat_rs client")
        .with_version("0")
        .add_command(Command::new("ping", ping).with_help("Ping the server"));

    repl.run().expect("Failed to start repl");
}
