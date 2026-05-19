# 書籍ハンドラの解説

`api/src/handler/book.rs` に追加した書籍の「追加（register）」「読み取り（show / list）」
ハンドラについて、シグネチャ・ネストされた構造体・`map` / クロージャの使い方を解説する。

対象コード:

- `api/src/handler/book.rs` … ハンドラ本体・`AppError`
- `api/src/model/book.rs` … リクエスト / レスポンス用の構造体と `From` 実装
- `api/src/route/book.rs` … ルーティング登録

---

## 全体像：4つの層とデータの流れ

このプロジェクトはクレート（≒モジュール）が層ごとに分かれている。

| 層 | 役割 | 主なファイル |
|---|---|---|
| **api** | HTTP の入口。ハンドラ・ルーティング・JSON 用の構造体 | `api/src/handler/book.rs`, `api/src/model/book.rs` |
| **kernel** | ドメインの中心。モデルとリポジトリの interface（trait）だけ | `kernel/src/model/book/`, `kernel/src/repository/book.rs` |
| **adapter** | 実際の DB 処理。kernel の trait を実装 | `adapter/src/repository/book.rs` |
| **registry** | DI コンテナ。`AppRegistry` | `registry/src/lib.rs` |

書籍を 1 冊取得するときの流れ:

```
HTTP リクエスト → axum がルーティング → ハンドラ関数
  → AppRegistry からリポジトリ取得 → SQL 実行
  → BookRow(DB の行) → Book(ドメイン) → BookResponse(JSON 用)
  → axum が JSON 化して返す
```

「似たような構造体が次々出てくる」のが分かりにくさの正体。後述の「ネストされた構造体」で説明する。

---

## 1. ハンドラのシグネチャの読み方

まず `register_book`（`api/src/handler/book.rs:24-27`）。

```rust
pub async fn register_book(
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> Result<StatusCode, AppError> {
```

ポイントは **引数が「型」ではなく「パターン」で書かれている** こと。
`State(registry): State<AppRegistry>` を分解すると:

- `: State<AppRegistry>` … 引数の **型**
- `State(registry)` … **パターン分解（destructuring）**

`State` は中身を 1 つ持つタプル構造体。`State(registry)` は「`State` の中身を取り出して
`registry` という名前で受け取る」という意味。`let State(registry) = ...;` を引数の位置で
やっているだけ。

なぜこうするかというと、axum の **エクストラクタ** という仕組みのため。axum は引数の **型**
を見て「リクエストのどこから値を取り出すか」を決める。

| 引数 | 型 | どこから取り出すか |
|---|---|---|
| `State(registry)` | `State<AppRegistry>` | アプリ全体で共有する状態（DI コンテナ） |
| `Json(req)` | `Json<CreateBookRequest>` | リクエストボディの JSON → `CreateBookRequest` にデシリアライズ |
| `Path(book_id)` | `Path<Uuid>` | URL パス（`/books/{book_id}` の部分） |

つまり「**型を書くと axum が値を用意してくれて、パターン分解で中身だけ受け取る**」二段構え。
引数の順番は自由（axum は型で判断するため）。

### 戻り値の型

`register_book` の戻り値 `Result<StatusCode, AppError>`:

- 成功なら `StatusCode`（`201 CREATED`）、失敗なら `AppError`
- axum は「`Ok` の中身と `Err` の中身がそれぞれ `IntoResponse`（HTTP レスポンスに変換できる）
  なら」ハンドラとして受け付ける。`AppError` には `IntoResponse` 実装がある
  （`api/src/handler/book.rs:18-22` — 中身は常に 500 を返すだけ）。

`show_book_list` の戻り値 `Result<Json<Vec<BookResponse>>, AppError>` が、いわゆる
**ネストした型**。内側から読む:

```
BookResponse        … 1 冊
Vec<BookResponse>   … 複数冊
Json<Vec<...>>      … JSON 化して返す印
```

`Json` はリクエストの受け取りにもレスポンスの返却にも使える点に注意。

---

## 2. ネストされた構造体：なぜ似た構造体がたくさんあるのか

書籍まわりに **5 つの構造体** が登場する。フィールドはほぼ同じなのに別物。

