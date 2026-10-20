# Rust WinUI 3 Demo

一个使用 Rust 构建的原生 WinUI 3 桌面应用示例，基于 [`windows-rs`](https://github.com/microsoft/windows-rs) 仓库中的 `windows-reactor` 封装，以组件化的方式声明界面。

## 功能特性

- **三页面导航**：首页 / 计数器 / 控件，带返回栈的 NavigationView 导航
- **标题栏集成**：高标题栏 + 搜索框 + 主题切换按钮
- **窗口材质**：Mica 背景，支持 System / Light / Dark 主题循环切换
- **常用控件**：Button、TextBox、ToggleSwitch、Slider、ComboBox、CheckBox、ProgressBar、InfoBar、FontIcon（Segoe MDL2 Assets）
- **受控状态更新**：所有控件通过消息（Message）驱动状态并触发重新渲染

## 运行要求

- Windows 10 1809+（建议 Windows 11）
- 本机已安装 [Windows App SDK / Windows App Runtime](https://learn.microsoft.com/windows/apps/windows-app-sdk/downloads)（应用以 framework-dependent 方式运行）
- [Rust 工具链](https://rustup.rs/)（edition 2024，需要较新的 nightly/稳定版）

## 构建与运行

```bash
cargo run          # 调试运行
cargo run --release  # 发布运行（无控制台窗口）
```

## 项目结构

```
src/main.rs   全部示例代码（DemoApp 组件 + 三个页面）
Cargo.toml    依赖 windows-reactor（git 依赖，指向 windows-rs 仓库）
```

## 技术要点

- `App::run_component::<DemoApp>(())` 启动应用，界面由 `Component` trait 的 `view()` 声明式构建
- 用户交互统一转为 `Message`，在 `update()` 中修改状态，与 Elm/ELM 架构类似
- 主题、窗口尺寸、背景材质通过 `WindowVisuals` 每帧更新

## License

MIT
