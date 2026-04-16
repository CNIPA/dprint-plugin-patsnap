# dprint-plugin-patsnap

格式化[智慧芽(Patsnap)](https://analytics.zhihuiya.com/)专利检索表达式的 [dprint](https://dprint.dev/) 插件。

## 格式化效果

输入：
```
TTL:(空调 OR "air condition") AND TAC:(蒸发器 OR evaporator) OR IPC:F25B
```

输出：
```
    ttl:(空调 or "air condition")
and tac:(蒸发器 or evaporator)
 or ipc:F25B
```

- 顶级布尔运算符强制换行，操作数左对齐，运算符右对齐
- 内层表达式在行宽允许时保持单行，超出时自适应换行并对齐
- 字段名和布尔运算符默认转为小写

## 安装

需要先安装 [dprint CLI](https://dprint.dev/install/)。

在 `dprint.json` 中添加插件：

```json
{
  "patsnap": {},
  "includes": ["**/*.patsnap"],
  "plugins": [
    "https://github.com/CNIPA/dprint-plugin-patsnap/releases/download/v0.1.0/dprint_plugin_patsnap.wasm"
  ]
}
```

然后运行：

```bash
dprint fmt
```

## 支持的语法

- 布尔运算符: `AND`, `OR`, `NOT`, `GAND`
- 邻近算符: `$Wn`, `$PREn`, `$WS`, `$SEN`, `$PARA`
- 频率算符: `$FREQn`
- 通配符: `*`, `?`, `#`
- 范围表达式: `[20200101 TO 20231231]`
- 公司树: `TREE@"公司名"`
- 字段表达式: `TTL:keyword`, `IPC:(A61K or B01J)`
- 注释: `# 行注释`
- CJK 及多语言关键词

完整的字段列表和语法说明见 [docs/patsnap-search-reference.md](docs/patsnap-search-reference.md)。

## 配置项

所有配置项均为可选，以下为默认值：

```json
{
  "patsnap": {
    "lineWidth": 120,
    "indentWidth": 2,
    "quoteStyle": "preserve",
    "fieldCase": "lowercase",
    "booleanOperatorCase": "lowercase"
  }
}
```

| 配置项 | 可选值 | 默认值 | 说明 |
|-------|--------|-------|------|
| `lineWidth` | 数字 | `120` | 行宽限制 |
| `indentWidth` | 数字 | `2` | 缩进宽度 |
| `quoteStyle` | `double` / `single` / `preserve` | `preserve` | 引号风格 |
| `fieldCase` | `lowercase` / `uppercase` / `preserve` | `lowercase` | 字段名大小写 |
| `booleanOperatorCase` | `lowercase` / `uppercase` / `preserve` | `lowercase` | 布尔运算符大小写 |

## 忽略格式化

在不想格式化的行前加注释：

```
# patsnap-ignore
ttl:( 保持   原样 )
```

## 从源码构建

```bash
# 运行测试
cargo test

# 构建 WASM 插件
cargo build --release --target wasm32-unknown-unknown --features wasm
```

构建产物位于 `target/wasm32-unknown-unknown/release/dprint_plugin_patsnap.wasm`。

## License

[GPL-3.0](LICENSE)
