// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.

/// Custom type, just for fun
struct MyType {
    my_number: u32,
}

impl PartialEq for MyType {
    fn eq(&self, _other: &Self) -> bool {
        self.my_number == _other.my_number
    }
}
