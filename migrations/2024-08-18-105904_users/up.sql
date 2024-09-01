-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    `id` CHAR(36) PRIMARY KEY DEFAULT (UUID()), -- 使用 UUID 作为主键
    `username` VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名',
    `password_hash` VARCHAR(255) NOT NULL COMMENT '密码哈希',
    `phone` VARCHAR(100) NOT NULL UNIQUE COMMENT '手机号',
    `avatar` VARCHAR(255) COMMENT "头像",
    `is_active` BOOLEAN DEFAULT TRUE COMMENT '是否激活',
    `role` VARCHAR(20) DEFAULT 'admin' COMMENT '角色',

    `created_by` VARCHAR(255)  COMMENT '创建人',
    `updated_by` VARCHAR(255) COMMENT '更新人',
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间' 
);