mod currency;

use std::io;

use currency::{clr, currency_conversion, supported_currencies, user_prompt, CURRENCY_LIST};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    clr();
    println!(
        r#"
_________                                                   _________                                   __                
\_   ___ \ __ ________________   ____   ____   ____ ___.__. \_   ___ \  ____   _______  __ ____________/  |_  ___________ 
/    \  \/|  |  \_  __ \_  __ \_/ __ \ /    \_/ ___<   |  | /    \  \/ /  _ \ /    \  \/ // __ \_  __ \   __\/  _ \_  __ \
\     \___|  |  /|  | \/|  | \/\  ___/|   |  \  \___\___  | \     \___(  <_> )   |  \   /\  ___/|  | \/|  | (  <_> )  | \/
 \______  /____/ |__|   |__|    \___  >___|  /\___  > ____|  \______  /\____/|___|  /\_/  \___  >__|   |__|  \____/|__|   
        \/                          \/     \/     \/\/              \/            \/          \/                          
"#
    );

    loop {
        println!("\n👋 Hey there! Press 'q' to quit ❌, 'c' for the latest currency exchange rates 💰, or 's' to see which currencies are supported 🌍. Don't miss out!");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read a line");

        match user_input.trim().to_lowercase().as_str() {
            "c" => {
                let from_currency = loop {
                    let value = user_prompt("\nSource Currency (e.g., USD)").to_uppercase();
                    if CURRENCY_LIST.contains(&value.as_str()) {
                        break value;
                    } else {
                        println!("This currency {} is not supported", value);
                    }
                };

                let to_currency = 'main_loop: loop {
                    let value =
                        user_prompt("\nTarget Currencies (e.g., EUR,GBP,JPY)").to_uppercase();

                    let currency_code: Vec<&str> = value.split(',').collect();

                    for item in &currency_code {
                        if !CURRENCY_LIST.contains(item) {
                            println!("This currency {} is not supported", item);
                            continue 'main_loop;
                        }
                    }
                    break 'main_loop value;
                };

                let desired_amount: u64 = loop {
                    match user_prompt("\nAmount to convert").parse() {
                        Ok(amount) => break amount,
                        Err(_) => {
                            println!("Invalid integer value given");
                        }
                    }
                };
                if let Err(e) = currency_conversion(from_currency, to_currency, desired_amount) {
                    eprintln!("Error during conversion: {}", e);
                };
            }
            "s" => supported_currencies(),
            "q" => break println!("\nThank you for using our system !!"),
            _ => println!("\nInvalid input please try again!!"),
        }
    }

    Ok(())
}
