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
    pub fn new(action_type: &'a ActionType,entity: &'a ActionEntity,entity_ref: &'a str) -> Action {
        Action {
            action_type,
            entity,
            entity_ref
        }
    }
}