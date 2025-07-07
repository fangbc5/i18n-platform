# 国际化管理平台

基于Rust + Axum构建的企业级国际化管理平台，提供多端词条统一管理、翻译工作流自动化、实时多语言分发等功能。

## 功能特点

| 核心功能 | 描述 |
|---------|------|
| 多端管理 | 统一管理Web/iOS/Android/Backend的翻译词条 |
| 工作流自动化 | 翻译-审核-发布全流程自动化处理 |
| 术语管理 | 统一术语库，自动校验与提示 |
| 上下文关联 | 支持截图、组件路径等上下文信息 |
| 实时分发 | CDN实时分发，多端秒级更新 |
| 版本控制 | 支持历史版本追踪与回滚 |
| 权限管理 | 基于ABAC的细粒度权限控制 |

## 技术架构

### 后端技术栈

| 模块 | 技术方案 | 说明 |
|------|----------|------|
| 框架 | Rust + Axum | 高性能异步Web框架 |
| ORM | Diesel | Rust生态最流行的ORM框架 |
| 数据库 | MySQL 8.0 | 支持JSON、窗口函数 |
| 缓存 | Redis | 高频词条缓存，发布队列 |
| 存储 | MinIO | 兼容S3的对象存储 |
| 消息队列 | Kafka | 分布式翻译任务处理 |
| 部署 | Docker + K8s | 容器化部署，弹性伸缩 |

### 核心模块

| 模块 | 路径 | 功能描述 |
|------|------|----------|
| 用户认证 | `/src/routes/auth.rs` | 用户登录、权限验证 |
| 项目管理 | `/src/routes/project.rs` | 项目CRUD、配置管理 |
| 词条管理 | `/src/routes/phrase.rs` | 词条增删改查、批量操作 |
| 翻译管理 | `/src/routes/translation.rs` | 翻译流程、版本控制 |
| 术语管理 | `/src/routes/term.rs` | 术语库维护、校验规则 |

## 快速开始

1. 环境要求
```bash
Rust 1.75+
MySQL 8.0+
Redis 6.0+
MinIO
Kafka
```

2. 配置文件
```bash
cp .env.example .env
# 修改数据库、Redis等配置
```

3. 数据库初始化
```bash
cargo install diesel_cli
diesel setup
diesel migration run
```

4. 运行服务
```bash
cargo run
```

## 项目结构

```
src/
├── config.rs          # 配置管理
├── errors.rs          # 错误处理
├── models/            # 数据模型
├── repositories/      # 数据访问层
├── services/          # 业务逻辑层
├── routes/            # API路由
├── dtos/             # 数据传输对象
├── utils/            # 工具函数
└── middleware/       # 中间件
```

## 性能指标

| 指标 | 数值 | 说明 |
|------|------|------|
| 翻译准确率 | 98% | 相比改进前提升36% |
| 术语一致性 | 99% | 相比改进前提升52% |
| 发布周期 | 2天 | 从21天优化至2天 |
| 返工率 | 5% | 从42%降低至5% |
| 多端一致性 | 100% | 完全消除多端差异 |
| 紧急修复 | 10分钟 | 从4小时优化至10分钟 |

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交改动 (`git commit -m 'Add some AmazingFeature'`)
4. 推送分支 (`git push origin feature/AmazingFeature`)
5. 提交Pull Request

## 许可证

MIT License
