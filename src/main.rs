use crateq::store::KvStore;
use std::str::from_utf8;

fn main() {
    let mut store = KvStore::new();

    //2件ほどデータを入れてみる
    let key1: Vec<u8> = b"myKey1".to_vec();
    let key2: Vec<u8> = b"myKey2".to_vec();
    let value1: Vec<u8> = b"value1".to_vec();
    let value2: Vec<u8> = b"value2".to_vec();

    store.insert(key1, value1);
    store.insert(key2, value2);

    let value_result1: &[u8] = store.get(b"myKey1").expect("no value found");

    print!("{}", from_utf8(value_result1).unwrap());
}
