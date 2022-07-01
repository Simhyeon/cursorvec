use crate::result;
use crate::OpResult;

/// Cursor that points value
#[derive(Debug, PartialEq)]
pub(crate) struct Cursor {
    capacity: usize,
    rotation: bool,
    index: usize,
}

impl Cursor {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            rotation: false,
            index: 0,
        }
    }

    pub fn set_rotation(&mut self, rotation: bool) {
        self.rotation = rotation;
    }

    pub fn get_value(&self) -> usize {
        self.index
    }

    pub fn set_value(&mut self, value: usize) -> OpResult {
        if value > self.capacity {
            result::error("Cursor out of range")
        } else {
            self.index = value;
            result::ok()
        }
    }

    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
        if self.index >= self.capacity {
            self.index = self.capacity - 1;
        }
    }

    pub fn increase(&mut self) -> OpResult {
        if self.capacity == 0 {
            return result::error("Empty container");
        };
        if self.index == self.capacity - 1 {
            if self.rotation {
                self.index = 0;
                result::ok()
            } else {
                result::error("Cursor out of range")
            }
        } else {
            self.index += 1;
            result::ok()
        }
    }

    pub fn decrease(&mut self) -> OpResult {
        if self.capacity == 0 {
            return result::error("Empty container");
        };
        if self.index == 0 {
            if self.rotation {
                self.index = self.capacity - 1;
                result::ok()
            } else {
                result::error("Cursor out of range")
            }
        } else {
            self.index -= 1;
            result::ok()
        }
    }
}

/// State of a cursor
///
/// You can convert cursor state into an option with "value" method
#[derive(Debug, PartialEq)]
pub enum CursorState<'container, T> {
    /// Cursor cannot move previous because it reached minimal index (0)
    MinOut,
    /// Cursor cannot point anything because container is empty or cursor's information was not
    /// updated
    EmptyContainer,
    /// Cursor is in valid position. Valid cursor state refers a container value
    Valid(&'container T),
    /// Cursor is not in container's range. This is caused by container's manual modification
    /// without update
    OutOfRange,
    /// Cursor cannot move next because it reached maximal index (container.len() - 1)
    MaxOut,
}

impl<'container, T> CursorState<'container, T> {
    /// Try get value from cursor state
    ///
    /// This will return none if cursor is in invalid state.
    pub fn value(&self) -> Option<&T> {
        if let Self::Valid(val) = self {
            Some(*val)
        } else {
            None
        }
    }
}
