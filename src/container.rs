use crate::result;
use crate::{cursor::Cursor, cursor::CursorState, OpResult};
use std::ops::{Deref, DerefMut};

/// Vector container with inner cursor variable
///
/// Cursor operation returns cursor state which indicates whether the operation succeeded or not.
///
/// User can also modify vector container manually but needs to always check cursor is valid in newer context.
///
/// # Usage
///
/// ```rust
/// use cursorvec::CursorVec;
///
/// let mut vec = CursorVec::new()
///     .with_container(vec!["first", "second", "third", "fourth", "fifth"]);
///
/// assert_eq!(Some(&"first"), vec.get_current().value());
/// ```

#[derive(Debug)]
pub struct CursorVec<T> {
    vector: Vec<T>,
    cursor: Cursor,
}

impl<T> Deref for CursorVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.vector
    }
}

impl<T> DerefMut for CursorVec<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.vector
    }
}

impl<T> Default for CursorVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> CursorVec<T> {
    /// Constructor
    ///
    /// Combine with builder methods to configure container with single method chains.
    ///
    /// # Usage
    /// ```rust
    /// use cursorvec::CursorVec;
    ///
    /// let mut vec = CursorVec::new()
    ///     .rotatable(true) // Set cursor rotatable
    ///     .with_container(vec![1,2,3,4,5,]); // Assign container
    ///
    /// // You can set properties later
    /// vec.set_rotatable(false);
    /// vec.set_container(vec![10,11,23]);
    /// ```
    pub fn new() -> Self {
        Self {
            vector: vec![],
            cursor: Cursor::new(0),
        }
    }

    /// Build cursorvec with container
    ///
    /// This gets argument as vector not an array because container type doesn't require copy trait
    pub fn with_container(mut self, vector: Vec<T>) -> Self {
        self.vector = vector;
        self.cursor.set_capacity(self.vector.len());
        self
    }

    /// Build cursorvec with rotatable cursor
    pub fn rotatable(mut self, rotatable: bool) -> Self {
        self.cursor.set_rotation(rotatable);
        self
    }

    /// Set cursor rotatable
    pub fn set_rotatable(&mut self, rotatable: bool) {
        self.cursor.set_rotation(rotatable);
    }

    /// Set container
    pub fn set_container(&mut self, container: Vec<T>) {
        self.vector = container;
        self.update_cursor();
    }

    /// Modify inner container with given closure
    ///
    /// This method automatically calls update_cursor after every closure call.
    ///
    /// # Usage
    ///
    /// ```rust
    /// use cursorvec::CursorVec;
    ///
    /// let mut vec = CursorVec::new().with_container(vec![1,2,3]);
    /// vec.set_cursor(2);
    ///
    /// // Removes 2 and 3 from the vector and automatically update cursor bounds
    /// vec.modify(|cont| {cont.drain(1..);});
    ///
    /// // Now cursor is set to 0
    /// assert_eq!(Some(0), vec.get_cursor());
    /// ```
    pub fn modify<F: Fn(&mut Vec<T>)>(&mut self, f: F) {
        f(&mut self.vector);
        self.update_cursor();
    }

    /// Update cursor's state
    ///
    /// This is usually not necessary but mostly useful when user has modified inner container's
    /// values that possibly changes cursors range bounds.
    ///
    /// # Usage
    ///
    ///
    /// ```rust
    /// use cursorvec::CursorVec;
    ///
    /// let mut vec = CursorVec::new().with_container(vec![1,2,3,4,5,6]);
    /// vec.set_cursor(vec.len() - 1);
    ///
    /// // Removes odd numbers
    /// vec.retain(|num| num % 2 == 0);
    /// // Update cursor bounds
    /// vec.update_cursor();
    ///
    /// // Now cursor is set to 2
    /// assert_eq!(Some(2), vec.get_cursor());
    /// ```
    pub fn update_cursor(&mut self) {
        self.cursor.set_capacity(self.vector.len());
    }

    /// Try move cursor to next and get cursor's value
    pub fn move_next_and_get(&mut self) -> CursorState<T> {
        if self.is_empty_container() {
            return CursorState::EmptyContainer;
        }
        if !result::is_true(self.cursor.increase()) {
            return CursorState::MaxOut;
        }
        match self.get_cursor_value() {
            Some(v) => CursorState::Valid(v),
            None => CursorState::OutOfRange,
        }
    }

