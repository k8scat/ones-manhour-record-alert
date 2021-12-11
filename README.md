# ones-manhour-record-alert

ONES 工时登记提醒机器人

## 安装

### 使用 Cargo

```bash
cargo install --version 0.1.2 ones-manhour-record-alert
```

安装目录：`$HOME/.cargo/bin`

### 下载二进制文件

https://github.com/k8scat/ones-manhour-record-alert/releases

## 使用

```bash
./ones-manhour-record-alert --config ./config.yml
```

### 配置文件

配置文件采用 yaml 的格式

```yaml
email: "" # ONES 登录邮箱
password: "" # ONES 登录密码
alert_webhook: "" # 企业微信群机器人的 webhook 地址
members:
  - "张三"
  - "李四"

# 可选项
base_api: "https://ones.ai/project/api"
days: 7
at_all: true
```
