# BrainMassage
Brainf\*ckの記述を実用言語風の読みやすい形式に置き換え、トランスパイルの難易度が上がらない範囲で記述を簡略化した言語です。  

## Build and Installation
### Prerequisites
このプログラムのビルドには、LLVM 20.1およびそれに対応するlibPollyが必要です。  
APTから取得する場合、例えば以下のコマンドでインストールが可能です(`libllvm20`のバージョンは適宜置き換えてください)。  
```bash
sudo apt install libllvm20=1:20.1.8-2ubuntu8
sudo apt install libpolly-20-dev
```

また、実行可能ファイルへのコンパイル機能を利用する場合、オブジェクトファイルをコンパイル可能な任意のCコンパイラが必要です。  
gccなどいずれかのCコンパイラをインストールし、コマンド名`cc`で実行可能な状態にしてください。  

### Build from source
`git clone`等でコピーしたリポジトリのルートで以下のコマンドを実行すると、target/releaseディレクトリ内に実行可能ファイルが生成されます。  
```bash
cargo build --release
```
あるいは、同じくリポジトリルートで以下のコマンドを実行すると、Cargoに設定されたインストール先(Linux環境のデフォルトでは$HOME/.cargo/bin)に実行可能ファイルがインストールされます。  
```bash
cargo install --path . --offline
```

## Usage
BrainMassageのコードをテキストファイルに記述し、以下のコマンドでコンパイルします。出力ファイル名を省略した場合、テキスト出力であれば標準出力、バイナリ出力であればソースファイル名から拡張子変換などを行ったデフォルトの出力ファイルに出力します。  
```bash
# <SOURCE_FILE>はBrainMassageコードを記述したソースファイル名
# -o <OUTPUT_FILE>はコンパイル結果を書き込むファイルの指定(省略可)
bmsgc <SOURCE_FILE> -o <OUTPUT_FILE>
```
オプションの一部として以下が利用可能です。  
|オプション|効果|
|:---|:---|
|-t, --emit-bf|Brainf\*ckコードにトランスパイルしたものを出力します|
|--emit-llvm|テキスト形式のLLVM IRにコンパイルしたものを出力します|
|-S, --compile-only|アセンブリコードを出力します|
|-c, --compile-and-assemble-only|リンクされていないオブジェクトファイルを出力します|
|-O<OPT_LV>|数値<OPT_LV>でLLVMの最適化レベルを設定します。内容は`llc`などのコマンドと同じです|

より詳細な使い方については、`--help`オプションを付けてコマンドを実行してください。  

## Language Specification
簡易的な入門チュートリアルを[言語仕様(簡易版)](docs/tutorial.md)に解説しています。BrainMassageのソースコードを自身で新しく書く場合に参照してください。  

## Samples
samplesディレクトリにBrainMassageのサンプルコードを置いています。  
