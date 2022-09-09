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
            dbg!(operation);
          }
        }
        TransactionTypes::Withdrawal => {
          if let Some(amount) = transaction.amount {
            operation.withdraw(amount);
            dbg!(operation);
          }
        }
        TransactionTypes::Dispute => {
          if let Some(existing_transaction) = self.find_transaction(transaction.tx) {
            operation.hold(existing_transaction.amount.unwrap());
            dbg!(operation);
          }
        }
        TransactionTypes::Resolve => {
          if let Some(existing_transaction) = self.find_transaction(transaction.tx) {
            operation.release(existing_transaction.amount.unwrap());
            dbg!(operation);
          }
        }
        TransactionTypes::Chargeback => {
          // Check if transaction to charge back is under dispute
          if let Some(_existing_transaction_dispute) = self.find_transaction_dispute(transaction.tx) {
           if  let Some(existing_transaction) = self.find_transaction(transaction.tx){
              operation.chargeback(existing_transaction.amount.unwrap());
              dbg!(operation);
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
