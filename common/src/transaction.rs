pub struct Transaction {
    from: String,
    to: String,
    amount: f64
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