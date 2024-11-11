# rust-json-formatter

Rust 勉強用の JSON フォーマッター
refs: [Rust 入門がてら JSON フォーマッターを書く](https://zenn.dev/sa2knight/articles/rust-json-formatter)

```bash
$ echo "{\"key1\":10,\"key2\":[1,2,{\"key3\": [\"Hello\",true, false, null]}]}" | cargo run
{
  "key1": 10,
  "key2": [
    1,
    2,
    {
      "key3": [
        "Hello",
        true,
        false,
        null
      ]
    }
  ]
}
```
