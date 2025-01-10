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
    pub unit_price: i32
}

impl Order {
    pub fn new(init_product_name: String, init_quantity: i32, init_unit_price: i32) -> Self {

        if init_product_name.is_empty() {
            panic!("The product name can't be empty");
        } else if init_product_name.len() > 300 {
            panic!("The product name can't be longer than 300 bytes");
        }

        if init_quantity <= 0 {
            panic!("The quantity must be strictly greater than zero");
        }

        if init_unit_price <= 0 {
            panic!("The unit price must be strictly greater than zero")
        }

        Self {
            product_name: init_product_name,
            quantity: init_quantity,
            unit_price: init_unit_price
        }
    }

    pub fn product_name(&self) -> String {
        self.product_name.clone()
    }

    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, new_name: String) {
        if new_name.is_empty() {
            panic!("The product name can't be empty");
        } else if new_name.len() > 300 {
            panic!("The product name can't be longer than 300 bytes");
        }

        self.product_name = new_name;
    }

    pub fn set_quantity(&mut self, quan: i32) {
        if quan <= 0 {
            panic!("The quantity must be strictly greater than zero");
        }

        self.quantity = quan;
    }

    pub fn set_unit_price(&mut self, new_unit_price: i32) {
        if new_unit_price <= 0 {
            panic!("The unit price must be strictly greater than zero")
        }

        self.unit_price = new_unit_price;
    }

    pub fn total(&self) -> i32 {
        self.unit_price * self.quantity
    }
}