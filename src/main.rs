use uuid::Uuid;

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

mod menu {
    use std::io;

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
                "exit" => Some(MenuOption::Exit),
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

    pub fn show() {
        println!();
        println!("== Bill Manager ==");
        println!("1.) Add a bill");
        println!("2.) View bills");
        println!("3.) Remove a bill");
        println!();
        println!("Enter a number or type \"{}\": ", EXIT_CMD);
    }
}

fn add_bill(bills: &mut Vec<Bill>) {
    println!("Enter a name: ");
    let name = menu::get_input().unwrap();
    println!("Enter an amount: ");
    let amount = menu::get_input().unwrap().parse().unwrap();
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
    let id = menu::get_input().unwrap();
    bills.retain(|bill| bill.id.to_string() != id);
}

fn main() {
    let bills = Bills::new();

    loop {
        menu::show();
        let input = menu::get_input().expect("No data entered");
        match menu::MenuOption::from_str(&input) {
            Some(menu::MenuOption::Exit) => break,
            Some(menu::MenuOption::AddBill) => (),
            Some(menu::MenuOption::ViewBills) => (),
            Some(menu::MenuOption::RemoveBill) => (),
            _ => println!("Invalid option")
        }
    }
}
