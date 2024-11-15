use cursive::style::{BaseColor, Color, PaletteColor};
use cursive::theme::Theme;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::{File, OpenOptions};
use std::io::{self, Read};

const FILE_PATH: &str = "products.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub product_name: String,
    pub quantity: usize,
    pub price_per_unit: f64,
    pub sales_tax: f64,
    pub total_price: f64,
}

pub fn save_products_to_file(products: &Vec<Product>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)?;
    serde_json::to_writer(file, products)?;
    Ok(())
}

pub fn load_products_from_file() -> Vec<Product> {
    match File::open(FILE_PATH) {
        Ok(mut file) => {
            let mut data = String::new();
            match file.read_to_string(&mut data) {
                Ok(_) => {
                    match serde_json::from_str::<Vec<Product>>(&data) {
                        Ok(products) => products,
                        Err(e) => {
                            eprintln!("Failed to parse JSON: {}", e);
                            Vec::new()
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read file content: {}", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            Vec::new()
        }
    }
}

pub fn delete_product_from_file(product_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().read(true).open(FILE_PATH)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let mut products: Vec<Product> = serde_json::from_str(&data)?;

    products.retain(|product| product.product_name != product_name);
    let file = OpenOptions::new().write(true).truncate(true).open(FILE_PATH)?;
    serde_json::to_writer(file, &products)?;
    Ok(())
}

pub fn custom_theme() -> Theme {
    let mut theme = Theme::retro();
    theme.palette[PaletteColor::Background] = Color::Light(BaseColor::Cyan);
    theme.palette[PaletteColor::View] = Color::Light(BaseColor::White);
    theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Secondary] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Highlight] = Color::Light(BaseColor::Green);
    theme
}