use super::{TransactionInput, TransactionRecord};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TransactionOperationRecord {
  operation_record: HashMap<u16, TransactionRecord>,
}

impl TransactionOperationRecord {
    pub fn new() -> Self {
      TransactionOperationRecord {
        operation_record: HashMap::new(),
      }
    }

    pub fn add_transaction(&mut self, transaction: TransactionInput){
      self.operation_record
          .entry(transaction.client)
          .or_insert_with(|| TransactionRecord::new(transaction.client))
          .add_transaction(transaction);
    }

    pub fn get_operation_record(&self) -> &HashMap<u16, TransactionRecord> {
      dbg!(&self.operation_record);
      &self.operation_record
    }
}