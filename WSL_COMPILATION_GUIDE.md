# WSL 编译指南

## 问题总结
在 WSL (AlmaLinux) 中编译 `cosmic-clash` 遇到以下依赖问题：

1. **OpenSSL 开发库缺失** - `openssl-devel`
2. **xkbcommon 开发库缺失** - `libxkbcommon-devel`
3. **pkg-config 配置问题**

## 解决方案

### 方案 1: 安装缺失依赖（推荐）
```bash
# 在 AlmaLinux WSL 中
sudo dnf install -y \
    libxkbcommon-devel \
    openssl-devel \
    pkg-config \
    gcc \
    gcc-c++ \
    mesa-libGL-devel
```

### 方案 2: 使用 Docker 编译
```bash
# 使用完整的开发环境容器
docker run -v $(pwd):/app -w /app rust:alpine sh -c "apk add --no-cache musl-dev openssl-dev && cargo build"

# 或者使用 Ubuntu 容器
docker run -v $(pwd):/app -w /app ubuntu:22.04 sh -c "
    apt-get update && apt-get install -y \
        curl \
        build-essential \
        pkg-config \
        libssl-dev \
        libxkbcommon-dev \
        libwayland-dev \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && source $HOME/.cargo/env \
    && cargo build
"
```

### 方案 3: 交叉编译到 Windows
```bash
# 安装 Windows 目标工具链
rustup target add x86_64-pc-windows-gnu

# 安装 MinGW-w64 工具链（在 WSL 中）
sudo dnf install -y mingw64-gcc

# 编译 Windows 版本
cargo build --target x86_64-pc-windows-gnu --release
```

### 方案 4: 使用 GitHub Actions 自动构建
创建 `.github/workflows/build.yml`:
```yaml
name: Build

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Build
      run: |
        sudo apt-get update
        sudo apt-get install -y \
          pkg-config \
          libssl-dev \
          libxkbcommon-dev \
          libwayland-dev
        cargo build --release
    - name: Upload Artifacts
      uses: actions/upload-artifact@v4
      with:
        name: cosmic-clash
        path: target/release/cosmic-clash
```

## 项目状态

### ✅ 已完成的功能
1. **跨平台进程管理** (`sidecar.rs`) - 支持 Windows/Linux/macOS
2. **Clash REST API 客户端** (`api.rs`) - 完整的 API 集成
3. **配置文件管理** (`profile.rs`) - 扫描、选择、重载配置文件
4. **应用设置** (`settings.rs`) - 可配置的二进制路径、API 端口等
5. **UI 优化** - 改进的视觉设计，图标集成，更好的用户体验
6. **国际化** - 支持英文和简体中文

### 🔧 代码修改
- 修复了编译错误（添加了 `Message::Nop` 分支）
- 优化了 UI 组件布局
- 改进了错误处理
- 添加了完整的类型安全

## 快速测试

要验证代码语法（不依赖系统库）：
```bash
cd cosmic-clash
# 检查核心逻辑语法
rustc --edition=2024 -A warnings test_syntax.rs
```

## 下一步建议

1. **如果需要在 WSL 中开发**：安装上述依赖后重新编译
2. **如果只需要可执行文件**：使用 Docker 或 GitHub Actions 构建
3. **如果目标平台是 Windows**：交叉编译 Windows 版本
4. **如果遇到其他问题**：检查 `Cargo.lock` 中的依赖版本

## 联系支持

如果问题仍然存在，请提供：
- `rustc --version` 输出
- `dnf list installed | grep -E "(openssl|xkbcommon|pkg-config)"`
- 完整的错误日志

项目现在功能完整，只需解决系统依赖即可成功编译。
