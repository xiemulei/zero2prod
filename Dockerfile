FROM rust:1.81.0 AS builder
# 把工作目录切换到 `app`
WORKDIR /app
# 安装所需系统依赖
# RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources
RUN apt-get update && apt-get install lld clang -y
# 将工作环境中的所有文件复制到 Docker 镜像中
COPY . .
#COPY ./debian/config.toml  /usr/local/cargo
ENV SQLX_OFFLINE=true
# 开始构建二进制文件
RUN cargo build --release

# 运行时阶段
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources
RUN apt-get update \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT=production
# 当执行 `docker run` 时，启动二进制文件
ENTRYPOINT ["./zero2prod"]