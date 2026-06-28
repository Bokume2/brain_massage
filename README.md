# BrainMassage
Brainf\*ckの記述を実用言語風の読みやすい形式に置き換え、トランスパイルの難易度が上がらない範囲で記述を簡略化した言語です。  

## Build and Installation
`git clone`等でコピーしたリポジトリのルートで以下のコマンドを実行すると、target/releaseディレクトリ内に実行可能ファイルが生成されます。  
```bash
cargo build --release
```
あるいは、同じくリポジトリルートで以下のコマンドを実行すると、Cargoに設定されたインストール先(Linux環境のデフォルトでは$HOME/.cargo/bin)に実行可能ファイルがインストールされます。  
```bash
cargo install --path . --offline
```

## Usage
BrainMassageのコードをテキストファイルに記述し、以下のコマンドでコンパイルします。  
```bash
# <SOURCE_FILE>はBrainMassageコードを記述したソースファイル名
# -o <OUTPUT_FILE>はコンパイル結果を書き込むファイルの指定(省略可)
bmsgc -t <SOURCE_FILE> -o <OUTPUT_FILE>
```
より詳細な使い方については、各コマンドを`--help`オプションを付けて実行してください。  

**注意**: 現在のバージョンではBrainf\*ckへのトランスパイルのみ利用可能です。  

## Language Specification
簡易的な入門チュートリアルを[言語仕様(簡易版)](docs/tutorial.md)に解説しています。BrainMassageのソースコードを自身で新しく書く場合に参照してください。  
