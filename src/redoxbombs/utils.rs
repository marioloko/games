/// Container types which can have empty positions.
///
/// This trait provides an interface to insert elements
/// into the first empty position.
pub trait InsertAtEmpty {
    type Item;

    /// Look for the first empty position in the container
    /// and insert item at that position.
    ///
    /// It returns the insertion index.
    fn insert_at_empty(&mut self, item: Self::Item) -> usize;
}

/// Implementation of InsertAtEmpty for Vec<Option<T>>,
/// empty positions are None, or the end of the Vector.
impl<T> InsertAtEmpty for Vec<Option<T>> {
    type Item = T;

    /// Insert the item `item` in the position of the
    /// first `None` element, which represent empty positions,
    /// or at the end of the `Vector`, if there are no `None`s.
    ///
    /// It returns the insertion index.
    fn insert_at_empty(&mut self, item: Self::Item) -> usize {
        // Get the first `None` index.
        match self.iter().position(Option::is_none) {
            Some(insertion_index) => {
                // Replace `None` with item.
                self[insertion_index].replace(item);
                insertion_index
            }
            None => {
                // Inserted at the end of the array.
                let insertion_index = self.len();
                let item = Some(item);
                self.push(item);

                insertion_index
            }
        }
    }
}
