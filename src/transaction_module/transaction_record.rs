use super::TransactionInput;
use super::TransactionOperation;
use super::TransactionTypes;

#[derive(Debug)]
pub struct TransactionRecord {
  pub client: u16,
  transactions: Vec<TransactionInput>,
}

impl TransactionRecord {
    pub fn new(client: u16) -> Self {
      TransactionRecord {
        client,
        transactions: Vec::new(),
      }
    }
       
    pub fn add_transaction(&mut self, transaction: TransactionInput){
      self.transactions.push(transaction);

    }

    pub fn process_transactions(&self) -> TransactionOperation {
      
      let mut operation = TransactionOperation::new(self.client);

      for transaction in &self.transactions {
        self.process_transaction(&transaction, &mut operation)
      }
      operation
    }

    pub fn process_transaction(&self, transaction: &TransactionInput, operation: &mut TransactionOperation,){
      match &transaction.transaction_type{
        TransactionTypes::Deposit => {
          if let Some(amount) = transaction.amount {
            operation.deposit(amount);
          }
        }
        TransactionTypes::Withdrawal => {
          if let Some(amount) = transaction.amount {
            operation.withdraw(amount);
          }
        }
        TransactionTypes::Dispute => {
          if let Some(existing_transaction) = self.find_transaction(transaction.tx) {
            operation.hold(existing_transaction.amount.unwrap());
          }
        }
        TransactionTypes::Resolve => {
          if let Some(existing_transaction) = self.find_transaction(transaction.tx) {
            operation.release(existing_transaction.amount.unwrap());
          }
        }
        TransactionTypes::Chargeback => {
          // Check if transaction to charge back is under dispute
          if let Some(_existing_transaction_dispute) = self.find_transaction_dispute(transaction.tx) {
           if  let Some(existing_transaction) = self.find_transaction(transaction.tx){
              operation.chargeback(existing_transaction.amount.unwrap());
            }
          }else {
            eprintln!("Charge back ignored, transaction is not under dispute");
          }
        }
      }
    } 

    fn find_transaction(&self, tx: u32) -> Option<&TransactionInput> {
      self.transactions
          .iter()
          .filter(|transaction| {
            TransactionTypes::Deposit == transaction.transaction_type 
            || TransactionTypes::Withdrawal == transaction.transaction_type
          })
          .find(|transaction| transaction.tx == tx)
    }

