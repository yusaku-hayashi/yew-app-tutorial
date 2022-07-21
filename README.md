# Yew Tutorial

https://yew.rs/ja/docs/tutorial
を実施した時のメモ

## development server の実行

```
trunk serve --open --proxy-backend=https://yew.rs/tutorial
```

## Yew 上における html 表記の注意点

1. 式(Expression)は`{}`で囲まれている必要がある
1. ルートノードは一つ。コンテナで囲まずに複数の要素を表現したい時はフラグメント（空タグ、`<> ... </>`）を使う
1. 要素は必ず適切に閉じられていること

```html
<h1>RustConf Explorer</h1>
<div>
    <h3>Videos to watch</h3>
    <p>John Doe: Building and breaking things</p>
    <p>Jane Smith: The development process</p>
    <p>Matt Miller: The Web 7.0</p>
    <p>Tom Jerry: Mouseless development</p>
</div>
<div>
    <h3>John Doe: Building and breaking things</h3>
    <img
        src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder"
        alt="video thumbnail"
    />
</div>
```

を変換すると

```rust
html! {
    <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{"Videos to watch"}</h3>
            <p>{ "John Doe: Building and breaking things" }</p>
            <p>{ "Jane Smith: The development process" }</p>
            <p>{ "Matt Miller: The Web 7.0" }</p>
            <p>{ "Tom Jerry: Mouseless development" }</p>
        </div>
        <div>
            <h3>{ "John Doe: Building and breaking things" }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    </>
}
```

マークアップ内で Rust の構文を利用できる

## Component の種類

２種類ある。React の Class component と Function component みたいな感じ

-   Struct Component
-   Function Components

### Function Components の引数（props）

-   引数を一つ取ることができる
-   引数に渡す構造体は`Properties`トレイトを実装する必要がある（`derive`する）
    -   v0.19 以降、`Properties`トレイトの実装には`PartialEq`の実装が必要（prop の値を比較して、必要な時にのみ再レンダリングするため）

```rust
#[derive(Properties, PartialEq)]
struct MyProp { data: String, another_data: u32 }

#[function_component(MyFnComp)]
fn my_fn_comp(prop: &MyProp) -> Html { /* snip */ }
// 基本パターン記法使ってpropは受け取る
fn my_fn_comp(MyProp { data, another_data }: &MyProp) -> Html { /* snip */ }
```

prop は以下のように渡す

```rust
html! {
    <>
        // ...
        <MyFnComp data={data} another_data={another_data} />
    </>
}
```

# css

yew 自体はビルトインのスタイル適用方法を提供していないので、crate を導入するか、trunk の機能を使う
詳細は[Trunk のアセット](https://trunkrs.dev/assets/)を参照

```html
<!-- index.html -->
<link data-trunk ref="scss" href="assets/style.scss" />
```
