use std::collections::HashMap;
use std::io;
use std::time;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

type Bills = HashMap<String, Bill>;

fn main() {
    let mut bills: Bills = HashMap::new();

    loop {
        println!(
            "
            < Bill Management Project >
            1. View all bills
            2. View a bill
            3. Add new bill
            4. Update a bill
            5. Delete a bill
            6. Total
        "
        );
        let chosen = readline("Select your option:");

        match chosen.as_str() {
            "1" => view_all_bills_interface(&bills),
            "2" => view_bill_interface(&bills),
            "3" => add_bill_interface(&mut bills),
            "4" => update_bill_interface(&mut bills),
            "5" => delete_bill_interface(&mut bills),
            "6" => show_total_interface(&mut bills),
            _ => {}
        }
    }
}

fn readline(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    if let Ok(_) = io::stdin().read_line(&mut buffer) {
        String::from(buffer.trim())
    } else {
        panic!("Something went wrong while taking input from user.")
    }
}

fn view_all_bills_interface(bills: &Bills) {
    for key in bills.keys() {
        let bill = bills.get(key).unwrap();
        println!(
            "- ID = {:?} | - Name = {:?} | - Amount = {:?}",
            key, bill.name, bill.amount
        );
    }
}

fn add_bill_interface(bills: &mut Bills) {
    let mut bill_name: String = String::new();
    loop {
        let bill_name_str = readline("Enter name of the bill: ");
        if bill_name_str.len() == 0 {
            println!("Name of the bill is required.\n");
        } else {
            bill_name = bill_name_str;
            break;
        }
    }

    let mut bill_amount: f64 = 0f64;
    loop {
        let bill_amount_str = readline("Enter amount of the bill: ");
        if bill_amount_str.len() == 0 {
            println!("Amount of the bill is required.\n");
        } else {
            if let Ok(parsed_amount) = bill_amount_str.parse::<f64>() {
                bill_amount = parsed_amount;
                break;
            } else {
                println!("Enter a valid amount.");
            }
        }
    }

    let bill = Bill {
        name: bill_name,
        amount: bill_amount,
    };
    let id: u128 = get_new_id();
    bills.insert(id.to_string(), bill);
    println!("New bill added successfully")
}

fn get_new_id() -> u128 {
    let now = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap();
    now.as_millis()
}

fn view_bill_interface(bills: &Bills) {
    let bill_id = readline("Enter the id of the bill: ");
    if let Some(bill) = bills.get(bill_id.as_str()) {
        println!(
            "- ID = {:?} | - Name = {:?} | - Amount = {:?}",
            bill_id, bill.name, bill.amount
        );
    } else {
        println!("No such bill exists with that ID.")
    }
}

fn delete_bill_interface(bills: &mut Bills) {
    let bill_id = readline("Enter the id of the bill to delete: ");
    if let Some(_) = bills.get(bill_id.as_str()) {
        bills.remove(bill_id.as_str());
        println!("Bill deleted successfully.")
    } else {
        println!("No such bill exists with that ID.")
    }
}

fn show_total_interface(bills: &Bills) {
    let mut total: f64 = 0f64;

    for key in bills.keys() {
        if let Some(bill) = bills.get(key) {
            total += bill.amount;
        }
    }

    println!("Tota is = {}", total)
}

fn update_bill_interface(bills: &mut Bills) {
    let bill_id = readline("Enter the id of bill to update:\n");
    if let Some(_) = bills.get(bill_id.as_str()) {
        let mut bill_new_name: String = String::new();
        loop {
            let bill_name = readline("Enter new name of the bill: ");
            if bill_name.len() != 0 {
                bill_new_name = bill_name;
                break;
            } else {
                println!("Name is required.")
            }
        }

        let mut bill_new_amount: f64 = 0f64;
        loop {
            let bill_amount_str = readline("Enter new amount of the bill: ");
            if bill_amount_str.len() != 0 {
                if let Ok(parsed_amount) = bill_amount_str.parse::<f64>() {
                    bill_new_amount = parsed_amount;
                    break;
                };
            } else {
                println!("Amount is required.")
            }
        }

        bills.insert(
            bill_id,
            Bill {
                name: bill_new_name,
                amount: bill_new_amount,
            },
        );
    } else {
        println!("Invalid bill ID.")
    }
}