| 構造体 | 場所 | 役割 |
|---|---|---|
| `CreateBookRequest` | `api/src/model/book.rs` | 登録 **リクエスト** の JSON を受ける器 |
| `CreateBook` | `kernel/src/model/book/event.rs` | 「本を作れ」というドメインの命令 |
| `Book` | `kernel/src/model/book/mod.rs` | ドメインの「本」そのもの |
| `BookRow` | `adapter/src/database/model/book.rs` | DB の 1 行に対応 |
| `BookResponse` | `api/src/model/book.rs` | **レスポンス** の JSON を作る器 |

**なぜ分けるのか → 層ごとに「都合」が違うから。**

- `CreateBookRequest` … `#[derive(Deserialize)]` + `#[serde(rename_all = "camelCase")]`
  → 外部 JSON（camelCase）を読む都合
- `BookResponse` … `#[derive(Serialize)]` → JSON を書き出す都合
- `Book` … serde 非依存の純粋なドメイン都合
- `BookRow` … DB の都合（カラム名が `book_id`）

こう分離すると「**API の JSON 形式を変えてもドメインは変わらない**」「**DB スキーマを変えても
API は変わらない**」が成立する。実際 `Book` / `BookResponse` は `id` なのに `BookRow` だけ
`book_id` — DB カラム名に合わせており、変換時に名前を付け替えている。

`#[serde(rename_all = "camelCase")]` … Rust 側は `snake_case`、JSON 側は `camelCase` という
変換指定。今回のフィールドは 1 単語なので見た目の差は出ないが、複数単語のフィールドを
足すと効いてくる。

---

## 3. From 変換：`.into()` の正体

層が違う構造体同士をつなぐのが `From` トレイト実装（`api/src/model/book.rs:14-29`）。

```rust
impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest { title, author, isbn, description } = value;
        Self { title, author, isbn, description }
    }
}
```

読み方:

- `impl From<CreateBookRequest> for CreateBook` … 「`CreateBookRequest` から `CreateBook`
  を作れる」と宣言
- `let CreateBookRequest { title, author, isbn, description } = value;` … **構造体パターン
  分解**。`value` の各フィールドを同名のローカル変数へ取り出す
- `Self { title, author, isbn, description }` … `Self` は `CreateBook`。`title: title` の
  略記（フィールド初期化省略形）

`From` を実装すると、標準ライブラリが対になる `Into` を自動導出するので、**`.into()` が
自動で使える**。だから `register_book` 内の `req.into()`（`api/src/handler/book.rs:30`）は
`CreateBookRequest` を `CreateBook` に変換している。

> **なぜ `.into()` が `CreateBook` を作ると分かるのか？**
> `.create(req.into())` の `create` が引数に `CreateBook` を要求しているから。
> コンパイラが「結果が `CreateBook` になる `Into` 実装」を逆算する。

同じ仕組みが `Book → BookResponse`（`api/src/model/book.rs:41-58`）と
`BookRow → Book`（adapter 側）にもある。`show_book` の `bc.into()` は
`Book → BookResponse` を呼んでいる。

---

## 4. map / map_err / and_then / クロージャ

ここが一番こんがらがる箇所。**同じ `.map` でも対象の型によって意味が違う** のがコツ。

### 大前提：`.map` は 1 つではない

| メソッド | 対象 | 何をするか |
|---|---|---|
| `Result::map` | `Result<T, E>` | `Ok` の中身だけ変換（`Err` は素通り） |
| `Result::map_err` | `Result<T, E>` | `Err` の中身だけ変換（`Ok` は素通り） |
| `Result::and_then` | `Result<T, E>` | `Ok` の中身を「`Result` を返す関数」に通す（**新たに失敗を起こせる**） |
| `Option::map` | `Option<T>` | `Some` の中身だけ変換 |
| `Iterator::map` | イテレータ | 各要素を変換（`collect` で実体化） |

名前が同じなので「**いま何型に対して呼んでいるか**」を意識すると読める。

### register_book

```rust
registry
    .book_repository()              // Arc<dyn BookRepositry>
    .create(req.into())             // create() の戻りは Future
    .await                          // → Result<(), anyhow::Error>
    .map(|_| StatusCode::CREATED)   // → Result<StatusCode, anyhow::Error>
    .map_err(AppError::from)        // → Result<StatusCode, AppError>
```

- `.create(...)` は `Result<(), anyhow::Error>` を返す（`()` は「中身なし」のユニット型）
- `.map(|_| StatusCode::CREATED)` … **Result の map**。`Ok` の中身を捨て（`|_|` の `_` が
  「使わない」印）、`StatusCode::CREATED` に差し替え。成功＝201 を返したいだけで中身の
  `()` は不要だから
