# OS HomeWork

- [OS HomeWork](#os-homework)
  - [電腦環境](#電腦環境)
  - [說明](#說明)
  - [編譯檔案](#編譯檔案)
  - [資料結構](#資料結構)
  - [執行結果](#執行結果)

## 電腦環境

- 系統：Mac OSX 11.0
- 記憶體：8 GB 2133 MHz LPDDR3
- CPU：2C4T

## 說明

>使用Rust所編寫的，拆分成初始化矩陣＆轉置矩陣＆建立多執行緒的函數

## 編譯檔案

```bash=
cargo build --release && target/release/homework
```

## 資料結構

```bash=
homework
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src                    #程式碼
│   ├── main.rs            #主程式
│   ├── method
│   │   └── mod.rs         #multi thread 計算程式
│   └── utils.rs           #init vec function & transpose vec function
└── target
    ├── release
    │   ├── build
    │   ├── deps
    │   ├── examples
    │   ├── homework       #執行程式
    │   ├── homework.d
    │   └── incremental
    └── rls
        ├── CACHEDIR.TAG
        └── debug
```

## 執行結果

Round 1

```bash=
cargo build --release && target/release/homework

Single Thread (迴圈) : 401.0 us
Multi Thread (每個元素開一個Thread) : 838188.0 us
Multi Thread (每個row開一個線程) : 1884.0 us
Multi Thread (每個block開一個線程) : 799.0 us
```

Round 2

```bash=
cargo build --release && target/release/homework

Single Thread (迴圈) : 432.0 us
Multi Thread (每個元素開一個Thread) : 821365.0 us
Multi Thread (每個row開一個線程) : 1835.0 us
Multi Thread (每個block開一個線程) : 664.0 us
```

Round 3

```bash=
cargo build --release && target/release/homework

Single Thread (迴圈) : 403.0 us
Multi Thread (每個元素開一個Thread) : 813663.0 us
Multi Thread (每個row開一個線程) : 2089.0 us
Multi Thread (每個block開一個線程) : 1270.0 us
```
