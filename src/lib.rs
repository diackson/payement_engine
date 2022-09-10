mod transaction_module;
use transaction_module::{TransactionOperationRecord, TransactionInput};
use std::error;
use std::io;

pub type Error = Box<dyn error::Error + Sync + Send>;

pub fn read_transaction_file<T: io::Read>(mut reader: csv::Reader<T>) -> Result<(), Error> {
  let mut transaction_operation_record = TransactionOperationRecord::new();

  for input in reader.deserialize() {
    let transaction: TransactionInput = input?;
    transaction_operation_record.add_transaction(transaction);
  }
  let mut csv_writer = csv::Writer::from_writer(io::stdout());
  transaction_operation_record
      .get_operation_record()
      .iter()
      .map(|(_, transaction_process)| transaction_process.process_transactions())
      .try_for_each(|t| csv_writer.serialize(t))?;

  
  Ok(())
}
