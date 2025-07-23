use crate::models::product::Product;
use std::sync::RwLock;
use lazy_static::lazy_static;

lazy_static! {
    static ref PRODUCTS: RwLock<Vec<Product>> = RwLock::new(Vec::new());

}


pub async fn get_products_in_price_range(min_price: i32, max_price: i32) -> Result<Vec<Product>, String> {
   
    let mut products = PRODUCTS.write().unwrap();
    
    products.push(Product { id: 1, name: "Product A".to_string(), price: 10.0 });
    products.push(Product { id: 2, name: "Product B".to_string(), price: 20.0 });
    products.push(Product { id: 3, name: "Product C".to_string(), price: 30.0 });


    let products = PRODUCTS.read().unwrap();
    let filtered_products: Vec<Product> = products.iter()
        .filter(|&product| product.price >= min_price as f64 && product.price <= max_price as f64)
        .cloned()
        .collect();

    if filtered_products.is_empty() {
        return Err("No products found in the specified price range".to_string());
    }

    Ok(filtered_products)
}