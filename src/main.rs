use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Bill {
    id: u128,
    name: String,
    amount: f64
}

fn main() {
    let mut bills:Vec<Bill> = Vec::new();
    while let Ok(selected) = reader(Some("
    ==Manage Bills==
    Select an option:
    1.Add Bills
    2.View Bill
    3.Remove bill
    4.Update Bill
    5.All Bills
    ")){
        let selected_str = selected.as_str();
        match selected_str {
            "1" => {
                match  add_bill(&mut bills) {
                    Ok(msg) =>  println!("{:?}", msg),
                    Err(err) => println!("{:?}",err)
                }
            },
            "2" => {
                match view_bill(&bills) {
                    Ok(bill) => {
                        println!("Your bill is -> {:?}", bill);
                    }
                    Err(err) => println!("{:?}", err)
                }
            },
            "3" => {
                match remove_bill(&mut bills){
                    Ok(msg) => println!("{:?}", msg),
                    Err(err) => println!("{:?}",err)
                }
            },
            "4" => {
                match update_bill(&mut bills){
                    Ok(msg) => println!("{:?}", msg),
                    Err(err) => println!("{:?}", err)
                }
            }
            "5" => {
                println!("{:?}", bills)
            }
            _ => println!("Selected option is not valid.")
        }
    }
}


pub fn reader(text: Option<&str>) -> io::Result<String> {
    if let Some(text) = text {
        println!("{}", text);
    }
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    return Ok(String::from(buffer.trim()));
}

fn add_bill(bills: &mut Vec<Bill>) -> Result<String, String> {
    let mut bill_name = String::new();
    let mut bill_amount: f64 = 0f64;

   while let Ok(name) =  reader(Some("Enter new bill name.")){
       if !(name.len() > 0) {
            println!("Enter a valid name.");
           continue;
       }
       bill_name = name;
       while let Ok(amount_str) = reader(Some("Enter new bill amount.")){
           if let Ok(amount) = amount_str.parse::<f64>(){
                bill_amount  = amount;
           } else{
               println!("Enter a valid amount.");
               continue;
           }
           let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
           let new_bill = Bill{id , name: bill_name, amount: bill_amount};
           bills.push(new_bill);
           return Ok(String::from("Bill added successfully."));
       }
       return Err(String::from("Error"));

   }
    return Err(String::from("Error"));
}
fn view_bill(bills: &Vec<Bill>) -> Result<Bill, String>{
    while let Ok(id_str) = reader(Some("Enter bill id.")){
        if let Ok(id) = id_str.parse::<u128>() {
            // TODO: Review this
            return  match bills.iter().position(|bill| bill.id == id){
                 Some(index) => {
                     let bill = &bills[index];
                     return Ok(bill.to_owned());
                 },
                 None => Err(String::from("Bill not found"))
             }
        } else {
            println!("Enter a valid bill id.")
        }
    }
    return Err(String::from("Bill not found."));

}

fn remove_bill(bills: &mut Vec<Bill>) -> Result<String, String>{
    while let Ok(id_str) = reader(Some("Enter bill id.")){
        if let Ok(id) = id_str.parse::<u128>() {
            return match bills.iter().position(|bill| bill.id == id) {
                Some(index) => {
                    let bill = &bills[index];
                    let mut new_bills: Vec<Bill> = Vec::new();
                    for b in bills.to_owned() {
                        if b.id != bill.id {
                            new_bills.push(b);
                        }
                    }
                    bills.splice(.., new_bills);
                    Ok(String::from("Bill deleted successfully."))
                },
                None => Err(String::from("Bill not found"))

            }
        } else {
            println!("Enter valid bill id.")
        }
    }
    return Err(String::from("Bill not found."));
}


fn update_bill(bills: &mut Vec<Bill>) -> Result<String, String>
{
    while let Ok(id_str) = reader(Some("Enter bill id.")){
        if let Ok(id) = id_str.parse::<u128>() {
            // TODO: Review this
            return match bills.iter().position(|bill| bill.id == id) {
                Some(index) => {
                    let bill = &bills[index];
                    while let Ok(new_name) = reader(Some("Enter new name of the bill.")){
                            if !(new_name.len() > 0){
                                println!("Enter a valid name");
                                continue;
                            }
                            while let Ok(amount_str) = reader(Some("Enter new amount")){
                                if let Ok(amount) = amount_str.parse::<f64>(){
                                    let new_bill = Bill{id: bill.id, name: new_name.to_owned(), amount};
                                    bills[index] = new_bill;
                                    return Ok(String::from("Bill updated successfully."))
                                } else {
                                    println!("Enter a valid amount.")
                                }
                            }
                    }
                    return Err(String::from("Bill not found"))
                },
                None => Err(String::from("Bill not found"))
            }
        } else {
            println!("Enter a valid bill id.")
        }
    }
    return Err(String::from("Bill not found."));

}