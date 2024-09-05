-- Your SQL goes here

-- 2024-09-02-151448_imports/up.sql

-- 导入房源

create table if not exists imports_properties (
  `id` varchar(255) primary key comment '主键',
  `pice_type` varchar(255) not null comment '销售类型',
  `usage` varchar(255) not null comment '用途',
  `platform` varchar(255) not null comment '平台',
  `file_status` varchar(255) not null comment '状态 -1 准备同步 0 同步中 1 同步成功 2 同步失败',
  `file_error` varchar(255) comment '错误信息',  
  `file_name` varchar(255) not null comment '文件名',
  `file_path` varchar(255) not null comment '文件路径',
  `file_size` bigint not null comment '文件大小',
  `created_at` datetime not null default current_timestamp comment '创建时间',
  `updated_at` datetime not null default current_timestamp on update current_timestamp comment '更新时间'
)