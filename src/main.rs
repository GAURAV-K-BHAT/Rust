use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!(" guess the number ! ");
    let secrate_number = rand::thread_rng().gen_range(1..=100);
    loop {
            // println!("The secrate number is {secrate_number}");
            println!( "{}","Please Enter the number ...".trim().on_white().bold().truecolor(0,0,0));

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess) 
                .expect(" failed to read line");

                let guess: u32 = match guess.trim().parse(){
                    Ok(num) => num,
                    Err(_) => continue,
                }; 
     
                println!(" Your Guessed Number Is : {guess}");

                match guess.cmp(&secrate_number){
                    Ordering::Less => println!("{}","Input Is Low ".on_yellow().truecolor(0,0,0)),
                    Ordering::Greater => println! ("{}","Input Is High".on_red().truecolor(0,0,0)),
                    Ordering::Equal => {
                        println!("{}","Congratulations You Won !!! " .on_green().green().italic().bold().truecolor(0,0,0)); 
                        break;
                    }

                }
        }  
    }

