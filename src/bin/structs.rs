struct GroceryItem {
    stock: i32,
    price: f64, //float 64 bits aka double
}
fn main() {
    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };
    println!("stock {:?}", cereal.stock);
    println!("price {:?}", cereal.price);
}
