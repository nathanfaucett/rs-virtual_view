use std::sync::mpsc::Sender;

use super::super::Transaction;

pub trait Handler: 'static {
    fn handle(&self, Transaction);
}

impl Handler for Sender<Transaction> {
    #[inline]
    fn handle(&self, transaction: Transaction) {
        let _ = self.send(transaction).expect("failed to send transaction");
    }
}
