// 定义 FIND_IN_SET SQL 函数
use diesel::{define_sql_function, sql_types::Text};

define_sql_function!(fn find_in_set(needle: Text, haystack: Text) -> Integer);
