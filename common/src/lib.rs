pub mod transaction;
pub mod constants;


#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn correct_transaction() {
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 42.0 as f64);

        assert_eq!(tx.amount(), 42.0 as f64);
        assert_eq!(tx.from(), "Alice".to_string());
        assert_eq!(tx.to(), "Bob".to_string());

    }
}
