# kani-life

プログラミングハンズオンのためのちょっとしたゲーム

Web 画面はゲーム状況の表示だけで、API でコマンドを送信することで自キャラ（カニ）を操作する。

# todo

- [ ] ゲーム画面(frontend)の実装
    - [x] 盤面の描画
    - [x] カニの描画
    - [x] カニのご飯の描画
    - [ ] WebSocket でゲーム状況を受信して反映する
- [ ] backend の実装
    - [x] 静的ファイルの提供
    - [x] コマンド処理の仕組み
    - [ ] 個別のコマンド処理の実装
      - [ ] spawn カニの誕生
      - [ ] scan 正面に見えるものはなにか
      - [ ] rotate 超信地旋回
      - [ ] move 右か左に1マス移動
        - [ ] ご飯があったらポイントゲット
        - [ ] 壁だったら | カニがいたら 移動失敗
      - [ ] paint 今いるマスに色を塗る
    - [ ] ランダムイベント（ゲームサイクル）の実装
      - [ ] かにのご飯のランダム出現
    - [x] コマンドAPI
    - [ ] WebSocketでゲーム状況を送信
        - [x] 疎通確認用のゲーム状況送信
        - [ ] 本番のゲーム状況送信
