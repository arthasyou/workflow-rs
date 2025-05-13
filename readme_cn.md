# 🧠 workflow-rs 项目说明

`workflow-rs` 是一个用 Rust 编写的通用工作流引擎，融合了图执行模型、可扩展节点系统、数据上下文传递与处理器机制，目标是构建一个清晰、模块化、可运行的工作流框架。

---

## ✨ 核心设计理念

1. **职责清晰，模块解耦：** 各模块单一职责，彼此通过接口协作。
2. **图驱动执行：** 整体流程通过 DAG（有向无环图）控制节点执行顺序。
3. **节点可插拔：** 所有节点实现 `Executable` trait，可灵活组合与扩展。
4. **数据驱动流转：** 节点执行依赖输入数据，由上下游节点传递。
5. **Processor 插件机制：** 支持对节点输入输出进行通用处理（如日志、校验、转换等）。

---

## 🗂️ 目录结构

```bash
workflow-rs/
├── graph/            # 图结构与编译逻辑
├── model/            # 数据模型（Node 定义、Context 等）
├── node/             # 所有节点定义与核心 trait
│   ├── task/         # 执行型节点（如 PromptNode）
│   └── orchestration/# 控制型节点（如 BranchNode）
├── processor/        # Processor 插件模块（日志、校验等）
│   ├── processors/   # 已实现的 processor 类型
│   ├── traits.rs     # ProcessorTrait 定义
│   ├── registry.rs   # 注册与获取 Processor 的逻辑
│   └── mod.rs        # processor 模块入口
├── runner.rs         # 工作流执行器 Runner，控制执行调度
├── workflow.rs       # 工作流启动封装入口
└── example_workflow.rs # 示例运行代码
```

---

## 🧱 各模块功能说明

### 1. `graph/`

用于定义工作流图的结构：包括节点集合 (`node_data`)、边集合 (`edges`)、前后继映射 (`predecessors`, `successors`)。`compile()` 方法用于构建图拓扑结构并检查环。

### 2. `model/`

定义各种运行时模型：

- `Node`：用于描述静态配置（类型、参数）。
- `Context`：运行时上下文，包含所有节点实例和共享数据（如 metadata）。
- `NodeOutput`：统一的节点输出包装。

### 3. `node/`

节点模块，包含所有可执行节点类型。

- 所有节点实现 `Executable` trait。
- 分为两类：

  - `task/`：执行型节点（如 PromptNode，生成内容）
  - `orchestration/`：控制型节点（如 BranchNode，用于条件分支）

### 4. `processor/`

用于输入输出处理的插件模块。

- 定义了通用 `ProcessorTrait` 接口（可扩展为异步）。
- `ProcessorRegistry` 支持动态注册与获取。
- 内含多个 `Processor` 示例，如 LoggingProcessor、ValidationProcessor。

### 5. `runner.rs`

工作流执行器，控制节点调度：

- 管理队列与前驱节点状态。
- 支持输入数据累积与输出传递。
- 调用节点的 `execute()` 方法依次执行整张图。

### 6. `workflow.rs`

封装统一启动入口，调用链路为：

```
Graph -> compile() -> Context::from_graph -> Runner::run()
```

适用于业务方直接调用，无需关心底层模块组合。

### 7. `example_workflow.rs`

工作流使用示例：

- 构建节点与图。
- 执行 `Workflow::start()` 启动运行。
- 用于验证各模块是否能协同运行。

---

## ✅ 当前进展

- [x] Graph 构建与拓扑排序
- [x] Node 结构与 Executable 执行封装
- [x] PromptNode / BranchNode 实现
- [x] Context 构建机制
- [x] Runner 控制执行流程
- [x] Workflow 启动封装
- [x] Processor 模块结构搭建（traits + registry）

---

## 🚧 待开发功能

- [ ] ParallelNode、AggregatorNode、RepeatNode
- [ ] ProcessorTrait 异步支持
- [ ] Processor 执行顺序链路
- [ ] 节点状态跟踪（Running/Failed）
- [ ] Task 异常捕获与重试机制

---

## 📌 使用说明

```rust
let graph = build_graph(); // 构建 Graph
let input = json!("Start");
Workflow::start(graph, input).await.unwrap();
```
