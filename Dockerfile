# 使用官方Rust镜像作为构建阶段
FROM rust:latest AS builder

# 设置工作目录
WORKDIR /app

# 复制项目文件
COPY Cargo.toml config.toml ./
COPY src ./src

# 构建依赖
RUN cargo build --release

# 运行阶段使用更小的基础镜像
FROM debian:bullseye

# 安装运行时依赖
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 从构建阶段复制可执行文件
COPY --from=builder /app/target/release/wakeup-render ./

# 复制配置文件
COPY config.toml ./

# 设置默认环境变量
ENV RUST_LOG=info
ENV WAKEUP_CONFIG_PATH=/app/config.toml

# 暴露服务端口
EXPOSE 8080

# 运行应用
CMD ["./wakeup-render"]