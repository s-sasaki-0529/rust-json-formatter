# rust-json-formatter

Rust 勉強用の JSON フォーマッター

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
