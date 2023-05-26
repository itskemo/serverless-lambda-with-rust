# Minimal Serverless Lambda example written in Rust (macOS)

## Required:
- Node v18, brew, Serverless 3, Rust environment installed, Musl Libc

### Installing Musl Libc
- `brew install filosottile/musl-cross/musl-cross`
- `rustup target add x86_64-unknown-linux-musl`

### Installing Serverless & deps
- `npm install`

## Deploying the Lambda to AWS
- `./node_modules/.bin/serverless deploy -s development --verbose`
