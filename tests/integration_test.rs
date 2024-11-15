use inventory_system_app::Product;
use inventory_system_app::{delete_product_from_file, load_products_from_file, save_products_to_file};

#[test]
fn test_save_and_load_products() {
    let products = vec![
        Product {
            product_name: "Example".to_string(),
            quantity: 10,
            price_per_unit: 20.0,
            sales_tax: 1.2,
            total_price: 212.0,
        },
    ];

    let result = save_products_to_file(&products);
    assert!(result.is_ok());

    let loaded_products = load_products_from_file();
    assert_eq!(loaded_products, products);

    let deleted_products = delete_product_from_file(&products[0].product_name);
    assert!(deleted_products.is_ok());
}