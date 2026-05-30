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
    pub fn delete(&mut self, key: &[u8]) {
        self.data.remove(key);
    }

    // get
    // data自体に変更は加えないし、他の場所に移したりもしない
    // keyだけでいいね。keyも参照だけで良くて、変更もしない
    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.data.get(key).map(|v| v.as_slice())
    }
}