# Payment engine

We'd like you to implement a simple toy payments engine that reads a series of transactions
from a CSV, updates client accounts, handles disputes and chargebacks, and then outputs the
state of clients accounts as a CSV.

## Details
Given a CSV representing a series of transactions, implement a simple toy transactions engine
that processes the payments crediting and debiting accounts. After processing the complete set
of payments output the client account balances

## Run the projet

to run the app payement engine, enter the command

`cargo run -- transactions.csv > accounts.csv`

## Test

Run unit test by typing the command

`cargo test`

## Project Structure
The project is organized as a single module, regrouping all the transaction operation.
We used serde vrate for serialization and deserialization and csv crates to read our input file.

## Input 

The input will be a CSV file with the columns type, client, tx, and amount. You can assume the
type is a string, the client column is a valid u16 client ID, the tx is a valid u32 transaction ID, and
the amount is a decimal value with a precision of up to four places past the decimal.
## Output

The output should be a list of client IDs (client), available amounts (available), held amounts
(held), total amounts (total), and whether the account is locked (locked). Columns are defined
as



