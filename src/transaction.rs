use crate::Hash;

#[derive(Debug)]
pub enum PaymentMethod {
    DebitCard,
    CreditCard,
}
#[derive(Debug)]
pub struct Transaction<'a> {
    id: &'a str,
    value: &'a f64,
    payment_method: &'a PaymentMethod,
    card_number: &'a str,
    card_holder: &'a str,
    good_thru: &'a str,
    cvv: &'a str,
    is_blocked: bool,
}

impl Copy for Transaction<'_> {}

impl Clone for Transaction<'_> {
    fn clone(&self) -> Self {
        *self
    }
}

impl Hash for Transaction<'_> {}

impl<'a> Transaction<'a> {
    pub fn new(id: &'a str,value: &'a f64,payment_method: &'a PaymentMethod, card_number: &'a str,card_holder: &'a str,good_thru: &'a str,cvv: &'a str) -> Transaction<'a> {
        Transaction {
            id,
            value,
            payment_method,
            card_number,
            card_holder,
            good_thru,
            cvv,
            is_blocked : false,
        }
    }
}

#[derive(Debug)]
pub struct TransactionList<'b> {
    transactions: Vec<Transaction<'b>>
}

impl<'b> TransactionList<'b> {
    fn insert(&mut self,transaction: Transaction<'b> ) -> () {
        self.transactions.push(transaction.clone())
    }

    fn block_transaction(&self,transaction: &'b Transaction) -> Transaction {
        Transaction {
            is_blocked: true,
            ..*transaction
        }
    }

    fn revert_transaction(&mut self, transaction: &'b Transaction) -> TransactionList {
        self.transactions.push(*transaction);
        TransactionList {
            transactions: vec![*transaction]
        }
    }
}

pub trait Accountability {
    fn check_balance(&self) -> f64;
    // fn revert_transaction(&self) -> ();
    // fn repeat_transaction(&self) -> ();
    // fn report_fraud(&self) -> ();
    // fn mask_credit_card(&self);
}

impl Accountability for TransactionList<'_> {
    fn check_balance(&self) -> f64 {
        let mut balance_sum: f64 = 0.00;
        for t in &self.transactions {
            balance_sum += t.value;
        }
        balance_sum
    }
}
