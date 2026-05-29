use std::collections::{HashMap}; //TODO: データベースを理解するためにはこのHashMapを理解する必要がある気がする。メモリに保存するって所は理解できたけど、ちゃんとソースコードで確認するべき



pub struct KvStore {
    data: HashMap<Vec<u8>, Vec<u8>>
}

impl KvStore {

    //インスタンスの作成
    pub fn new() -> Self {
        let data = HashMap::new();
        Self { data }
    }

    // insert
    //　HashMap自体にものを追加したいから（可変でありたい）mut ただHashMap自体をどこか他の場所に移すわけじゃないから所有権自体はいらない
    // 検索キーと保存するものどっちもほしよねinsertは。insertの場合はどっちも別の場所HashMapに変数を移動させたいから所有権が欲しい
    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.data.insert(key, value);
    }

    // delete
    // HashMap自体に影響は与えるものではあるが、HashMap自体を別の場所に移動させたいわけではないから&はつける（&つけなかったら所有権も来ちゃうからね）
    // key valueに関しては、deleteするのにそもそもvalueはいらない。keyだけ欲しくて、しかも移動させるわけではないから、所有権はいらない、参照も一応したいけど、可変である必要もない。
    pub fn delete(&mut self, key: &Vec<u8>) {
        self.data.remove(key);
    }
    
    // get
    // HashMap自体に変更は加えない。単純に使い方から参照が欲しいだけか。hashmapに対して関数を呼んではいるが、影響を与えているものではないため、mutをつけるかは疑問
    // keyだけで良くて、単純に値が欲しいだけだから、所有権はいらないし、編集も別にしないからmutもいらないか
    pub fn get(&self, key: &Vec<u8>) -> Option<&Vec<u8>> {
        self.data.get(key)
    }
}