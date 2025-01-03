# FPGA 制御ユーティリティ

## 概要

[jelly-uidmng](https://github.com/ryuz/jelly-uidmng)を使って、root権限の必要な FPGA 制御などをラッピングしています。

dfx-mgr-client を使った操作や、[こちら](https://zenn.dev/ryuz88/articles/linux_fpga_download) にあるような操作を実装しています。

おもに KV260 の Ubuntu でのみ動作確認しています。

- dfx-mgr-client 用の アクセラレータ登録や load/unload
- FPGA manager を使った bitstream のダウンロード
- DeviceTree Overlay の実施

などを、簡単に行う為のツールとしています。



## 使い方

bitstream をダウンロードする例です。

```rust
use jelly_fpgautil as fpgautil;
use jelly_uidmng as uidmng;

fn main() {
    uidmng::change_user().unwrap();
    uidmng::set_allow_sudo(true);

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <bitstream>", args[0]);
        std::process::exit(1);
    }
    let bitstream = &args[1];
    fpgautil::load_bitstream(bitstream).unwrap();
}
```

