use std::io;
use uuid::Uuid;

const EXIT_CMD: &str = "exit";

struct Bill {
    id: Uuid,
    name: String,
    amount: u32
}

impl Bill {
    fn new(name: String, amount: u32) -> Bill {
        Bill {
            id: Uuid::new_v4(),
            name: name,
            amount: amount
        }
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(String::from(buffer.trim()))
}

fn display_menu() {
    println!("== Billing Menu ==");
    println!("1.) Add a bill");
    println!("2.) View bills");
    println!("Enter a number or type \"{}\": ", EXIT_CMD);
}

fn add_bill(bills: &mut Vec<Bill>) {
    println!("Enter a name: ");
    let name = get_input().unwrap();
    println!("Enter an amount: ");
    let amount = get_input().unwrap().parse().unwrap();
    bills.push(Bill::new(name, amount));
}

fn view_bills(bills: &mut Vec<Bill>) {
    for bill in bills {
        println!();
        println!("ID: {}", bill.id);
        println!("Name: {}", bill.name);
        println!("Amount: ${:.2}", bill.amount as f64 / 100.0);
        println!();
    }
}

fn main() {
    let mut bills: Vec<Bill> = vec![];

    loop {
        display_menu();
        let input = get_input();
        match input {
            Ok(opt) => {
                match opt.as_str() {
                    EXIT_CMD => break,
                    "1" => add_bill(&mut bills),
                    "2" => view_bills(&mut bills),
                    _ => println!("Invalid option")
                }
            },
            _ => ()
        }
    }
}
