-- Your SQL goes here
CREATE TABLE residential_aggregate (
    community_id VARCHAR(255) NOT NULL PRIMARY KEY COMMENT '字符串唯一标识',
    name VARCHAR(255) NOT NULL,    -- 小区名称
    address VARCHAR(255) NOT NULL, -- 小区地址
    city VARCHAR(255) NOT NULL,    -- 城市
    state VARCHAR(255) NOT NULL    -- 省份
);

CREATE TABLE house_aggregate (
    `house_id` VARCHAR(255) NOT NULL PRIMARY KEY COMMENT '字符串唯一标识',
    `house_address` VARCHAR(255) NOT NULL COMMENT '房屋地址',
    `community_id` VARCHAR(50) NOT NULL COMMENT '小区编号',
    `registration_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '房屋登记时间',
    `delete_time` datetime DEFAULT NULL COMMENT '房屋删除时间',
    `second_hand_sale_time` datetime DEFAULT NULL COMMENT '二手房成交时间',
    `second_hand_listed_time` datetime DEFAULT NULL COMMENT '二手房上架时间',
    `second_hand_unlisted_time` datetime DEFAULT NULL COMMENT '二手房下架时间',
    `rental_listed_time` datetime DEFAULT NULL COMMENT '租房上架时间',
    `rental_unlisted_time` datetime DEFAULT NULL COMMENT '租房下架时间',
    `rental_sale_time` datetime DEFAULT NULL COMMENT '租房成交时间',
    `rental_end_time` datetime DEFAULT NULL COMMENT '租房租期结束时间'
);

-- 上架
CREATE TABLE house_second_hand_listed (
    `id` INT NOT NULL AUTO_INCREMENT  PRIMARY KEY  COMMENT '自增主键',
    `house_id` VARCHAR(255) NOT NULL COMMENT '字符串唯一标识',
    `pice`  DECIMAL(10, 2) NOT NULL COMMENT '房屋单价',
    `low_pice`  DECIMAL(10, 2) NULL COMMENT '房屋低价价',
    `event_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '事件时间'
);

-- 下架
CREATE TABLE house_second_hand_unlisted (
    `id` INT NOT NULL AUTO_INCREMENT  PRIMARY KEY COMMENT '自增主键',
    `house_id` VARCHAR(255) NOT NULL COMMENT '字符串唯一标识',
    `event_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '事件时间'
);

-- 卖出
CREATE TABLE house_second_hand_sale (
    `id` INT NOT NULL AUTO_INCREMENT  PRIMARY KEY COMMENT '自增主键',
    `sale_price` DECIMAL(10, 2) NOT NULL COMMENT '二手房成交价格',
    `house_id` VARCHAR(255) NOT NULL COMMENT '字符串唯一标识',
    `event_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '事件时间'
);