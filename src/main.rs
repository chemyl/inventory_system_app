use cursive::views::{Dialog, TextView, ListView, EditView};
use cursive::{Cursive, CursiveExt};
use cursive::traits::{Nameable,Resizable};
use std::sync::{Arc, Mutex};
use std::fs::{File, OpenOptions};
use std::io::{self,Read};
use serde::{Serialize, Deserialize};

// derive - макрос для автоматической генерации кода определенных трейтов для структуры
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Products{
    id: u32,
    name: String,
    quantity: usize,
    price_per_unit: f64,
    sales_tax: f64,
    total_price:f64,
}

// db file
const FILE_PATH: &str = "inventory.json";

fn save_products_to_file(products: &Vec<Products>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(FILE_PATH)?;
    serde_json::to_writer(file,products)?;
    Ok(())
}

fn load_products_from_file() -> Vec<Products>{
    if let Ok(mut file) = File::open(FILE_PATH){
        let mut data = String::new();
        if file.read_to_string(&mut data).is_ok(){
            if let Ok(products) = serde_json::from_str::<Vec<Products>>(&data){
                return products;
            }
        }
    }
    Vec::new()
}

fn main() {
    println!("Hello, world!");

}
