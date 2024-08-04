-- Your SQL goes here
CREATE TABLE residential_aggregate (
    `community_name` VARCHAR(255) NOT NULL PRIMARY KEY COMMENT '小区名称',
    `region` VARCHAR(255) NOT NULL, -- 小区地址
    `city` VARCHAR(255) NOT NULL,    -- 城市
    `state` VARCHAR(255) NOT NULL    -- 省份
);

-- 房源聚合
CREATE TABLE house_aggregate (
    `house_id` VARCHAR(255) NOT NULL PRIMARY KEY COMMENT '字符串唯一标识',
    `house_address` VARCHAR(255) NOT NULL COMMENT '房屋地址',
    `community_name` VARCHAR(50) NOT NULL COMMENT '小区名称',
    `registration_time` DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '房屋登记时间',
    `delete_time` DATETIME DEFAULT NULL COMMENT '房屋删除时间',
    `second_hand_sale_time` DATETIME DEFAULT NULL COMMENT '二手房成交时间',
    `second_hand_listed_time` DATETIME DEFAULT NULL COMMENT '二手房上架时间',
    `second_hand_unlisted_time` DATETIME DEFAULT NULL COMMENT '二手房下架时间',
    `rental_listed_time` DATETIME DEFAULT NULL COMMENT '租房上架时间',
    `rental_unlisted_time` DATETIME DEFAULT NULL COMMENT '租房下架时间',
    `rental_start_time` DATETIME DEFAULT NULL COMMENT '租房开始时间',
    `rental_end_time` DATETIME DEFAULT NULL COMMENT '租期结束时间',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 上架的二手房
CREATE TABLE house_second_hand (
    `house_id` VARCHAR(255) NOT NULL PRIMARY KEY COMMENT '字符串唯一标识',
    `community_name` VARCHAR(50) NOT NULL COMMENT '小区名称',
    `pice`  DECIMAL(10, 2) NOT NULL COMMENT '房屋单价',
    `low_pice`  DECIMAL(10, 2) NULL COMMENT '房屋低价价',
    `listed` TINYINT NOT NULL DEFAULT 1 COMMENT '是否上架 0 为下架 1 为上架',
    `listed_time` DATETIME DEFAULT NULL COMMENT '二手房上架时间',
    `unlisted_time` DATETIME DEFAULT NULL COMMENT '二手房下架时间',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 卖出的二手房
CREATE TABLE house_second_hand_sold (
    `sold_id`  INT AUTO_INCREMENT NOT NULL COMMENT '唯一标识',
    `house_id` VARCHAR(255) NOT NULL COMMENT '字符串唯一标识',
    `community_name` VARCHAR(50) NOT NULL COMMENT '小区名称',
    `days_to_sell` INT  NOT NULL COMMENT '卖了多少天',
    `sold_price` DECIMAL(10, 2) NOT NULL COMMENT '二手房成交价格',
    `sold_time` DATETIME  NOT NULL COMMENT '二手房成交时间',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',

    PRIMARY KEY (`sold_id`)
);

-- 出租房信息
CREATE TABLE house_rental (
    `house_id` VARCHAR(255) NOT NULL PRIMARY KEY COMMENT '字符串唯一标识',
    `community_name` VARCHAR(50) NOT NULL COMMENT '小区名称',
    `rent_pice`  DECIMAL(10, 2) NOT NULL COMMENT '租金',
    `rent_low_pice`  DECIMAL(10, 2) NULL COMMENT '最底租金',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

-- 出租房租出
CREATE TABLE house_rental_sold (
    `sold_id` INT AUTO_INCREMENT NOT NULL PRIMARY KEY COMMENT '唯一标识',
    `house_id` VARCHAR(255) NOT NULL  COMMENT '字符串唯一标识',
    `community_name` VARCHAR(50) NOT NULL COMMENT '小区名称',
    `rent_pice`  DECIMAL(10, 2) NOT NULL COMMENT '租金',
    `rent_start_time` DATETIME NOT NULL COMMENT '开始时间',
    `rent_end_time`  DATETIME NOT NULL COMMENT '到期时间',
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);