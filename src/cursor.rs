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

    pub fn set_value(&mut self, value: usize) -> bool {
        if value > self.capacity {
            false
        } else {
            self.index = value;
            true
        }
    }

    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
        if self.index >= self.capacity {
            self.index = self.capacity - 1;
        }
    }

    pub fn increase(&mut self) -> bool {
        if self.capacity == 0 {
            return false;
        };
        if self.index == self.capacity - 1 {
            if self.rotation {
                self.index = 0;
                true
            } else {
                false
            }
        } else {
            self.index += 1;
            true
        }
    }

    pub fn decrease(&mut self) -> bool {
        if self.capacity == 0 {
            return false;
        };
        if self.index == 0 {
            if self.rotation {
                self.index = self.capacity - 1;
                true
            } else {
                false
            }
        } else {
            self.index -= 1;
            true
        }
    }
}

/// State of a cursor
///
/// You can convert cursor state into an option with "value" method
#[derive(Debug, PartialEq)]
pub enum CursorState<'container, T> {
    MinOut,
    EmptyContainer,
    Valid(&'container T),
    OutOfRange,
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
