CREATE TABLE "items"
(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    quantity INT NOT NULL,
    category VARCHAR NOT NULL,
    inventory_id INT NOT NULL,
    FOREIGN KEY (inventory_id) REFERENCES Inventories(id) 
)