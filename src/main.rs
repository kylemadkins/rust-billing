use std::io;
use uuid::Uuid;

const EXIT_CMD: &str = "exit";

struct Bill {
    id: Uuid,
    name: String,
    amount: u32
}

struct Bills {
    inner: Vec<Bill>
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

impl Bills {
    fn new() -> Self {
        Self {
            inner: vec![]
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.push(bill);
    }

    fn remove(&mut self, id: String) {
        self.inner.retain(|bill| bill.id.to_string() != id);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.iter().collect()
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
    println!("3.) Remove a bill");
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

fn remove_bill(bills: &mut Vec<Bill>) {
    println!("Enter the bill ID: ");
    let id = get_input().unwrap();
    bills.retain(|bill| bill.id.to_string() != id);
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
                    "3" => remove_bill(&mut bills),
                    _ => println!("Invalid option")
                }
            },
            _ => ()
        }
    }
}
