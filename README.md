# BrainMassage (わかばカップ版PoC)
Brainf\*ckの記述を実用言語風の読みやすい形式に置き換え、トランスパイルの難易度が上がらない範囲で記述を簡略化した言語です。  

本リポジトリの[poc-20260516ブランチ](https://github.com/Bokume2/brain_massage/tree/poc-20260516)では、わかばカップ 2026のデモで使用したPoC程度のトランスパイラのみを実装します。  

## Usage
1. Cargoを含むRustツールチェーンをインストールします。  
1. リポジトリをcloneします。  
   ```bash
   git clone --branch=poc-20260515 https://github.com/Bokume2/brain_massage.git
   cd brain_massage
   ```
1. ビルドします。  
   ```bash
   cargo build --release
   ```
1. target/release/bmsgc_pocにトランスパイラの実行バイナリが作成されるので、必要に応じて任意の場所に移動します(以下では移動しない前提でコマンド類を記述します)。  
1. BrainMassageのコードを記述し、テキストファイルに保存します。または、samplesディレクトリ内のサンプルコードを利用しても構いません。  
   PoC版ではエラー処理が不十分なため、なるべく正しいコードを記述するよう注意してください。  
1. トランスパイラにBrainMassageのソースファイルを渡して実行し、Brainf\*ckコードを生成します。  
   ```bash
   # `-o`オプションは出力ファイルを指定、省略した場合は標準出力にBrainf*ckコードを出力
   target/release/bmsgc_poc samples/tamamo.bmsgc -o tamamo.bf
   ```
1. 生成したBrainf\*ckコードを任意のBrainf\*ck処理系を用いて実行します。  
   ```bash
   # 例: `bfi`というBrainf*ckインタプリタがある場合
   bfi tamamo.bf
   ```
