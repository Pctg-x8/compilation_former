CompilationFormer
---

コンピ提出用にwavファイルをいい感じにノーマライズしてくれるマン

## ダウンロード/ビルド

[exeはGitHub Releaseから](https://github.com/Pctg-x8/compilation_former/releases)

Mac向けバイナリはないので各自でビルドしてください(気が向いたら用意するかも)
Rust開発環境を持っている人はリポジトリをクローンして`cargo build [--release]`

## 使い方

**任意のwavファイルをexeの上にドロップする**とひとまず-6dBピークでノーマライズしてくれるようになっています。
ファイル名はもとのやつの後ろに"_16_44100"がついたものになっています(例えば"premaster.wav"を渡すと自動的に"premaster_16_44100.wav"が出てくる)。

### もっと細かい使い方をする場合

コマンドプロンプトかPowerShellその他で`-p(--peak)`の後ろにピークレベルをdBで指定してください。

```PowerShell
compilation_former premaster32.wav -o premaster16.wav -p=-3
compilation_former premaster32.wav --output premaster16.wav --peak=-3
# -p "-3"という指定はできないので注意
```
