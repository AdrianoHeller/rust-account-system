use finances::transaction::{PaymentMethod, Transaction, TransactionList};
use finances::{Hash, transaction};

fn main() {

    let transaction: Transaction = Transaction::new("v682139213v69213",&150.00, &PaymentMethod::CreditCard, "5444566765542543", "Bruno da Silva", "05/29", "111");
    println!("{:#?}",transaction);
    transaction::Transaction::create_random_hash(50);
}
