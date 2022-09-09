use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionTypes  {
    Deposit,
    Dispute,
    Withdrawal,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
pub struct  TransactionInput {
    #[serde(rename="type")]
    pub transaction_type: TransactionTypes,

    #[serde(rename="client")]
    pub client: u16,

    #[serde(rename="tx")]
    pub tx: u32,

    #[serde(rename="amount")]
    pub amount: Option<f64>,
    
}