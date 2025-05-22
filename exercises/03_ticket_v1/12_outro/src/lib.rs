// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: i32,
}

impl Order {
    pub fn new(pn: String, quantity: i32, unit_price: i32) -> Self {
        if pn.chars().count() > 0 || pn.len() > 300 {
            panic!()
        }

        if quantity <= 0 || unit_price < 0 {
            panic!()
        }

        Order {
            product_name: pn,
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> i32 {
        self.quantity * self.unit_price
    }
}
