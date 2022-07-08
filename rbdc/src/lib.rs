use std::collections::HashMap;
use indexmap::IndexMap;

pub mod encode;
pub mod decode;
pub mod db;


#[cfg(test)]
mod test {
    #[test]
    fn test_ser() {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct A {
            pub name: String,
            pub i32:i32,
            pub u32:u32,
            pub i64:i64,
            pub u64:u64
        }
        let buf = rbmp_serde::to_vec(&A {
            name: "s".to_string(),
            i32: i32::MAX,
            u32: u32::MAX,
            i64: i64::MAX,
            u64: u64::MAX,
        }).unwrap();
        let v: rbmpv::Value = rbmpv::decode::read_value(&mut &buf[..]).unwrap();
        println!("{}", v);

        let v: A = rbmp_serde::decode::from_slice(&buf).unwrap();
        println!("{:?}", v);

        let v: rbmpv::Value = rbmp_serde::decode::from_slice(&buf).unwrap();
        println!("{}", v);
    }
}