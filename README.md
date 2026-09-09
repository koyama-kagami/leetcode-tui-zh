# 力扣终端中文版
> 本项目基于 [Akarsh Jain（akarsh1995）](https://github.com/akarsh1995) 的 [leetcode-tui](https://github.com/akarsh1995/leetcode-tui) 修改，原项目采用 MIT 许可证。本仓库由 koyama-kagami 维护中文适配，重点支持力扣中国站、Hot 100 和面试经典 150。原作者版权声明保留于 [LICENSE](LICENSE)。

在终端中浏览中文题目、编写 Python 3 等语言的解答，并在力扣运行或提交。

默认进入 **Hot 100**，第二个分类是 **面试经典 150 题**。按 `1` / `2` 直接切换，按官方学习计划顺序刷题，标题显示当前题单的完成进度。题单已于 2026-09-08 从力扣官方核对，分别包含 100 / 150 道题。

## 启动

Windows 使用已编译版本时，将 `leetui.exe` 与 `启动中文版.cmd` 放在同一目录，然后双击启动脚本。仓库不包含预编译程序，请先从源码编译。按 `e` 选择 `python3` 后即可编写 Python 解答。

编译：`cargo build --release`。Windows 运行 `target\release\leetui.exe`。

首次启动会生成配置并显示其位置。填入同一站点浏览器中的 `LEETCODE_SESSION` 和 `csrftoken`：

```toml
site = "cn"
lc_session = "你的 LEETCODE_SESSION"
csrftoken = "你的 csrftoken"
```

默认使用力扣中国站。国际站使用 `site = "com"`，并替换为国际站的 Cookie 后重启。Cookie 不要提交到仓库。

更新登录 Cookie 后，进入程序按 `*` 同步个人完成状态。未登录时可以浏览公开题目，运行和提交需要有效登录。

中国站优先显示中文题名、题面和分类；没有翻译时显示英文原文。菜单和帮助使用中文。两站题库缓存、解答目录相互独立，旧文件保留。升级后首次运行会重新同步题库。

## 操作

| 按键 | 操作 |
| --- | --- |
| `1` / `2` | Hot 100 / 面试经典 150 |
| `t` / `T` | 下一个 / 上一个分类 |
| `j` / `k` 或方向键 | 选择题目 |
| `Enter` | 阅读题面 |
| `/` | 搜索中文题名、英文 slug、分类或题号 |
| `e` | 选择语言并打开编辑器；Python 请选择 `python3` |
| `R` | 运行样例 |
| `s` | 提交解答 |
| `d` | 每日一题 |
| `r` | 随机题目 |
| `Ctrl+s` | 显示 / 隐藏统计 |
| `*` | 同步题库 |
| `c` | 打开配置 |
| `?` | 帮助 |
| `Esc` | 关闭弹窗或搜索 |
| `q` / `Ctrl+c` | 退出 |

编辑器由 `EDITOR` 环境变量指定。Windows 默认可使用记事本，或在启动前设置 `$env:EDITOR = 'code --wait'`。建议使用 Windows Terminal 和支持中文的字体。搜索输入依赖终端的中文输入法支持，也可粘贴中文。

## 验证

`cargo test --workspace` 运行离线测试。联网只读验证示例见 `leetcode-core/examples/check_cn.rs`；运行和提交需要有效登录 Cookie。
