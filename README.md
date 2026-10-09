# territ

一个用 Rust + [macroquad](https://github.com/not-fl3/macroquad) 实现的俄罗斯方块（Tetris）。

## 功能

- 完整的俄罗斯方块玩法：7 种方块、消行计分、每 10 行升一级并加速下落
- DAS 按键处理：按住左右键自动重复移动，软降加速
- 硬降（空格瞬间落底）
- 下一块预览
- 暂停 / 重开
- TOP 3 排行榜：进入前 3 名时输入玩家名字，持久化到 `~/.territ_highscores`，重启游戏记录保留

## 操作

| 按键 | 功能 |
| --- | --- |
| `←` `→` | 左右移动 |
| `↑` | 旋转 |
| `↓` | 软降 |
| `空格` | 硬降 |
| `P` | 暂停 / 继续 |
| `R` | 重新开始 |

## 构建与运行

```bash
cargo run
```

需要 Rust 工具链（建议通过 [rustup](https://rustup.rs/) 安装）。

## 项目结构

```
src/
  main.rs       # 入口、渲染、UI、排行榜输入状态机
  game.rs       # 游戏逻辑：输入处理、下落、锁定、计分
  board.rs      # 棋盘数据与碰撞检测
  tetromino.rs  # 方块形状与颜色
  scores.rs     # TOP 3 排行榜（加载/保存/插入）
assets/
  menlo_regular.ttf  # UI 字体
```

## 字体声明

`assets/menlo_regular.ttf` 由 macOS 系统自带字体 Menlo（TTC face 0）经 [fontTools](https://github.com/fonttools/fonttools) 子集化生成，仅包含 ASCII 与方向箭头字形（约 20KB）。该字体版权归 Apple 所有，**仅供个人学习使用，请勿再分发**。如需用于其他用途，请自行替换为开源字体（如 DejaVu Sans Mono）。
