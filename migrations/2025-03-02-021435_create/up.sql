-- Your SQL goes here
CREATE TABLE available_goods(
    good_id INT AUTO_INCREMENT,
    name VARCHAR(40) NOT NULL,
    PRIMARY KEY(good_id)
);

CREATE TABLE attributes(
	attribute_id INT AUTO_INCREMENT,
    good_id INT NOT NULL,
    contents JSON NOT NULL,
    PRIMARY KEY (attribute_id),
    FOREIGN KEY (good_id) REFERENCES available_goods(good_id)
);

CREATE TABLE standard_keep_units (
	sku_id INT AUTO_INCREMENT,
	good_id INT NOT NULL,
	price DECIMAL(10, 2) NOT NULL,
	volumes DECIMAL(10, 2) NOT NULL DEFAULT 0,
	PRIMARY KEY (sku_id),
	FOREIGN KEY (good_id) REFERENCES available_goods(good_id)
);

CREATE TABLE discount (
	discount_id INT AUTO_INCREMENT,
	good_id INT NOT NULL,
	discount_type VARCHAR(10) NOT NULL DEFAULT 'None',
	value DECIMAL(10, 2) NOT NULL DEFAULT 0,
	PRIMARY KEY (discount_id),
	FOREIGN KEY (good_id) REFERENCES available_goods(good_id)
);

CREATE VIEW good_property AS
SELECT name, price, volumes, discount_type, value
FROM available_goods g 
INNER JOIN standard_keep_units sku 
ON g.good_id = sku.good_id
INNER JOIN discount d
ON g.good_id = d.good_id;