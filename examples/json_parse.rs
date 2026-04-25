use hkdt_rs::{impl_jsonable, json::Jsonable};
use hkdt_rs::log_info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Person
{
    name : String,
    age : i32
}

impl_jsonable!(Person);

fn main()
{
    let data = Person { name : "Taro".to_string(), age : 291};

    log_info!("Jsonableな構造体はStringへ変換可能です");
    let str_from_data = data.convert_to_string();
    log_info!("変換後-->{}", str_from_data);

    log_info!("更にStringから構造体を取り出すこともできます。");
    let data_from_string = Person::from_string(&str_from_data);
    log_info!("{}({}歳)", data_from_string.name, data_from_string.age);
}