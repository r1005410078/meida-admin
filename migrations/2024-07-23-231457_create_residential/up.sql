-- Your SQL goes here
CREATE TABLE residential (
    community_id VARCHAR(255) NOT NULL PRIMARY KEY COMMENT '字符串唯一标识',
    name VARCHAR(255) NOT NULL UNIQUE COMMENT '唯一名称',
    address VARCHAR(255) NOT NULL UNIQUE COMMENT '唯一地址',
    city VARCHAR(100) NOT NULL DEFAULT '安庆市' COMMENT '默认安庆市', 
    state VARCHAR(100) NOT NULL DEFAULT '安徽省' COMMENT '默认安徽省',
    postal_code VARCHAR(20) NOT NULL DEFAULT '246000' COMMENT '邮政编码',
    year_built SMALLINT NOT NULL COMMENT '建设年份，使用 YEAR 数据类型',
    community_type  VARCHAR(100) NOT NULL DEFAULT '住宅' COMMENT '住宅区类型，使用 ENUM 数据类型限定为特定值',
    description TEXT COMMENT '描述'
);