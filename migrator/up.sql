CREATE TABLE available_goods(
    uuid VARCHAR(30),
    name VARCHAR(40),
    PRIMARY KEY(uuid),
);

CREATE TABLE goods_storage(
    uuid VARCHAR(30),
    volumes DECIMAL(10,4),
    PRIMARY KEY (uuid),
    FOREIGN KEY fk_uuid REFERENCES available_goods(uuid)
);

CREATE TABLE good_property(
    uuid VARCHAR(30),
);
