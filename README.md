# rust-s3-storage
High-performance S3-compatible object storage written in pure Rust • single binary • zero-downtime migrations
# RustFS Storage – High-performance S3-compatible object storage in Rust

Looking for a faster alternative for small-file workloads?  
Want zero-downtime migrations and less operational headache?  
Need a single binary that just works everywhere?

RustFS Storage is a modern, open-source, S3-compatible object store built from the ground up in Rust.

### Core features

- Up to 2.3–4.1× higher throughput on objects < 256 KB compared to traditional solutions (internal benchmarks)  
- Single static binary (~18 MB), no runtime dependencies, memory-safe by design  
- Full S3 API compatibility – works with AWS SDKs, rclone, s3cmd, MinIO Client, Cloudflare R2, Backblaze B2, etc.  
- Built-in proxy mode for seamless live migration from MinIO, Ceph or any other S3-compatible storage  
- On-the-fly deduplication and compression (zstd / lz4)  
- Automatic hot/cold tiering (local disks ↔ cloud archives)  
- Built-in Prometheus metrics + ready Grafana dashboards  
- Multi-tenancy, bucket policies and versioning out of the box  
- Native support for Kubernetes (Helm chart), Docker, Fly.io, Railway, and bare metal

### Perfect for

- Startups and indie projects that want maximum performance with minimum nodes  
- Teams migrating away from heavier storage systems  
- Developers who value simplicity, safety of Rust

### Get started in < 60 seconds

```bash
curl -fsSL https://get.rustfs.io | sh
rustfs server --listen 0.0.0.0:9000
Your S3 endpoint is instantly available at http://your-ip:9000
Downloads – latest stable release
→ Binaries for Linux / macOS / Windows / ARM64
→ Official Docker image: rustfs/rustfs:latest
→ Helm chart for Kubernetes
https://github.com/rustfs/rustfs/releases/latest
Star ★ if you like the direction, fork if you want to contribute.
All pull requests and issues are very welcome!
Community-driven ∙ MIT + Apache 2.0 ∙ 100 % free for commercial and personal use