- `.map_err(AppError::from)` … `Err` の中身 `anyhow::Error` を `AppError` に変換

> `|_| StatusCode::CREATED` は「引数を受け取るが使わず、常に `StatusCode::CREATED` を
> 返す」**クロージャ**。
> `AppError::from` は **関数そのものを値として渡す** 書き方で、
> `.map_err(|e| AppError::from(e))` の短縮形。

### show_book_list：一番複雑な行（`api/src/handler/book.rs:43-45`）

```rust
.map(|v| v.into_iter().map(BookResponse::from).collect::<Vec<_>>())
.map(Json)
.map_err(AppError::from)
```

`find_all()` は `Result<Vec<Book>, anyhow::Error>` を返す。

外側の `.map(|v| ...)` は **Result の map**。`v` は `Vec<Book>`。中のクロージャを順に:

1. `v.into_iter()` … `Vec<Book>` を **イテレータ** へ。`into_iter` は所有権ごと取り出す
   （各 `Book` を `BookResponse::from` に渡すには所有権が要るため）
2. `.map(BookResponse::from)` … これは **Iterator の map**。各 `Book` を `BookResponse`
   に変換。ここでも関数 `BookResponse::from` をそのまま渡している
3. `.collect::<Vec<_>>()` … イテレータを実体の `Vec` にまとめる。`::<Vec<_>>` は
   **ターボフィッシュ** で「`Vec` に集めて」と指定。`_`（要素の型）は `BookResponse` だと
   コンパイラに推論させている

→ 結果、`Vec<Book>` が `Vec<BookResponse>` になる。つまり
**「Result の map の中で、さらに Iterator の map を使う」二重構造**。

次の `.map(Json)` … また **Result の map**。`Vec<BookResponse>` を `Json(...)` で包む。
`Json` はタプル構造体なので、名前 `Json` 自体が「中身を 1 つ受け取って `Json` を作る関数」
として使え、`.map(|x| Json(x))` の短縮形になっている。

### show_book：`and_then` とクロージャ内 match（`api/src/handler/book.rs:54-60`）

```rust
.find_by_id(book_id)        // → Result<Option<Book>, anyhow::Error>
.await
.and_then(|bc| match bc {
    Some(bc) => Ok(Json(bc.into())),
    None => Err(anyhow::anyhow!("The specific book was not found")),
})
.map_err(AppError::from)
```

`find_by_id` は `Result<Option<Book>, _>` … 「DB エラーは無いが、本が見つからない可能性は
ある」ので **`Result` の中に `Option`** という二重構造。

**なぜ `.map` ではなく `.and_then` か** … `.map` は「`Ok` の中身を別の値に変換」しか
できない。ここでは **「見つからない＝エラー扱いにしたい」＝ 成功を失敗に変えたい**。
それには「`Result` を返すクロージャ」を渡せる `and_then` が必要。

- `bc` は `Option<Book>`
- `Some(bc) => Ok(Json(bc.into()))` … 見つかった。`bc.into()` で `Book → BookResponse`、
  `Json(...)` で包み、`Ok` で返す
  - ※ 内側の `bc` は `Some` から取り出した `Book`。外側の `bc`（`Option<Book>`）を
    **同名で覆い隠している（シャドーイング）**。同じ名前なので紛らわしい点
- `None => Err(anyhow::anyhow!("..."))` … 見つからない。`anyhow!` マクロでその場で
  エラーを生成し `Err` で返す

`and_then` の結果はまた `Result`（中身は `Json<BookResponse>` か `anyhow::Error`）。
最後に `.map_err(AppError::from)` で `AppError` へ。

> `and_then` は「**失敗を新たに発生させられる map**」と捉えると分かりやすい。

---

## まとめ：3 つのコツ

1. **ハンドラ引数** = 「型＝axum がどこから値を取るかの指示」＋「`Name(x)` ＝中身の
   取り出し」の二段構え。
2. **似た構造体が多い** のは層ごとの都合の分離。つなぐのが `From` 実装、呼び口が `.into()`。
3. **`.map` は対象の型で意味が変わる**（`Result` / `Iterator` / `Option`）。成功を失敗に
   変えたいときだけ `and_then`。`Json` / `BookResponse::from` / `AppError::from` のように
   関数名だけ渡すのはクロージャの短縮形。
