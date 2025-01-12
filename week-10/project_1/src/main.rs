struct Laptop{
    brand:String,
    price:u32,    //Price of  one laptop
    quantity:u32, // Quantity of laptops purchased
    inventory:u32,   // Total laptops recieved by Mr Ogbeifuna
}

impl Laptop {
    fn total_cost(&self) -> u32 {
        if self.quantity > self.inventory {
            println!(
                "Error: Not enough inventory for {}. Only {} available.",
                self.brand, self.inventory
            );
            0 // Return 0 cost if purchase exceeds inventory
        } else {
            self.price * self.quantity
        }
    }
}


fn main() {
    let hp = Laptop {
         brand:String::from("HP"),
         price:650_000,
         quantity:3,
         inventory:10,
    };

    let ibm = Laptop{
        brand:String::from("IBM"),
        price: 755_000,
        quantity: 3,
        inventory: 6,
    };

    let toshiba = Laptop{
        brand: String::from("Toshiba"),
        price: 550_000,
        quantity: 3,
        inventory: 10
    };

    let dell = Laptop {
        brand:String::from("DELL"),
        price: 850_000,
        quantity:3,
        inventory: 4, 
    };

    let total_cost = hp.total_cost() + ibm.total_cost() + toshiba.total_cost() + dell.total_cost();
    println!("The total cost for this purchase is {}", total_cost);
}