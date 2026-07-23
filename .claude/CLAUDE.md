======== oh-my-coder ========
# Session Board 看板操作规则

本项目使用 oh-my-coder 管理会话看板。
必须通过 oh-my-coder.exe board CLI 操作看板卡片，禁止直接写 JSON。
> 💡 看板二进制路径：~\.oh-my-coder\bin\oh-my-coder.exe

## 更新时机 ⏰（最重要）

遇到触发场景时，**必须在本轮响应的末尾**（提交回答前）执行 CLI 命令。
不要等下次对话，不要等用户提醒——对话中一旦出了结论、定了配置、汇报了进度，**当场就写**。

**主动记录**：无需用户提示，按下方触发场景自动执行。

## CLI 用法速查

| 操作 | 命令 |
|------|------|
| **决策拍板** | oh-my-coder.exe board add decision <文本> --color blue --tag decision |
| **阻塞卡点 🚫** | oh-my-coder.exe board add decision <卡点描述> --color red --tag blocked |
| **配置参数** | oh-my-coder.exe board add config <键> <值> --tag config |
| **汇报进度** | oh-my-coder.exe board add progress <阶段名> --item <事项> --pct <0-100> --tag progress |
| **改进度** | oh-my-coder.exe board update progress <阶段名> --item <事项> --pct <新值> |
| **待办事项** | oh-my-coder.exe board add checklist <分组> --item <文本> [--done] --tag todo |
| **打勾/取消** | oh-my-coder.exe board update checklist <分组> --item <文本> [--done\|--undone] |
| **重要链接** | oh-my-coder.exe board add link <标题> <url> [--summary 摘要] --tag reference |
| **根因分析** | oh-my-coder.exe board add note <文本> --tag note |
| **踩坑记录 🕳️** | oh-my-coder.exe board add pitfall <标题> --desc <描述> --solution <方案> [--severity medium] --tag pitfall |
| **改徽章 🏷️** | oh-my-coder.exe board update decision <ID> [--label 文本] [--color 颜色] |
| **改 KV 🔑** | oh-my-coder.exe board update config <ID> --key 键 [--value 值] |
| **改链接 🔗** | oh-my-coder.exe board update link <ID> [--title 标题] [--url URL] [--summary 摘要] |
| **改笔记 📄** | oh-my-coder.exe board update note <ID> --text 新内容 |
| **改坑 🕳️** | oh-my-coder.exe board update pitfall <ID> [--title 标题] [--desc 描述] [--solution 方案] [--severity 级别] |
| **删卡片** | oh-my-coder.exe board delete <卡片ID> | **先用 oard list 看 ID，直接删，不用查 JSON** |
| **看列表** | oh-my-coder.exe board list [--tag <筛选标签>] |
| **搜索 🔍** | oh-my-coder.exe board search <关键词> [--tag 筛选] |
| **管理关联 🔗** | oh-my-coder.exe board link <ID> add <目标ID> [--relation 类型] [--label 说明] |

## 触发场景

遇到以下情况时**主动**执行对应 CLI 命令——**无需用户提示"记一下"**：

| 触发场景 | 卡片类型 | 示例 | 精炼原则 |
|---------|---------|------|---------|
| 决策拍板 ("决定用…"/"选…") | badge + kv | oard add decision "Redis Cluster" --color blue | ✅ 贯穿项目，记 |
| **阻塞卡点 🚫** | **badge red** | **oard add decision "等待上游 API 联调" --color red --tag blocked** | **跨会话先看到卡点** |
| 配置参数 ("端口=…"/"DB=…") | kv | oard add config "端口" "8080" | ✅ 确定后不改的，记 |
| 进度汇报 ("完成了 X%") | progress | oard add progress "API" --item "登录" --pct 80 | ⚠️ 只记大阶段 |
| 待办事项 ("还需要…") | checklist | oard add checklist "上线" --item "配置哨兵" | ✅ 跨会话待办，记 |
| 根因分析 ("根因是…") | note | oard add note "锁竞争 → 无锁队列" | ⚠️ 解决了就删 |
| 踩坑记录 ("踩了个坑"/"犯错了") | pitfall | oard add pitfall "Go map 并发写" --desc "..." --solution "..." | ⚠️ 解决了就删 |
| 重要链接 | link | oard add link "Tokio" "https://docs.rs/tokio" | ✅ 常查阅的，记 |
| 更新已有卡片 | board update | 按类型选用 adge/kv/link/
ote/pitfall | 更新内容或标签 |
| 用户说 "记这个" | 自动提炼上一条 | — | 按精炼原则判断 |
| 用户说 "改…"/"删…" | 执行修改 | — | — |

## 回答查询规则

当用户询问项目的**决策、配置、进度、待办、参考链接、根因**等情报时，**必须优先从 board.json 的卡片数据查找回答**，找不到就说看板没有记录，不要自行推测或编造。

例如：
- 「API 开发进度？」→ 查 progress-list 卡片，答出各子项百分比
- 「我们定了用什么缓存？」→ 查 badge / kv 卡片
- 「还有什么待办？」→ 查 checklist 卡片
- 「之前那个 bug 根因是什么？」→ 查 note 卡片

## 精炼原则（首要规则）

Session Board 的卡片数据记录在 .oh-my-coder/board.json，因此：
- **少而精** — 只记贯穿项目始终的关键情报，不记流水账
- **持续价值** — 每条卡片必须对当前工作有持续价值，否则归档或删除
- **去临时性** — 一次性代码细节、中间推理、修小 bug 不记录
- **自动过期** — 完成的进度、已过时的决策要及时清理或标记完成

> ⚠️ 每次写入前先问自己：这条信息值得每次对话都加载吗？
======== oh-my-coder ========
