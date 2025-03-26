# 智能云眠数据采集工具 [![publish](https://github.com/SmartCloudNest/smart-cloud-nest-tool/actions/workflows/publish.yml/badge.svg)](https://github.com/SmartCloudNest/smart-cloud-nest-tool/actions/workflows/publish.yml)

基于Vue3 + Tauri构建的跨平台桌面应用，用于智能云眠设备的串口数据采集与分析

## 技术栈
- **前端框架**: Vue 3 + TypeScript
- **桌面运行时**: Tauri (Rust)
- **构建工具**: Vite
- **UI组件**: Naive UI
- **串口通信**: WebSerial API + Rust后端

## 功能特性
- 📡 实时串口数据监控与采集
- 📊 数据可视化展示
- ⚡ 数据导出功能
- 🖥️ 跨平台运行（Windows/macOS/Linux）

## 快速开始

### 环境要求
- Rust 1.81.x
- bun 1.1.*

### 安装步骤
```bash
#克隆项目
gh repo clone SmartCloudNest/smart-cloud-nest-tool
#切换项目路径
cd ./smart-cloud-nest-tool
# 安装依赖
bun install
```

## 项目结构
```
├── public/                # 静态资源
│   ├── tauri.svg          # Tauri图标
│   └── vite.svg           # Vite图标
├── src/                   # 前端源码
│   ├── assets/            # 静态资源
│   ├── components/        # Vue组件
│   │   ├── DataGrid.vue   # 数据表格组件
│   │   ├── RecordPanel.vue # 记录控制面板
│   │   └── SerialPanel.vue # 串口控制面板
│   ├── hooks/             # 自定义Hook
│   ├── stores/            # Pinia状态管理
│   │   ├── port.ts        # 串口状态管理
│   │   └── record.ts      # 数据记录状态
│   ├── App.vue            # 根组件
│   ├── main.ts            # 应用入口
│   └── vite-env.d.ts      # 类型声明
│
├── src-tauri/             # Tauri后端
│   ├── src/               # Rust源码
│   │   ├── commands.rs    # 前后端通信命令
│   │   ├── serial.rs      # 串口通信实现
│   │   └── config.rs      # 应用配置
│   └── tauri.conf.json    # Tauri配置文件
│
├── package.json           # 前端依赖
├── tsconfig.json          # TypeScript配置
└── vite.config.ts         # Vite配置
```

## 配置说明
### 前端配置
- `vite.config.ts`: 构建工具配置

### 后端配置
- `src-tauri/tauri.conf.json`: 应用窗口设置/权限配置
- `src-tauri/src/config.rs`: 串口超时设置/缓存大小
- `src-tauri/src/commands.rs`: 前后端通信协议定义

## 开发指南
```bash
# 运行应用
bun tauri dev
```

## 构建命令
```bash
# 构建项目
bun tauri build
```

## 贡献指南
1. 提升权限（联系qume2005）
2. 创建特性分支 (`git checkout -b {name}/新功能`)
3. 提交修改 (`git commit -am '添加新功能'`)
4. 推送分支 (`git push origin {name}/新功能`)
5. 创建Pull Request

## 许可证
MIT License © 2025 智能云眠团队

## 常见问题
Q: 无法检测到串口设备？
A: 请检查设备是否能正常检测传感器，若确认传感器能被检测但无法连接请提交issue。
