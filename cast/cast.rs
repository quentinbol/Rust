use reqwest::Client;
use serde_json::json;
use std::error::Error;
use std::env;

async fn get_balance(address: &str, node_url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let request_body = json!({
        "jsonrpc": "2.0",
        "method": "eth_getBalance",
        "params": [address, "latest"],
        "id": 1,
    });

    let response = client.post(node_url).json(&request_body).send().await?;

    print!("{}\n", response.status());

    let response_json: serde_json::Value = response.json().await?;
    let balance_hex = response_json["result"]
        .as_str()
        .ok_or("Invalid response format")?;
    print!("{}\n", balance_hex);
    let balance_wei: u128 = u128::from_str_radix(&balance_hex[2..], 16)?;

    Ok(format!("{}", balance_wei))
}

async fn balance(address: &str, node_url: &str) -> Result<(), Box<dyn Error>> {
    match get_balance(address, node_url).await {
        Ok(balance) => print!("{}\n", balance),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}

fn help() {
    println!("Perform Ethereum RPC calls from the comfort of your command line\n");
    println!("\x1b[4mUsage:\x1b[0m cast <COMMAND>\n");

    println!("Commands:");
    println!("  balance                Get the balance of an account in wei [aliases: b]");

    println!("\nOptions:");
    println!("  -h, --help     Print help");
    println!("  -V, --version  Print version");

    println!("\nFind more information in the book: http://book.getfoundry.sh/reference/cast/cast.html");
}

fn wrong_command(command: &String) {
    eprintln!(
        "\x1b[31merror:\x1b[0m unrecognized subcommand '\x1b[33m{}\x1b[0m'\n",
        command
    );
    eprintln!("\x1b[1m\x1b[4mUsage:\x1b[0m\x1b[0m \x1b[1mcast\x1b[0m <COMMAND>\n");
    eprintln!("For more information try \x1b[1mcast help\x1b[0m\n");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No command provided.");
        help();
        return Ok(());
    }

    let command: &String = &args[1];

    match command.as_str() {
        "help" => {
            help();
        }
        "balance" => {

            let address: &String = if args.len() > 2 {
                &args[2]
            } else {
                return Err(Box::from("Error: Address is required for the 'balance' command."));
            };

            let node_url: &str = if let Some(pos) = args.iter().position(|arg: &String| arg == "--rpc-url") {
                args.get(pos + 1).map(|s: &String| s.as_str()).ok_or_else(|| {
                    Box::<dyn Error>::from("a value is required for '--rpc-url <URL>' but none was supplied")
                })?
            } else {
                return Err(Box::from("error: '--rpc-url' flag is missing"));
            };

            balance(address, node_url).await?;
        }
        _ => {
            wrong_command(command);
        }
    };

    Ok(())
}
