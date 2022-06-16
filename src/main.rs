use finances::transaction::{PaymentMethod, Transaction, TransactionList};

fn main() {
    let transaction: Transaction = Transaction::new("9273vx8937v192361v983",&150.00, &PaymentMethod::CreditCard, "5444566765542543", "Bruno da Silva", "05/29", "111");
    println!("{:#?}",transaction);
}