    fn find_transaction_dispute(&self, tx: u32) -> Option<&TransactionInput>{
      self.transactions
          .iter()
          .filter(|transaction| TransactionTypes::Dispute == transaction.transaction_type)
          .find(|transaction| transaction.tx == tx)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn transaction_deposit_test(){
    let mut record_test = TransactionRecord::new(1);

    let deposit = TransactionInput {
      transaction_type: TransactionTypes::Deposit,
      client:1,
      tx:1,
      amount: Some(1.2222_f64),
    };
    record_test.add_transaction(deposit);
    let transaction = record_test.process_transactions();
    assert_eq!(transaction.total, 1.2222_f64);
    assert_eq!(transaction.held, 0.0_f64);
    assert_eq!(transaction.available, 1.2222_f64);
    assert_eq!(transaction.locked, false);
  }
  #[test]
  fn test_deposit_and_withdrawal(){
    let mut record_test = TransactionRecord::new(1);

    let deposit = TransactionInput {
      transaction_type: TransactionTypes::Deposit,
      client:1,
      tx:1,
      amount: Some(1.2222_f64),
    };

    let withdraw = TransactionInput {
      transaction_type: TransactionTypes::Withdrawal,
      client:1,
      tx:2,
      amount: Some(0.2222_f64),
    };
    record_test.add_transaction(deposit);
    record_test.add_transaction(withdraw);
    let transaction = record_test.process_transactions();
    assert_eq!(transaction.total, 1.000_f64);
    assert_eq!(transaction.held, 0.0_f64);
    assert_eq!(transaction.available, 1.000_f64);
    assert_eq!(transaction.locked, false);
  }

#[test]
  fn test_ignore_deposit_with_infity_amount(){
    let mut record_test = TransactionRecord::new(1);

      let deposit_1 = TransactionInput {
        transaction_type: TransactionTypes::Deposit,
        client:1,
        tx:1,
        amount: Some(f64::MAX),
      };

      let deposit_2 = TransactionInput {
        transaction_type: TransactionTypes::Deposit,
        client:1,
        tx:2,
        amount: Some(1.0),
      };
      record_test.add_transaction(deposit_1);
      record_test.add_transaction(deposit_2);
      let transaction = record_test.process_transactions();
      assert_eq!(transaction.total, f64::MAX);
      assert_eq!(transaction.held, 0.0_f64);
      assert_eq!(transaction.available, f64::MAX);
      assert_eq!(transaction.locked, false);
  }

  #[test]
  fn test_ignore_withdrawal_more_than_available(){
    let mut record_test = TransactionRecord::new(1);

    let deposit = TransactionInput {
      transaction_type: TransactionTypes::Deposit,
      client:1,
      tx:1,
      amount: Some(1.0_f64),
    };

    let withdraw = TransactionInput {
      transaction_type: TransactionTypes::Withdrawal,
      client:1,
      tx:2,
      amount: Some(2.0_f64),
    };
    record_test.add_transaction(deposit);
    record_test.add_transaction(withdraw);
    let transaction = record_test.process_transactions();
    assert_eq!(transaction.total, 1.000_f64);
    assert_eq!(transaction.held, 0.0_f64);
    assert_eq!(transaction.available, 1.000_f64);
    assert_eq!(transaction.locked, false);
  }
  #[test]
  fn test_dispute(){
    let mut record_test = TransactionRecord::new(1);

    let deposit_1 = TransactionInput {
      transaction_type: TransactionTypes::Deposit,
      client:1,
      tx:1,
      amount: Some(10.0_f64),
    };

    let deposit_2 = TransactionInput {
      transaction_type: TransactionTypes::Deposit,
      client:1,
      tx:2,
      amount: Some(2.0_f64),
    };
    let dispute = TransactionInput {
      transaction_type: TransactionTypes::Dispute,
      client:1,
      tx:2,
      amount: None,
    };
    record_test.add_transaction(deposit_1);
    record_test.add_transaction(deposit_2);
    record_test.add_transaction(dispute);
    let transaction = record_test.process_transactions();
    assert_eq!(transaction.total, 12.000_f64);
    assert_eq!(transaction.held, 2.0_f64);
    assert_eq!(transaction.available, 10.000_f64);
    assert_eq!(transaction.locked, false);
  }
  #[test]
  fn test_resolve(){
    let mut record_test = TransactionRecord::new(1);

    let deposit_1 = TransactionInput {
      transaction_type: TransactionTypes::Deposit,
      client:1,
      tx:1,
      amount: Some(10.0_f64),
    };

    let deposit_2 = TransactionInput {
      transaction_type: TransactionTypes::Deposit,
      client:1,
      tx:2,
      amount: Some(2.0_f64),
    };
    let dispute = TransactionInput {
      transaction_type: TransactionTypes::Dispute,
      client:1,
      tx:2,
      amount: None,
    };
    let resolve = TransactionInput {
      transaction_type: TransactionTypes::Resolve,
      client:1,
      tx:2,
      amount: None,
    };
    record_test.add_transaction(deposit_1);
    record_test.add_transaction(deposit_2);
    record_test.add_transaction(dispute);
    record_test.add_transaction(resolve);
    let transaction = record_test.process_transactions();
    assert_eq!(transaction.total, 12.000_f64);
    assert_eq!(transaction.held, 0.0_f64);
    assert_eq!(transaction.available, 12.000_f64);
    assert_eq!(transaction.locked, false);
  }
  #[test]
  fn test_chargeback(){
    let mut record_test = TransactionRecord::new(1);

    let deposit_1 = TransactionInput {
      transaction_type: TransactionTypes::Deposit,
      client:1,
      tx:1,
      amount: Some(10.0_f64),
    };

    let deposit_2 = TransactionInput {
      transaction_type: TransactionTypes::Deposit,
      client:1,
      tx:2,
      amount: Some(2.0_f64),
    };
    let dispute = TransactionInput {
      transaction_type: TransactionTypes::Dispute,
      client:1,
      tx:2,
      amount: None,
    };
    let chargeback = TransactionInput {
      transaction_type: TransactionTypes::Chargeback,
      client:1,
      tx:2,
      amount: None,
    };
    record_test.add_transaction(deposit_1);
    record_test.add_transaction(deposit_2);
    record_test.add_transaction(dispute);
    record_test.add_transaction(chargeback);
    let transaction = record_test.process_transactions();
    assert_eq!(transaction.total, 10.000_f64);
    assert_eq!(transaction.held, 0.0_f64);
    assert_eq!(transaction.available, 10.000_f64);
    assert_eq!(transaction.locked, true);
  }

  #[test]
  fn test_ignore_chargeback_when_no_dispute(){
    let mut record_test = TransactionRecord::new(1);

    let deposit_1 = TransactionInput {
      transaction_type: TransactionTypes::Deposit,
      client:1,
      tx:1,
      amount: Some(10.0_f64),
    };

    let deposit_2 = TransactionInput {
      transaction_type: TransactionTypes::Deposit,
      client:1,
      tx:2,
      amount: Some(2.0_f64),
    };
    let chargeback = TransactionInput {
      transaction_type: TransactionTypes::Chargeback,
      client:1,
      tx:2,
      amount: None,
    };
    record_test.add_transaction(deposit_1);
    record_test.add_transaction(deposit_2);
    record_test.add_transaction(chargeback);
    let transaction = record_test.process_transactions();
    assert_eq!(transaction.total, 12.000_f64);
    assert_eq!(transaction.held, 0.0_f64);
    assert_eq!(transaction.available, 12.000_f64);
    assert_eq!(transaction.locked, false);
  }
}