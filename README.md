# CursorVec

Vec container that utilizes "Cursor"

[Changes](docs/changes.md)

## Usage

```rust
use cursorvec::*;

let mut vec =
    CursorVec::new().with_container(vec!["first", "second", "third", "fourth", "fifth"]);

// Move cursor to next and get cursor's value
assert_eq!(Some(&"first"), vec.get_current().value());
assert_eq!(Some(&"second"), vec.move_next_and_get().value());
assert_eq!(Some(&"fifth"), vec.move_next_nth_and_get(3).value());
assert_eq!(CursorState::MaxOut, vec.move_next_and_get());

// Move cursor to prevous and get cursor's value
assert_eq!(Some(&"fourth"), vec.move_prev_and_get().value());
assert_eq!(Some(&"first"), vec.move_prev_nth_and_get(3).value());
assert_eq!(CursorState::MinOut, vec.move_prev_and_get());

// Reset cursor
vec.set_cursor(0);

// Move cursor and tries to get values regardless of cursor success
assert_eq!(Some(&"fifth"), vec.move_next_nth_and_get_always(10000));
assert_eq!(Some(&"first"), vec.move_prev_nth_and_get_always(10000));

// Container with rotating cursor
let mut vec = CursorVec::new()
    .rotatable(true)
    .with_container(vec![1, 2, 3, 4, 5, 6, 7, 8]);

assert_eq!(Some(&3), vec.move_next_nth_and_get(10).value());
// always is not so differnt from non-always variant if rotation is set
assert_eq!(Some(&7), vec.move_next_nth_and_get_always(4));

// Modify container and update cursor
vec.drain(5..);
vec.update_cursor();

// Cursor automatically goes to available index
assert_eq!(Some(&5), vec.get_current().value());

// Modify without update can possibly cause out of range error
vec.drain(1..);
assert_eq!(CursorState::OutOfRange, vec.get_current());

vec.update_cursor();
assert_ne!(CursorState::OutOfRange, vec.get_current());

vec.set_container(vec![1, 2, 3, 4, 5, 6, 7, 8]);
vec.set_cursor(6);
assert_eq!(Some(&7), vec.get_current().value());

// Use modify method to auto update cursor
vec.modify(|cont| cont.retain(|num| *num % 2 == 0));

//             Cursor
//               |
// vec![2, 4, 6, 8]
assert_eq!(Some(3), vec.get_cursor());
assert_eq!(Some(&8), vec.get_current().value());
```

## TODO

* [ ] Feature : Panickable operation feature rather than bool return.
- This enables easier error handling with cost of possible panicking behaviour
