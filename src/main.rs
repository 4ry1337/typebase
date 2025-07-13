use sqlite_rust::app;

#[derive(Debug, Table("users"))]
pub struct UserTable {
    #[field(primary_key)]
    id: u32,
    username: String,
    #[field(unique)]
    email: String,
}

/* #[derive(Debug, Table("products"))]
pub struct ProductTable {
    #[field(primary_key)]
    id: u32,
    product_name: String,
    cost: u32,
}

#[derive(Debug, Table("orders"))]
pub struct OrderTable {
    #[field(primary_key)]
    id: u32,
    #[field(foreign_key(table = "users", column = "id"))]
    user_id: u32,
    #[field(foreign_key("products.id"))]
    product_id: u32,
} */

fn main() {
    let mut app = app::App::new();
    app.add_table(UserTable);
    app.run()
}
