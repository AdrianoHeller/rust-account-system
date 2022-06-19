use crate::Associate;
use crate::transaction::Transaction;

#[derive(Debug)]
pub enum ActionType {
    CashIn,
    CashOut,
}

#[derive(Debug)]
pub enum ActionEntity {
    Transaction,
    Payable,
}

#[derive(Debug)]
pub struct Action<'a> {
    action_type: &'a ActionType,
    entity: &'a ActionEntity,
    entity_ref: &'a str,
}

impl<'a> Action<'a> {
    pub fn new(action_type: &'a ActionType,entity: &'a ActionEntity,entity_ref: &'a &str) -> Action<'a> {
        Action {
            action_type,
            entity,
            entity_ref,
        }
    }
}

impl Associate<'static,&str> for Action<'static> {
    fn associate_to(&mut self, to_associate: &'static &'static str) -> &'static str {
       self.entity_ref = to_associate;
        to_associate
    }
    fn unassociate_from(&mut self, to_unassociate: &'static &'static str) -> &'static str {
        self.entity_ref = to_unassociate;
        to_unassociate
    }
}
