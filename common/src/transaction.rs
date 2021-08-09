use serde::{Serialize, Deserialize};
use crate::keys::sha::{double_sha256, Sha256Hash};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64
}


impl Transaction {

    pub fn new(from: String, to: String, amount: f64) -> Self {
        Transaction {
            from,
            to,
            amount
        }
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn from(&self) -> &str {
        self.from.as_str()
    }

    pub fn to(&self) -> &str {
        self.to.as_str()
    }

    // Returns a double SHA-256 hash which denotes the transaction ID.
    // The transaction struct is deserialized to bincode which is then
    // hashed to ensure portability.
    pub fn tx_id(&self) -> Sha256Hash {
        let tx_de = bincode::serialize(self).unwrap();
        double_sha256(&tx_de)
    }

}

#[derive(Serialize, Deserialize)]
pub struct SignedTransaction {
    pub tx: Transaction,
    pub tx_id: Sha256Hash
}

impl SignedTransaction {
   pub fn new(tx: Transaction) -> SignedTransaction {
       let tx_id = tx.tx_id();
       SignedTransaction {
           tx: tx,
           tx_id: tx_id
       }
   }
}

#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;

    #[test]
    fn correct_transaction() {
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 42.0 as f64);

        assert_eq!(tx.amount(), 42.0 as f64);
        assert_eq!(tx.from(), "Alice".to_string());
        assert_eq!(tx.to(), "Bob".to_string());

    }
}