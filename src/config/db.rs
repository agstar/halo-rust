use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_sqlite::driver::SqliteDriver;

use crate::settings::SETTINGS;

const MYSQL: String = String::from("mysql");
const SQLITE: String = String::from("mysql");

pub fn setup() {
    /// enable log crate to show sql logs
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    /// initialize rbatis. also you can call rb.clone(). this is  an Arc point
    let rb = Rbatis::new();
    /// connect to database
    match SETTINGS.database.platform {
        MYSQL => rb.init(MysqlDriver {}, SETTINGS.database.uri.as_str()).unwrap(),
        SQLITE => rb.init(SqliteDriver {}, SETTINGS.database.uri.as_str()).unwrap()
        _ => panic!("unknown database platform")
    }
}