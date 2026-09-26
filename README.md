# BrainMassage
Brainf\*ckの記述を実用言語風の読みやすい形式に置き換え、トランスパイルの難易度が上がらない範囲で記述を簡略化した言語です。  

## Using in Docker
複雑なビルド準備をせずに、BrainMassageを手軽に体験するためのDockerコンテナ定義を用意しています。  
Dockerをインストールした上で、リポジトリを`git clone`等でコピーし、リポジトリのルート(最上位ディレクトリ)で以下のコマンドを実行してコンテナをビルドします。(Linux以外では動作確認していませんが、Windowsなどの非UNIX系環境では`docker`以降のコマンドのみ実行してください)  
```bash
UID=$(id -u) GID=$(id -g) docker compose build
```
glibcベースのLinuxを使用している場合、次のコマンドの`<ARGS...>`に通常通りコマンドライン引数(ソースファイル名など)を渡せば、`bmsgc`コンパイラが実行できます。  
```bash
docker compose run -q --rm compiler bmsgc <ARGS...>
```
または、次のコマンドで対話シェルを起動すると、そのシェルの中で`bmsgc`コマンドが使用可能です。ホストOSがLinuxでないなどコンパイルによって出力された実行可能ファイルがコンテナ外で上手く動かない場合、このシェルの中で動かすようにすると正常に実行できる可能性が高いです。  
```bash
docker compose run --rm compiler bash
```
ただし、Dockerのボリューム機能を利用する都合上、特に設定しない限りリポジトリ以下のファイルしか参照できないようにしているため、入力ファイルと出力ファイルはすべてリポジトリ以下のパス(例えばリポジトリルート)に準備するようにしてください。  

## Build and Installation
### Prerequisites
このプログラムのビルドには、LLVM 20.1およびそれに対応するlibPollyが必要です。  
APTから取得する場合、例えば以下のコマンドでインストールが可能です。  
```bash
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 20 all
# 任意でインストールスクリプトを削除
rm llvm.sh
```
APTが使えないその他の環境では、LLVMプロジェクトのホームページなどを参考にインストールしてください。  

### Build from source
リポジトリのルートで以下のコマンドを実行すると、target/releaseディレクトリ内に処理系の実行可能ファイルが生成されます。  
```bash
cargo build --release
```
あるいは、同じくリポジトリルートで以下のコマンドを実行すると、Cargoに設定されたインストール先(Linux環境のデフォルトでは$HOME/.cargo/bin)に実行可能ファイルがインストールされます。  
```bash
cargo install --path . --offline
```

## Requirements
ビルド済みバイナリの実行には、オブジェクトファイルを実行可能ファイルにコンパイル可能な任意のCコンパイラが必要です。  
gccなどお好きなCコンパイラをインストールし、次のコマンドでオブジェクトファイル`tmp.o`を実行可能ファイル`<EXE>`にコンパイル可能な状態にしてください。  
```bash
cc tmp.o -o <EXE>
```
ただし、Cコンパイラがなくても、実行可能ファイルへのコンパイル機能以外は利用可能です。  

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

## Contact
不具合や機能提案など作者へのご連絡は[Twitter(現X)](https://x.com/boku_renraku)やその他までお気軽にお声掛けください。  
