# bilibili-webhook

[![build badge](https://github.com/LJason77/bilibili-webhook/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/LJason77/bilibili-webhook/actions/workflows/rust.yml)
![GitHub forks](https://img.shields.io/github/forks/LJason77/bilibili-webhook?style=social)
![GitHub Repo stars](https://img.shields.io/github/stars/LJason77/bilibili-webhook?style=social)

> 通过 webhook 自动下载 B站 视频。

## 介绍

bilibili-webhook 是受到 [RSSHub](https://github.com/DIYgod/RSSHub "Everything is RSSible")、[flowerss-bot](https://github.com/indes/flowerss-bot "一个支持应用内阅读的 Telegram RSS Bot") 和 [download-webhook](https://github.com/DIYgod/download-webhook "Download files through webhook") 的启发而诞生的。

相比 [download-webhook](https://github.com/DIYgod/download-webhook "Download files through webhook")，bilibili-webhook 不需要 IFTTT，不需要公网 ip 或域名，只需 [RSSHub](https://github.com/DIYgod/RSSHub "Everything is RSSible")，在内网即可使用。但目前功能上仅针对 B站 的视频，后期视需求可能会兼容其他站点。

## 安装（docker）

```
git clone --depth=1 https://github.com/LJason77/bilibili-webhook.git
cd bilibili-webhook
docker build -t bilibili-webhook .
```

## 配置

容器内有两个重要的挂载点：`/app/config` 和 `/app/downloads`，前者存放配置以及日志，后者是存放下载的视频。

将 *config.toml.example* 复制并重命名为 *config.toml* 放在将要挂载 `/app/config` 的目录下。

如果需要下载 4K 视频，可在运行命令中 `-e` 附上大会员的 **SESSDATA**，具体可查看 [bilili](https://github.com/SigureMo/bilili) 的项目说明。如果没有 **SESSDATA**，即下载普通的 1080P 视频。

## 运行

```
docker run -d --restart always --name bilibili-webhook -e SESSDATA=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA -v /mnt/downloads:/app/downloads -v /mnt/Data/bilibili-webhook:/app/config bilibili-webhook
```

## 许可

[![996.icu](https://img.shields.io/badge/link-996.icu-red.svg)](https://996.icu)
[![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg)](https://github.com/996icu/996.ICU/blob/master/LICENSE)
![GitHub](https://img.shields.io/github/license/LJason77/bilibili-webhook)
