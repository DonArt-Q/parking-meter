// THIS IS CODED INCREDIBLY BADLY!! TRY NOT TO GET CONFUSED!

// crates
use rand::Rng; // for random
use std::{thread, time, io}; // input/output and sleeping inbetween stuff
use clearscreen; // self explanatory, used to clear the screen.

fn main() {

    let mut program_running = true; // while this is true the program keeps running

    while program_running == true { // while loop that encompasses the program

        let money_needed: u32 = rand::thread_rng() // generate a random number (the money needed for the parking meter)
            .gen_range(200..=1000);

        let money_needed: f32 = money_needed as f32; // turn money_needed into a float
        let money_needed: f32 = money_needed/100 as f32; // divide money_needed by 100 so that it isn't absurdly high... and for 2 digit precision!
        
        println!("The amount of money that the parking meter needs is: £{:.2}", money_needed); // self explanatory

        println!("Would you like to pay for this? [y/n]");

        let mut yes_or_no = String::new();

        io::stdin() // input that asks whether the user would like to pay or not.
            .read_line(&mut yes_or_no)
            .expect("Failed to read line.");

        let yes_or_no: &str = &yes_or_no // turn it into lowercase, so that even uppercase works!
            .trim()
            .to_lowercase();

        if yes_or_no == ("y") { // if "y" was inputted, do this if statement, i guess.
            let mut money_paid = String::new();

            println!("How much would you like to pay?"); // again, self explanatory, asks the user how much they would like to pay
            io::stdin()
                .read_line(&mut money_paid)
                .expect("Failed to read line.");

            let mut money_paid: f32 = money_paid // turn money_paid into a float
                .trim()
                .parse()
                .expect("Not a number!");

            while money_needed > money_paid { // while loop - did the user pay too little?
                let amount_more = money_needed - money_paid;
                let mut need_more = String::new();

                println!("You still need to pay £{:.2} more.", amount_more);
                io::stdin()
                    .read_line(&mut need_more)
                    .expect("Failed to read line.");

                let need_more: f32 = need_more
                    .trim()
                    .parse()
                    .expect("Not a number!");

                money_paid = money_paid + need_more;
            }

            if money_needed == money_paid { // did the user pay exactly enough?
                println!("You've paid exactly enough.");
                program_running = false;
            } else if money_paid > money_needed { // did the user pay more than what was needed?
                let change = money_paid - money_needed;
                println!("You've paid £{:.2} in total, and your change is £{:.2}", money_paid, change);
                program_running = false;
            }
        } else if yes_or_no == "n" {
            another_parking_lot();
        } else {
            println!("Invalid option. Aborting.");
            program_running = false;
        }
    }
}

fn clear_screen() {
    clearscreen::ClearScreen::default().clear().expect("clearing screen failed.");
}

fn another_parking_lot() {
    clear_screen();

    println!("Going to another parking lot...");

    thread::sleep(time::Duration::from_millis(750));
}