<!-- Sync Impact Report
Version change: 0.0.0 -> 1.0.0
Modified principles:
- Established "Ultra Think Principles" (Reverse Thinking, Da Vinci Planning, Subtraction)
Added sections:
- Vision & Philosophy
- Technology Stack (The Armory)
Templates requiring updates:
- .specify/templates/plan-template.md (✅ pending)
- .specify/templates/spec-template.md (✅ pending)
- .specify/templates/tasks-template.md (✅ pending)
-->
# Teleflow 2025 Constitution

## Core Principles (Ultra Think)

### I. Reverse Thinking
质疑一切惯例。我们不因为"通常是这样做"而这样做。我们会反向思考问题的本质，寻找最优雅的解法。代码只是 specs 的投影。

### II. Da Vinci Planning
先有蓝图，后有砖瓦。我们构建一个 `.spec-kit` 目录，它是帝国的"宪法"。在编写任何应用代码之前，必须先在 `.spec-kit` 中确立详细的规格说明。AI 只读取这里面的指令，而不臆测。

### III. Subtraction
少即是多，移除所有非核心依赖。我们追求极致的精简与高效。如果不绝对必要，就不引入。

## Vision & Philosophy

### Vision
构建一个具备 "耐用执行" (Durable Execution)、"混合感知" (Hybrid Perception) 与 "物理级隐身" (Physical Stealth) 的自动化帝国。它不是工具，而是数字物种。

### Directory Philosophy
我们将创建一个隐形的"大脑"目录 `.spec-kit`，它独立于 `src`，却控制着 `src`。
- `.spec-kit/`: 帝国的灵魂 (The Soul)
- `apps/`: 应用代码 (The Body)

## Technology Stack (The Armory)

### Core
- **Language**: Rust
- **Runtime**: Tokio
- **Actor System**: Ractor
- **Database**: Sqlx (SQLite)
- **Browser Automation**: Chromiumoxide

### Shell
- **Frontend/Container**: Tauri v2

### Data
- **Storage**: SQLite (WAL mode, JSONB)

### Protocol
- **Serialization**: MessagePack (Binary)

## Governance

本宪法 (.spec-kit) 拥有最高效力。所有的架构决策、领域模型和执行逻辑必须首先在 .spec-kit 中定义。
- **AI 行为准则**: AI 必须严格遵循 .spec-kit 中的指令。
- **变更管理**: 对架构的修改必须先更新 .spec-kit 中的文档，然后再修改代码。
- **代码即投影**: 代码库是规格说明书的下游产物。

**Version**: 1.0.0 | **Ratified**: 2025-11-28 | **Last Amended**: 2025-11-28
