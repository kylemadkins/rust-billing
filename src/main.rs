use uuid::Uuid;

pub struct Bill {
    id: Uuid,
    name: String,
    amount: f64
}

pub struct Bills {
    inner: Vec<Bill>
}

impl Bill {
    fn new(name: String, amount: f64) -> Bill {
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

mod menu {
    use std::io;
    use crate::{Bill, Bills};

    const EXIT_CMD: &str = "exit";

    pub enum MenuOption {
        AddBill,
        ViewBills,
        RemoveBill,
        Exit
    }

    impl MenuOption {
        pub fn from_str(input: &str) -> Option<MenuOption> {
            match input {
                "1" => Some(MenuOption::AddBill),
                "2" => Some(MenuOption::ViewBills),
                "3" => Some(MenuOption::RemoveBill),
                EXIT_CMD => Some(MenuOption::Exit),
                _ => None
            }
        }
    }

    pub fn get_input() -> Option<String> {
        let mut buffer = String::new();
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("Please try again:");
        }
        let input = String::from(buffer.trim());
        if &input == "" {
            None
        } else {
            Some(input)
        }
    }

    pub fn get_amount() -> Option<f64> {
        loop {
            let amount = match get_input() {
                Some(input) => input,
                None => return Some(0.0)
            };
            let parsed_amount: Result<f64, _> = amount.parse();
            match parsed_amount {
                Ok(amount) => return Some(amount),
                Err(_) => println!("Please enter a number:")
            };
        }
    }

    pub fn show() {
        println!();
        println!("== Bill Manager ==");
        println!("1.) Add a bill");
        println!("2.) View bills");
        println!("3.) Remove a bill");
        println!();
        println!("Enter a number or type \"{}\": ", EXIT_CMD);
    }

    pub fn add_bill(bills: &mut Bills) {
        println!();
        println!("Enter a name: ");
        let name = match get_input() {
            Some(input) => input,
            None => return
        };
        println!("Enter an amount: ");
        let amount = match get_amount() {
            Some(input) => input,
            None => return
        };
        bills.add(Bill::new(name, amount));
    }

    pub fn view_bills(bills: &Bills) {
        let all_bills = bills.get_all();
        let count = all_bills.len();
        if count > 0 {
            println!();
            println!("Number of bills: {}", count);
            for bill in all_bills {
                println!();
                println!("ID: {}", bill.id);
                println!("Name: {}", bill.name);
                println!("Amount: ${:.2}", bill.amount);
            }
        } else {
            println!();
            println!("There are no bills");
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        println!();
        println!("Enter the bill ID: ");
        let id = match get_input() {
            Some(input) => input,
            None => return
        };
        bills.remove(id);
    }
}

fn main() {
    let mut bills = Bills::new();

    loop {
        menu::show();
        let input = menu::get_input().expect("No data entered");
        match menu::MenuOption::from_str(&input) {
            Some(menu::MenuOption::Exit) => break,
            Some(menu::MenuOption::AddBill) => menu::add_bill(&mut bills),
            Some(menu::MenuOption::ViewBills) => menu::view_bills(&bills),
            Some(menu::MenuOption::RemoveBill) => menu::remove_bill(&mut bills),
            _ => println!("Invalid option")
        }
    }
}
