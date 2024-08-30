struct Customer {
    customer_id: u32,
    name: String,
    sur_name: String,
    balance: f64,
    email: String,
    buying_history: Vec<String>,
}

struct Product {
    product_id: u32,
    name: String,
    price: f64,
    stk_qnty: u32,
    ctr: String,
    desc: String,
    discount: f64,
}

impl Customer {
    fn buy_product(&mut self, product: &mut Product, quantity: u32) -> bool {
        let discounted_price = product.price * (1.0 - product.discount / 100.0);
        let total_cost = discounted_price * quantity as f64;

        if self.balance >= total_cost && product.stk_qnty >= quantity {
            self.balance -= total_cost;
            product.stk_qnty -= quantity;
            self.buying_history.push(format!("{} x {} (ID: {})", quantity, product.name, product.product_id));
            println!(
                "{} {} (ID: {}) purchased {} x {} (ID: {}) for ${:.2} each (total: ${:.2}). Product Description: {}",
                self.name, self.sur_name, self.customer_id, quantity, product.name, product.product_id, discounted_price, total_cost, product.desc
            );
            true
        } else {
            println!(
                "{} {} could not afford {} x {} (ID: {}) or it is out of stock.",
                self.name, self.sur_name, quantity, product.name, product.product_id
            );
            false
        }
    }

    fn display_balance(&self) {
        println!(
            "{} {}'s remaining balance: ${:.2} (Email: {})",
            self.name, self.sur_name, self.balance, self.email
        );
    }

    fn display_buying_history(&self) {
        println!("{} {}'s buying history:", self.name, self.sur_name);
        for purchase in &self.buying_history {
            println!("- {}", purchase);
        }
    }
}

fn main() {
    let mut customer1 = Customer {
        customer_id: 1035,
        name: String::from("Taylor"),
        sur_name: String::from("Swift"),
        balance: 250.0,
        email: String::from("taylor.alison.swift@somerandommail.com"),
        buying_history: Vec::new(),
    };
    let mut customer2 = Customer {
        customer_id: 1036,
        name: String::from("Ed"),
        sur_name: String::from("Sheeran"),
        balance: 75.0,
        email: String::from("Ed.Sheeran@somerandommail.com"),
        buying_history: Vec::new(),
    };
    let mut product1 = Product {
        product_id: 101,
        name: String::from("Electric Guitar"),
        price: 43.0,
        stk_qnty: 7,
        ctr: String::from("Electric"),
        desc: String::from("An electric guitar with a sleek design and high-quality sound, perfect for rock and blues."),
        discount: 10.0,
    };

    let mut product2 = Product {
        product_id: 102,
        name: String::from("Acoustic Guitar"),
        price: 29.0,
        stk_qnty: 15,
        ctr: String::from("Acoustic"),
        desc: String::from("An acoustic guitar with a warm, resonant sound, perfect for folk and classical music."),
        discount: 0.0,
    };

    customer1.buy_product(&mut product1, 1);
    customer2.buy_product(&mut product2, 2);
    customer2.buy_product(&mut product1, 1);

    customer1.display_balance();
    customer2.display_balance();

    customer1.display_buying_history();
    customer2.display_buying_history();

    println!("\nRemaining stock of {} ({}): {}", product1.name, product1.ctr, product1.stk_qnty);
    println!("Remaining stock of {} ({}): {}", product2.name, product2.ctr, product2.stk_qnty);
}
