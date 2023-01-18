# lifegame_rust

Rust 製のライフゲーム, ターミナル上で動作します。
![](https://github.com/skuralll/lifegame_rust/blob/main/resource/demo.png?raw=true)

## 使用法

リポジトリをクローン後、cargo コマンドにより実行します。

```bash
git clone https://github.com/skuralll/lifegame_rust
cd lifegame_rust
cargo run
```

## ゲーム内部操作

実行モード(Run)と編集モード(Edit)を切り替えて遊びます。  
モードは Tab キーで切り替えることができます。
| モード | 内容 |
| ---- | ---- |
| 実行モード | ライフゲームが進行します。 |
| 編集モード | 方向キーカーソルを動かし、スペースキーでセルの生死を切り替えることができます。 |
