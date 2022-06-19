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