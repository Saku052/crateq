// 目的: インメモリKvStoreの構築
// 簡単にいうと、HashMapのラッパーではある

use std::{collections::HashMap};

// 基本のHashMapのstructを定義していく
pub struct KvStore {
    data: HashMap<Vec<u8>, Vec<u8>> // TODO: HashMapの中身は確認しておくべき
}

// KvStoreの関数を作れるようにしていく
impl KvStore {

    //コンストラクタ
    pub fn new() -> Self {
        let data = HashMap::new();
        Self { data }
    }

    // insert
    // dataは欲しいよね、しかも追加するから可変でありたい
    // keyもvalueも欲しい。hashmap本体の方に移したいから所有権も欲しい
    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.data.insert(key, value);
    }

    // delete
    // dataの変更権がほしい
    // keyだけでいいね。keyは参照だけでいいし、変更もしない。
    pub fn delete(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.data.remove(key) //TODO: 削除できたことをreturnするべき
    }

    // get
    // data自体に変更は加えないし、他の場所に移したりもしない
    // keyだけでいいね。keyも参照だけで良くて、変更もしない
    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.data.get(key).map(|v| v.as_slice())
    }
}

// TODO: modで囲んだ方がいい
#[test]
fn set_and_get () {
    let mut kvstore = KvStore::new();

    kvstore.insert(b"key".to_vec(), b"value".to_vec());
    assert_eq!(kvstore.get(b"key"), Some(b"value".as_slice()));
}

#[test]
fn check_delete() {
    let mut store = KvStore::new();

    let key = b"key";
    let value = b"value";

    store.insert(key.to_vec(), value.to_vec());

    let delete_output: Vec<u8>;
    delete_output = store.delete(key).unwrap();

    assert_eq!(delete_output, value.to_vec());
}