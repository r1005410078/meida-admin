-- Your SQL goes here
CREATE TABLE IF NOT EXISTS house (
    house_id VARCHAR(255) PRIMARY KEY NOT NULL COMMENT '房屋编号',
    community_name VARCHAR(255) NOT NULL COMMENT '小区名称',
    house_address VARCHAR(255) NOT NULL COMMENT '房屋地址',
    property VARCHAR(50) COMMENT '产权',
    decoration_status VARCHAR(255) COMMENT '房屋装修情况',
    area DECIMAL(10, 2) NOT NULL COMMENT '建筑面积',

    bedrooms INT NOT NULL COMMENT '卧室数量',
    living_rooms INT NOT NULL COMMENT '客厅数量', 
    bathrooms INT NOT NULL COMMENT '卫生间数量',
    balcony INT COMMENT '阳台',
    kitchen INT COMMENT '厨房',
    orientation VARCHAR(20) COMMENT '房屋朝向',
    house_description TEXT COMMENT '房屋描述',
    house_image TEXT COMMENT '房屋图片', 
    owner_name VARCHAR(100) NOT NULL COMMENT '业主姓名',
    owner_phone VARCHAR(20) NOT NULL COMMENT '业主联系方式', 
    floor INT  COMMENT '楼层',
    floor_range VARCHAR(255) COMMENT '总楼层',

    -- 2024-07-24 23:12:13
    title VARCHAR(255) NOT NULL COMMENT '房源标题',
    recommended_tags VARCHAR(255)  DEFAULT '' COMMENT '推荐标签',
    elevator INT COMMENT '梯',
    household INT COMMENT '户',
 
    building_structure VARCHAR(100) COMMENT '建筑结构',
    building_year DATE COMMENT '建筑年代',
    property_rights VARCHAR(100) COMMENT '产权性质',
    property_duration INT COMMENT '产权年限',
    property_date DATE COMMENT '产权日期',
    delivery_date DATE COMMENT '交房日期',
    school_qualification VARCHAR(50) COMMENT '学位',
    household_registration VARCHAR(50) COMMENT '户口',
    unique_house TINYINT(1) COMMENT '唯一住房',
    facilities TEXT COMMENT '配套',
    usable_area DECIMAL(10, 2) COMMENT '使用面积',
    current_status VARCHAR(255) COMMENT '现状',
    house_type VARCHAR(255) COMMENT '房屋类型',
    source VARCHAR(100) COMMENT '来源',

    `created_by` VARCHAR(255)  COMMENT '创建人',
    `updated_by` VARCHAR(255) COMMENT '更新人',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);

