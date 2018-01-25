use std::thread;
use std::io::{self, Write};
use std::time::Duration;

fn main() {
    let break_time = Duration::new(9, 0);
    let print_time = Duration::new(1, 0);
    let regiment = ["JUMPING JACKS", 
                    "WALL SIT", 
                    "PUSH-UPS",
                    "ABDOMINAL CRUNCH",
                    "STEP ONTO CHAIR",
                    "SQUAT",
                    "TRICEPS DIP ON CHAIR",
                    "PLANK",
                    "HIGH KNEE RUNNING",
                    "LUNGE",
                    "PUSH-UP + ROTATION",
                    "SIDE PLANK"];

    println!("WOHOO TRAINING!");
    println!(" ");
    thread::sleep(print_time);

    for i in regiment.iter() {
        println!("Make yourself ready for the next set");
        println!("NEXT UP - {}!", i);
        println!(" ");
        thread::sleep(break_time);
        do_set(i);
        thread::sleep(print_time);
        println!(" ");
        println!("WELL DONE!");
        println!(" ");
        thread::sleep(print_time);
    }

}

fn print_update(progress: i64){
    let t = Duration::new(1, 0);
    print!("\r[");
    for i in 0..30 {
        if i < progress {
            print!("=");
        }else if i == progress {
            print!(">");
        }else {
            print!(" ");
        }
    }
    print!("]  {}       ", progress.to_string());
    io::stdout().flush().unwrap();
    thread::sleep(t);
}
fn do_set(set_name: &str) {
    let print_time = Duration::new(1, 0);

    println!("OK LET'S GO!");
    println!(" ");
    thread::sleep(print_time);
    println!("    >>>>> {} <<<<<", set_name);
    for i in 0..30 {
        print_update(i);
    }
    print!("\r[~~~~~~~~~~~~~~DONE~~~~~~~~~~~~]        \n");
    thread::sleep(print_time);
}