    /// Try move cursor to next and always get value
    ///
    /// If the container is empty it returns none
    /// If the state is maxout, this will return valid cursor's value
    #[allow(unused_must_use)]
    pub fn move_next_and_get_always(&mut self) -> Option<&T> {
        if self.is_empty_container() {
            return None;
        }
        self.cursor.increase();
        self.get_cursor_value()
    }

    /// Try move cursor to next nth times and get cursor's value
    pub fn move_next_nth_and_get(&mut self, amount: usize) -> CursorState<T> {
        if self.is_empty_container() {
            return CursorState::EmptyContainer;
        }
        for _ in 0..amount {
            if !result::is_true(self.cursor.increase()) {
                return CursorState::MaxOut;
            }
        }
        match self.get_cursor_value() {
            Some(v) => CursorState::Valid(v),
            None => CursorState::OutOfRange,
        }
    }

    /// Try move cursor to next nth times and always get value
    ///
    /// If the container is empty it returns none
    /// If the state is maxout, this will return valid cursor's value
    pub fn move_next_nth_and_get_always(&mut self, amount: usize) -> Option<&T> {
        if self.is_empty_container() {
            return None;
        }
        for _ in 0..amount {
            if !result::is_true(self.cursor.increase()) {
                // Early return
                return self.get_cursor_value();
            }
        }
        self.get_cursor_value()
    }

    /// Try move cursor to previous and get cursor's value
    pub fn move_prev_and_get(&mut self) -> CursorState<T> {
        if self.is_empty_container() {
            return CursorState::EmptyContainer;
        }
        if !result::is_true(self.cursor.decrease()) {
            return CursorState::MinOut;
        }
        match self.get_cursor_value() {
            Some(v) => CursorState::Valid(v),
            None => CursorState::OutOfRange,
        }
    }

    /// Try move cursor to previous and always get value
    ///
    /// If the container is empty it returns none
    /// If the state is minout, this will return valid cursor's value
    #[allow(unused_must_use)]
    pub fn move_prev_and_get_always(&mut self) -> Option<&T> {
        if self.is_empty_container() {
            return None;
        }
        self.cursor.decrease();
        self.get_cursor_value()
    }

    /// Try move cursor to previous nth times and get cursor's value
    pub fn move_prev_nth_and_get(&mut self, amount: usize) -> CursorState<T> {
        if self.is_empty_container() {
            return CursorState::EmptyContainer;
        }
        for _ in 0..amount {
            if !result::is_true(self.cursor.decrease()) {
                return CursorState::MinOut;
            }
        }
        match self.get_cursor_value() {
            Some(v) => CursorState::Valid(v),
            None => CursorState::OutOfRange,
        }
    }

    /// Try move cursor to previous about nth times and always get value
    ///
    /// If the container is empty it returns none
    /// If the state is minout, this will return valid cursor's value
    pub fn move_prev_nth_and_get_always(&mut self, amount: usize) -> Option<&T> {
        if self.is_empty_container() {
            return None;
        }
        for _ in 0..amount {
            if !result::is_true(self.cursor.decrease()) {
                // Early return
                return self.get_cursor_value();
            }
        }
        self.get_cursor_value()
    }

    /// Get current cursor's value
    pub fn get_current(&self) -> CursorState<T> {
        if self.is_empty_container() {
            return CursorState::EmptyContainer;
        }
        match self.get_cursor_value() {
            Some(v) => CursorState::Valid(v),
            None => CursorState::OutOfRange,
        }
    }

    /// Move cursor to next
    pub fn move_next(&mut self) -> OpResult {
        if self.is_empty_container() {
            return result::error("empty container");
        }

        self.cursor.increase()
    }

    /// Move cursor to previous
    pub fn move_prev(&mut self) -> OpResult {
        if self.is_empty_container() {
            return result::error("empty container");
        }
        self.cursor.decrease()
    }

    // <Manual> Methods
    /// Get cursor index
    ///
    /// This returns none when container is empty
    pub fn get_cursor(&self) -> Option<usize> {
        if self.vector.is_empty() {
            None
        } else {
            Some(self.cursor.get_value())
        }
    }

    /// Set cursor with manual index
    pub fn set_cursor(&mut self, cursor: usize) -> OpResult {
        self.cursor.set_value(cursor)
    }
    // </Manual> Methods

    // <DRY> Codes

    fn get_cursor_value(&self) -> Option<&T> {
        self.vector.get(self.cursor.get_value())
    }

    fn is_empty_container(&self) -> bool {
        self.vector.is_empty()
    }
    // </DRY>
}
