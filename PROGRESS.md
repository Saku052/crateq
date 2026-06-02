# PROGRESS.md

CrateQ の進捗ログ。各ステップ完了時に追記する。
別チャット（claude.ai 側）に状況を引き継ぐ時はこのファイルを貼る。

---

## Step 1: インメモリ KV ストア ✅ (2026-06-02)

### 実装した API

`src/lib.rs`:

```rust
pub struct KvStore {
    data: HashMap<Vec<u8>, Vec<u8>>,
}

impl KvStore {
    pub fn new() -> Self;
    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>);
    pub fn get(&self, key: &[u8]) -> Option<&[u8]>;
    pub fn delete(&mut self, key: &[u8]);
}
```

### 設計判断のメモ

- **キー・値はバイト列 `Vec<u8>`**
  将来の WAL（Step 2）やネットワーク I/O（Step 8）でバイト列が必要なので、最初から統一しておく。文字列固定にしない。

- **引数の型: 所有権が必要な所は `Vec<u8>`、参照で十分な所は `&[u8]`**
  - `insert`: HashMap に格納するので所有権が必要 → `Vec<u8>`
  - `get` / `delete`: 検索キーは参照だけで十分 → `&[u8]`
  - これで呼び出し側がリテラル `b"foo"` や `&Vec<u8>` をそのまま渡せる（Deref 経由）
  - Clippy の `ptr_arg` ルールに沿った形

- **`get` の戻り値は `Option<&[u8]>`**
  - `&Vec<u8>` を返すと「内部で Vec を保持している」という実装の詳細が API に漏れる
  - `&[u8]` なら将来内部表現を変えても API が壊れない
  - `Option::map(|v| v.as_slice())` で `&Vec<u8>` → `&[u8]` に変換

- **抽象化はしない（CLAUDE.md 方針）**
  trait や generics は今は不要。Step 1 では「動く HashMap ラッパー」だけ。

### 選ばなかった道（次以降で検討）

- **`delete` の戻り値 `bool`**（消せたかどうか）
  Step 1 では戻り値なし。呼び出し側で必要になったタイミングで `Option<Vec<u8>>` か `bool` を返す形に変える。
- **`Default` trait の実装**
  `new() -> Self` を持つ型には Rust の慣習として Default も足すのが定番。Clippy が `new_without_default` を出したら追加。
- **ライブラリ全体のエラー型**
  今は失敗ケースが無いので不要。Step 2 で I/O エラーが入ってきたら導入する。

### テスト

`src/lib.rs` 末尾に `#[cfg(test)] mod tests` で 1 件：

- `set_and_get`: insert した値が get で取り出せる正常系

将来追加候補（時間ができたら）:
- `get_missing_returns_none`: 存在しないキーは `None`
- `delete_removes_key`: delete した後は `get` で取れない
- `overwrite_updates_value`: 同じキーへの再 insert で値が上書きされる

### 動作確認

`src/main.rs` で組み込み的に使う最小サンプル：
- KvStore を作って `insert` / `get` / `delete` を呼び、`println!` で結果を表示
- `&[u8]` の中身を人間が読める形にするため `std::str::from_utf8(...).unwrap()` を使った
- `cargo run` / `cargo test` どちらも緑

### Step 1 で実際に踏んだ Rust の概念

このステップで体に入った概念群（次以降で繰り返し出てくる）：

- `Vec<T>` と `&[T]` の違い（所有 vs ビュー）と `Deref` による自動変換
- `&self` / `&mut self` の使い分け、それに応じた変数側の `mut` 必須
- フィールドアクセス `self.data` と モジュールパス `self::data` の違い
- use after move（`insert(key, ...)` した後に同じ `key` を使おうとして詰まった）
- `Option::map` で `Option<A>` → `Option<B>`
- `Result<T, E>` は `Display` を実装していない → `unwrap()` で中身を取り出してから表示
- `#[cfg(test)] mod tests { use super::*; }` のテストモジュール定型
- 命名規約: 型は `PascalCase`、関数・変数は `snake_case`

### 次のステップで気をつけること（Step 2: WAL）

- **WAL の役割**: 書き込み（`insert` / `delete`）の前にディスクのログファイルに追記し、再起動時にそれをリプレイしてメモリ状態を復元する。これでクラッシュ耐性が出る。
- **責任分担の選択**:
  - KvStore 自体に WAL を持ち込むか、`Database` のような上位レイヤーを作って KvStore + WAL を束ねるか
  - 学習目的なら最初は KvStore に直接持たせる方が単純。後で分離してもいい
- **ファイル形式の選択**:
  - 独自の単純フォーマット（例: `OP key_len key value_len value` の繰り返し）から始めるのが学びになる
  - `bincode` などの serde 系 crate は便利だが、最初の WAL では避けて手で組み立てる方が中身が分かる
- **エラー型の導入**:
  - I/O が入るので `Result<T, E>` を返す API になる
  - 最初は `std::io::Error` をそのまま使う。複数のエラー源が出てきたら独自 `Error` enum に切り替える
- **`fsync` の扱い**:
  - 学習目的なら最初は `fsync` 無しで OK（プロセスクラッシュには耐えるが OS クラッシュには弱い）
  - バックログに「`fsync` チューニング」として置いてある通り、後回しでよい
- **テスト戦略**:
  - 「ファイルに書く → 別の KvStore インスタンスでリプレイ → 同じ状態になる」を確認するテストが核になる
  - `tempfile` crate を使うとテスト用の一時ファイルが楽に作れる
