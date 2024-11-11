use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
use cursive::traits::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, ListView};
use cursive::{Cursive, CursiveExt};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Product {
    product_type: String,
    quantity: usize,
    price_per_unit: f64,
    sales_tax: f64,
    total_price: f64,
}

// db file
const FILE_PATH: &str = "inventory.json";

fn save_products_to_file(products: &Vec<Product>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)?;
    serde_json::to_writer(file, products)?;
    Ok(())
}

fn load_products_from_file() -> Vec<Product> {
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

fn main() {
    let mut siv = Cursive::default();
    siv.set_theme(custom_theme());

    let products = Arc::new(Mutex::new(load_products_from_file()));

    siv.add_layer(Dialog::new()
        .title("Inventory management")
        .content(ListView::new()
            .child("Product Type: ", EditView::new().with_name("product_type"))
            .child("Quantity: ", EditView::new().with_name("quantity"))
            .child("Price per Unit: ", EditView::new().with_name("price_per_unit"))
        )
        .button("Save", {
            let product_clone = Arc::clone(&products);
            move |siv| {
                let product_type = siv.call_on_name("product_type", |view: &mut EditView| {
                    view.get_content()
                }).unwrap().to_string();

                let quantity = siv.call_on_name("quantity", |view: &mut EditView| {
                    view.get_content()
                }).unwrap().parse::<usize>().unwrap_or(0);

                let price_per_unit = siv.call_on_name("price_per_unit", |view: &mut EditView| {
                    view.get_content()
                }).unwrap().parse::<f64>().unwrap_or(0.0);

                if product_type.is_empty() {
                    siv.add_layer(Dialog::info("Error: Please enter a product type."));
                    return;
                }
                if quantity == 0 {
                    siv.add_layer(Dialog::info("Error: Please enter a valid quantity."));
                    return;
                }
                if price_per_unit == 0.0 {
                    siv.add_layer(Dialog::info("Error: Please enter a valid price per unit."));
                    return;
                }

                let sales_tax = 0.10 * price_per_unit;
                let total_price = (price_per_unit + sales_tax) * quantity as f64;

                let product = Product {
                    product_type,
                    quantity,
                    price_per_unit,
                    sales_tax,
                    total_price,
                };

                let mut product_store = product_clone.lock().unwrap();
                product_store.push(product.clone());
                // Save to file
                if let Err(err) = save_products_to_file(&product_store) {
                    siv.add_layer(Dialog::info(format!("Error: saving the product: {}", err)));
                } else {
                    siv.add_layer(Dialog::info("Product saved successfully!"))
                }
                siv.call_on_name("product_type", |view: &mut EditView| { view.set_content(""); });
                siv.call_on_name("quantity", |view: &mut EditView| { view.set_content(""); });
                siv.call_on_name("price_per_unit", |view: &mut EditView| { view.set_content(""); });
            }
        })
        .button("Show All", {
            let products = Arc::clone(&products);
            move |siv| {
                let product_store = products.lock().unwrap();
                let mut output = String::new();

                for (index, product) in product_store.iter().enumerate() {
                    output.push_str(&format!("{}. Item: {}, Qty: {}, Price: ${}, Sales Tax: ${:.2}, Total Price: ${:.2}\n",
                                             index + 1,
                                             product.product_type,
                                             product.quantity,
                                             product.price_per_unit,
                                             product.sales_tax,
                                             product.total_price,
                    ));
                }
                if output.is_empty() {
                    output = "No products in the inventory.".to_string();
                }
                siv.add_layer(Dialog::info(output));
            }
        })
        .button("Delete by ID", {
            let products = Arc::clone(&products);
            move |siv| {
                let id_input = EditView::new().with_name("delete_id").min_width(10);
                siv.add_layer(Dialog::new().title("Delete Product").content(ListView::new().child("Enter product Id to delete:", id_input))
                    .button("Confirm", {
                        let product_clone = Arc::clone(&products);
                        move |siv: &mut Cursive| {
                            let id_str = siv.call_on_name("delete_id", |view: &mut EditView| {
                                view.get_content()
                            }).unwrap().to_string();
                            // ID Parsing
                            if let Ok(id) = id_str.parse::<usize>() {
                                let mut product_store = product_clone.lock().unwrap();
                                if id > 0 && id <= product_store.len() {
                                    product_store.remove(id - 1);
                                    if let Err(err) = save_products_to_file(&product_store) {
                                        siv.add_layer(Dialog::info(format!("Error deleting the product: {}", err)));
                                    } else { siv.add_layer(Dialog::info("Product deleted")); }
                                } else { siv.add_layer(Dialog::info("No product found.")); }
                            } else {
                                siv.add_layer(Dialog::info("Error: Please enter a valid ID."));
                            }
                        }
                    })
                    .button("Cancel", |siv| {
                        siv.pop_layer();
                    })
                );
                siv.call_on_name("delete_id", |view: &mut EditView| { view.set_content(""); });
            }
        })
        .button("Quit", |siv: &mut Cursive| siv.quit()));
    siv.run();
}
fn custom_theme() -> Theme {
    let mut theme = Theme::retro();
    theme.palette[PaletteColor::Background] = Color::Light(BaseColor::Cyan);
    theme.palette[PaletteColor::View] = Color::Light(BaseColor::White);
    theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Secondary] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Highlight] = Color::Light(BaseColor::Green);
    theme
}
