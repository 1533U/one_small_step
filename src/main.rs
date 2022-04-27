use rand::{Rng, random};
use std::io::{self, Write};
use std::{thread, time};
fn main() {
    println!("Let Try again 1533U welcome to MAIN");
    menu()

}

fn menu(){
    println!("Welcome:\nPlease select from the below opttions:\n1: Number Game\n2: Loading Bar for X seconds");
    menu_options(get_int_input_from_user());
}

fn get_int_input_from_user()  -> i32 {
    loop {
        let mut menu_input = String::new();
        io::stdin().read_line(&mut menu_input).unwrap();
        //let menu_inputi: i32 = menu_input.trim().parse().unwrap();
         match menu_input.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_e) => println!("bad input try again, please use a number")
          }
    }

}

fn menu_options(menu_input:i32){
    println!("test");
    match menu_input {
        0=>println!("Closing app.."),
        1=>number_game(),
        2=>loading_bar(),
        _=>menu()
    } 
}

fn number_game(){
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1..101);
    println!("Guess a number between 1 and 100  {}",random);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let match_number: i32 = input.trim().parse().unwrap();
        println!("{}",match_number);
        if match_number == random {
            println!("Correct: {}",match_number);
            break;
        }else if match_number <= random {
            println!("Wrong guess higher than: {}",match_number);
        }else if match_number >= random {
            println!("Wrong guess lower than: {}",match_number);
        }
    }

}

fn loading_bar(){
    println!("load for how long?");
    let ticks = get_int_input_from_user();
    let mut loading_bar_filler: String = "".to_owned();
    let bar_tick = "#";
    let bar_lenght = "                                                                                                    ";
    io::stdout().flush().unwrap();
    let standardized_tickes = 100/ticks;
    println!("{}",standardized_tickes);
    let res = 100%standardized_tickes;
    println!("{}",res);
    print!("[{}]\r",bar_lenght);
    for _tick in 0..ticks{
        for _i in 0..(standardized_tickes){
            loading_bar_filler.push_str(bar_tick);
        }
        print!("[{}\r",loading_bar_filler);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_secs(1));
        
        
    }
    for _i in 0..(res){
        loading_bar_filler.push_str(bar_tick);
    }
    print!("[{}]",loading_bar_filler);

}
