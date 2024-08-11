# kani-life Player Command API

- Ping サーバーとの接続確認
- Spawn 自カニを出現させる
- Scan 正面になにが見えるか調べる
- Turn その場で左右に回転する
- Walk 左右に1マス移動する
- Paint 今いるマスを塗る

## コマンドAPI

コマンドはすべて共通の API エンドポイントにて送信します。

例

request:
```
POST {origin}/api/command
Content-Type: application/json

{"type": "Ping"}
```

response:
```
200 OK
{"type": "Pong"}
```


## コマンドの型

```typescript
type Command =
| {
    // サーバーとの接続確認
    type: "Ping"
}
| {
    // 自カニを出現させる
    type: "Spawn"
    name: string
    hue: number
}
| {
    // 正面になにが見えるか調べる
    type: "Scan"
    token: string
}
| {
    // その場で左右に回転する
    type: "Turn"
    token: string
    side: "Right" | "Left"
}
| {
    // 左右に1マス移動する
    type: "Walk"
    token: string
    side: "Right" | "Left"
}
| {
    // 今いるマスを塗る
    type: "Paint"
    token: string
}
```

## コマンドの結果の型

```typescript
type CommandResult =
| {
    // Ping に対する応答
    type: "Pong"
}
| {
    type: "Spawn"
    // 出現したカニを操作するためのトークン
    token: string
}
| {
    type: "Scan"
    whatYouCanSee: "Food" | "Crab" | "Wall"
}
| {
    type: "Turn"
}
| {
    type: "Walk"
    // 成功したか否か（壁やカニにぶつかると失敗する)
    success: bool
    // 移動した結果ごはんをGetしたらそのサイズに応じたポイント (0~3)
    point: number
    // 今の合計ポイント
    totalPoint: number
}
| {
    type: "Paint"
    // 成功したか否か(ポイントがない場合は失敗する)
    success: bool
    // 現在塗れているマスの配列
    yourPaints: Position[]
    // 今の合計ポイント
    totalPoint: number
}
| {
    // token に一致するカニが見つからなかったときのエラー
    type: "CrabNotFound"
}

type Position = {
    x: number
    y: number
}
```

## コマンドごとの説明

### Ping

サーバーとの接続を確認するためのコマンドです。レスポンスがはありますが、特に何もしません。

### Spawn

カニをフィールドに召喚します。位置や向いている方角はランダムです。  
同じ `name` でもう一度 `Spawn` すると、新たに召喚されます。取得したポイントは 0 になり、ペイントもリセットされます。

コマンドパラメータ:

- `name: string`
    - カニ名です。同じ名前のカニは1つまで召喚できます。自分の名前などを指定して遊びましょう。
- `hue: number`
    - カニの色相です。`350.0` にすると茹で上がったような真っ赤なカニになります。

コマンド結果:

- `token: string`
    - カニを操作するためのトークンです。操作系のコマンドで使うのでとっておきましょう。

### Scan

カニが向いている方角一直線上になにが見えるかを確認します。複数のものがあっても、カニから見て最初に見えるもののみわかります。影に隠れているものは見えません。

コマンドパラメータ:

- `token: string`
    - `Spawn` の結果で得られる、カニを操作するためのトークンです。

コマンド結果:

- `whatYouCanSee`
    - `"Food"`
        - ごはんが見えます。
    - `"Wall"`
        - 壁が見えます。つまり、ごはんはありません。
    - `"Crab"`
        - カニが見えます。

### Turn

カニが今向いている方角から、左右に90度回転します。

コマンドパラメータ:

- `token: string`
    - `Spawn` の結果で得られる、カニを操作するためのトークンです。
- `side`
    - `"Right"`
        - 右に90度回転
    - `"Left"`
        - 左に90度回転

### Walk

カニが今向いている方角に対し、左右に1マス移動します。

コマンドパラメータ:

- `token: string`
    - `Spawn` の結果で得られる、カニを操作するためのトークンです。
- `side`
    - `"Right"`
        - 右に1マス移動
    - `"Left"`
        - 左に1マス移動

コマンド結果:

- `success: bool`
    - 壁やカニにぶつかると移動失敗となり、 `false` になります。
- `point: number`
    - 移動により得られたポイントです。移動先にごはんがあるとポイントゲットです。ご飯は大中小あり、それぞれ 3pt, 2pt, 1pt です。
- `totalPoint: number`
    - 現在の合計ポイントです。

### Paint

カニが今いるマスを、カニの `hue` で塗ります。1マス塗るごとに1ポイント消費します。

コマンドパラメータ:

- `token: string`
    - `Spawn` の結果で得られる、カニを操作するためのトークンです。

コマンド結果:

- `success: bool`
    - ポイントがない場合に失敗となり、 `false` になります。
- `yourPaints: Position[]`
    - 今までにペイントし、まだ残っているマスの配列です。
- `totalPoint: number`
    - 現在の合計ポイントです。
