-- Your SQL goes here
CREATE TABLE house (
    house_id VARCHAR(255) PRIMARY KEY NOT NULL COMMENT '房屋编号',
    community_name VARCHAR(255) NOT NULL COMMENT '小区名称',
    house_address VARCHAR(255) NOT NULL COMMENT '房屋地址',
    house_type VARCHAR(50) NOT NULL COMMENT '房屋类型',
    area DECIMAL(10, 2) NOT NULL COMMENT '房屋面积',
    bedrooms INT NOT NULL COMMENT '卧室数量',
    living_rooms INT NOT NULL COMMENT '客厅数量', 
    bathrooms INT NOT NULL COMMENT '卫生间数量',
    orientation VARCHAR(20) COMMENT '房屋朝向',
    decoration_status VARCHAR(255) COMMENT '房屋装修情况', 
    status VARCHAR(50) COMMENT '房屋状态',
    house_description TEXT COMMENT '房屋描述',
    house_image VARCHAR(255) COMMENT '房屋图片', 
    owner_name VARCHAR(100) NOT NULL COMMENT '户主姓名',
    owner_phone VARCHAR(20) NOT NULL COMMENT '户主联系方式', 
    created_by VARCHAR(255)  COMMENT '创建人',
    updated_by VARCHAR(255) COMMENT '更新人',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

