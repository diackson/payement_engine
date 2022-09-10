# Payment engine

We'd like you to implement a simple toy payments engine that reads a series of transactions
from a CSV, updates client accounts, handles disputes and chargebacks, and then outputs the
state of clients accounts as a CSV.

## Details
Given a CSV representing a series of transactions, implement a simple toy transactions engine
that processes the payments crediting and debiting accounts. After processing the complete set
of payments output the client account balances

## Run the projet
Clone the repository and then build and run the app payement engine.
Enter the command

`cargo run -- transactions.csv > accounts.csv`

## Test

Run unit test by typing the command

`cargo test`

## Project Structure
The project is organized as a single module, regrouping all the transaction operation.
We used serde vrate for serialization and deserialization and csv crates to read our input file.
This will ensure readability, reusability, refactorable tu the code.

## Input 

The input will be a CSV file with the columns type, client, tx, and amount. You can assume the
type is a string, the client column is a valid u16 client ID, the tx is a valid u32 transaction ID, and
the amount is a decimal value with a precision of up to four places past the decimal.
## Output

The output should be a list of client IDs (client), available amounts (available), held amounts
(held), total amounts (total), and whether the account is locked (locked). Columns are defined
as

## Assumptions

T- he project covers all the operation required for a transaction operation

- It take an Input csv file, read and process dat and then write the results into an Output csv file

- I choose to set the project structure as a module to ensure readability, reusability, refactorability and maintaiability to the code. This way we can make changes, refactor any function without breaking the entire app.

I've made unit test on some functioion that i assume critic .
## Error Handeling

- I use Rust way of handeling errors to handle errors .

- I/O erros are printed to stderr

- Use Result<T,E> to handle erros on file reading and pars process.

But it can be improved, to set a more efficient error display.

## Improvements

- Add integration tests.
- Given a larger dataset, it would be better to persist data into datastore or database , in order to query them for the opération.
- To handle data concurency, we can use multithreading to split the transactions runing operation 
- If this code was bundle to a server, and CSVs came from concurrent TCP streams, we can use multithreading also to handle the concurrency and/or be asyncronous operation.
  












