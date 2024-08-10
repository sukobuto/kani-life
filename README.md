# kani-life

プログラミングハンズオンのためのちょっとしたゲーム

Web 画面はゲーム状況の表示だけで、API でコマンドを送信することで自キャラ（カニ）を操作する。

## dev

backend を起動 (0.0.0.0:8000)
```
cd backend
cargo run
```

frontend を起動 (起動ポートは標準出力される)
```
cd frontend
npm install
npm run dev
```

frontend の起動URLをブラウザで開くとゲーム画面が表示される。
コマンドは backend に対して直接送信すればOK。

## todo

- [x] ゲーム画面(frontend)の実装
    - [x] 盤面の描画
    - [x] カニの描画
    - [x] カニのご飯の描画
    - [x] WebSocket でゲーム状況を受信して反映する
- [ ] backend の実装
    - [x] 静的ファイルの提供
    - [x] コマンド処理の仕組み
    - [x] 個別のコマンド処理の実装
      - [x] spawn カニの誕生
      - [x] scan 正面に見えるものはなにか
      - [x] turn 超信地旋回
      - [x] move 右か左に1マス移動
        - [x] 移動のみ
        - [x] ご飯があったらポイントゲット
        - [x] 壁だったら | カニがいたら 移動失敗
      - [ ] paint 今いるマスに色を塗る
    - [x] ランダムイベント（ゲームサイクル）の実装
      - [x] かにのご飯のランダム出現
    - [x] コマンドAPI
    - [x] WebSocketでゲーム状況を送信
        - [x] 疎通確認用のゲーム状況送信
        - [x] 本番のゲーム状況送信
