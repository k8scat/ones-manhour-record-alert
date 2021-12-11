# ones-manhour-record-alert

ONES 工时登记提醒机器人

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
