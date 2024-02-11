Budget Manager Rust
========================

1. **Categories Table:**
   - **Columns:**
     - `category_id` (Primary Key)
     - `name` (Category name, e.g., groceries, utilities, entertainment)

   ```sql
   CREATE TABLE categories (
       category_id INTEGER PRIMARY KEY,
       name TEXT NOT NULL
   );
   ```

2. **Transactions Table:**
   - **Columns:**
     - `transaction_id` (Primary Key)
     - `amount` (Amount of the transaction)
     - `category_id` (Foreign Key referencing `categories`)
     - `date` (Date of the transaction)
     - `description` (Additional notes or description for the transaction)

   ```sql
   CREATE TABLE transactions (
       transaction_id INTEGER PRIMARY KEY,
       amount REAL NOT NULL,
       category_id INTEGER NOT NULL,
       timestamp DATE NOT NULL,
       description TEXT,
       FOREIGN KEY (category_id) REFERENCES categories (category_id)
   );
   ```

   In this structure:
   - Each transaction is associated with a specific category.
   - The `category_id` in the `transactions` table is a foreign key that references the `category_id` in the `categories` table.

