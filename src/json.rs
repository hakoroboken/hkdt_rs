use serde::{Deserialize, Serialize};
use serde_json;

pub trait Jsonable
where
    Self: Serialize + for<'de> Deserialize<'de> + Sized + Clone,
{
    fn convert_to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Json形式の文字列を構造体に変換する
    fn from_string(packet: &str) -> Self {
        serde_json::from_str(packet).unwrap()
    }
}

#[macro_export]
macro_rules! impl_jsonable {
    ($t:ty) => {
        impl Jsonable for $t {}
    };
}
