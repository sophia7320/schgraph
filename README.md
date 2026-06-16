# SchoolGraph

基于 Rust + ratatui 的终端图可视化与最短路径计算工具。数据结构课程设计作品。

## 功能

| 视图 | 说明 |
|------|------|
| 场所描述 | 列出所有地点及其介绍 |
| 邻接矩阵 | 表格展示图的邻接矩阵（∞ 表示不可达） |
| 邻接表 | 每个节点的邻居及对应边权 |
| 最短路查询 | 输入起点终点编号，实时显示最短路径与距离，支持 Canvas 可视化、索引映射与详情三种子视图 |
| TSP 回路 | Held-Karp 算法求最优旅行商回路，Canvas 绿色高亮显示 |

## 运行

```bash
# 编译
cargo build --release

# 运行（需指定数据文件）
cargo run --release -- -f input.txt
```

## 数据文件格式

```
<节点数 N>
<节点0名称> <节点0描述>
<节点1名称> <节点1描述>
...
<边数 M>
<起点名称> <终点名称> <权值>
...
```

示例见 `input.txt`（16 节点校园拓扑数据）。

## 操作

| 按键 | 功能 |
|------|------|
| `j` / `↓` | 菜单向下 / 选中项下移 |
| `k` / `↑` | 菜单向上 / 选中项上移 |
| `Enter` / `Tab` | 确认选择 |
| `Esc` / `q` | 返回上级菜单 / 退出程序 |
| `h` / `l` | 最短路视图中切换子视图 |

## 算法

- **Floyd-Warshall**：全源最短路径，O(n³)，前驱矩阵路径重建
- **Held-Karp**：状态压缩 DP 求解 TSP，O(n²·2ⁿ)，`u64::MAX / 2` 作无穷大避免加法溢出

## 运行测试

```bash
cargo test
```

## 技术栈

- **语言**：Rust（edition 2024）
- **TUI**：ratatui 0.30 + crossterm 0.29
- **错误处理**：color-eyre

## 项目结构

```
src/
├── main.rs                  # 入口，命令行参数解析
├── lib.rs                   # 库根
└── base/
    ├── mod.rs               # 模块声明
    ├── app.rs               # App 状态、事件循环、文件装配
    ├── events.rs            # 事件通道（mpsc），独立线程 poll
    ├── filescanner.rs       # 文件分词解析器
    ├── graph.rs             # 图数据结构，Floyd-Warshall，Held-Karp TSP
    ├── tui.rs               # 终端初始化/恢复
    ├── update.rs            # Action 枚举 + 按键→操作翻译
    └── ui/
        ├── mod.rs           # View 路由 + 渲染分派
        ├── main_menu.rs     # 主菜单
        ├── place_desc.rs    # 场所描述
        ├── adj_matri.rs     # 邻接矩阵
        ├── adj_list.rs      # 邻接表
        ├── shortest_path.rs # 最短路查询（输入控件 + Canvas + 详情）
        └── tsp.rs           # TSP 回路展示
```

## 许可

MIT
