use crate::move_::Move;
/// Represents a transaction.
pub struct Transaction<Unit, SumNumber, MoveMeta, Meta>
where
    Unit: Ord,
{
    pub(crate) metadata: Meta,
    pub(crate) moves: Vec<Move<Unit, SumNumber, MoveMeta>>,
}
/// Used to index moves in a transaction.
pub struct MoveIndex(pub usize);
impl<Unit, SumNumber, MoveMeta, Meta>
    Transaction<Unit, SumNumber, MoveMeta, Meta>
where
    Unit: Ord,
{
    /// Gets an iterator of existing moves in their order.
    pub fn moves(
        &self,
    ) -> impl Iterator<Item = (MoveIndex, &Move<Unit, SumNumber, MoveMeta>)>
    {
        self.moves
            .iter()
            .enumerate()
            .map(|(index, move_)| (MoveIndex(index), move_))
    }
    /// Gets the metadata of the transaction.
    pub fn metadata(&self) -> &Meta {
        &self.metadata
    }
}

#[cfg(test)]
mod test {
    use super::{MoveIndex, Transaction};
    use crate::book::TransactionIndex;
    use crate::test_utils::TestUnit;
    #[test]
    fn moves() {
        let mut book = test_book!("");
        book.insert_transaction(TransactionIndex(0), "");
        let debit_account_key = book.new_account("");
        let credit_account_key = book.new_account("");
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_account_key,
            credit_account_key,
            sum!(),
            "a",
        );
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(1),
            debit_account_key,
            credit_account_key,
            sum!(),
            "b",
        );
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(0),
            debit_account_key,
            credit_account_key,
            sum!(),
            "c",
        );
        book.insert_move(
            TransactionIndex(0),
            MoveIndex(2),
            debit_account_key,
            credit_account_key,
            sum!(),
            "d",
        );
        assert_eq!(
            book.transactions()
                .next()
                .unwrap()
                .1
                .moves()
                .map(|(_move_index, move_)| move_.metadata())
                .collect::<Vec<_>>(),
            vec![&"c", &"a", &"d", &"b"],
        );
    }
    #[test]
    fn metadata() {
        let transaction = Transaction::<TestUnit, u8, (), &str> {
            metadata: "deposit",
            moves: Vec::new(),
        };
        assert_eq!(transaction.metadata(), &"deposit",);
    }
}
