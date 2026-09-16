# Prompt theo Phase: Real-time Network Security Analytics & Alerting System

> Đề tài được chia thành 6 phase, làm tuần tự. Mỗi phase là 1 prompt độc lập
> — có thể đưa riêng từng phase cho AI/dev, review xong mới sang phase kế tiếp.
> Tech stack xuyên suốt: **Rust (backend + frontend WASM) + PostgreSQL/TimescaleDB**.

---

## PHASE 0 — Kiến trúc & Thiết lập nền tảng (Setup)

```
Thiết kế kiến trúc tổng thể và khởi tạo project cho hệ thống 
"Real-time Network Security Analytics & Alerting System".

TỔNG QUAN:
Hệ thống giám sát lưu lượng mạng theo thời gian thực, phát hiện hành vi 
bất thường/tấn công (port scan, DDoS, brute-force, ARP spoofing, DNS 
tunneling...), lưu trữ log, và gửi cảnh báo qua nhiều kênh.

Kiến trúc gồm 4 thành phần:
1. Packet Capture & Analysis Engine (Rust)
2. Backend API Server (Rust)
3. Frontend Dashboard (Rust - WASM)
4. Database (PostgreSQL + TimescaleDB extension)

TECH STACK:
- Backend: Axum (hoặc Actix-web) + Tokio (async runtime)
- Packet capture: pnet hoặc pcap (libpcap binding)
- DB access: SQLx (compile-time checked queries)
- WebSocket: axum ws / tokio-tungstenite
- Auth: JWT (jsonwebtoken) + argon2
- Serialization: serde + serde_json
- Logging: tracing + tracing-subscriber
- Config: config crate + dotenvy
- Frontend: Leptos (WASM) qua Trunk, styling Tailwind CSS
- Biểu đồ: plotters-rs hoặc Chart.js qua wasm-bindgen
- DB: PostgreSQL 15+ với extension TimescaleDB
- DevOps: Docker + docker-compose

YÊU CẦU OUTPUT CỦA PHASE NÀY:
1. Cấu trúc thư mục project đầy đủ (Cargo workspace với các member: 
   /backend, /frontend, /capture-engine — giải thích lý do tách hay gộp)
2. File Cargo.toml (workspace + từng member) với dependency cụ thể kèm version
3. File docker-compose.yml khởi tạo PostgreSQL + TimescaleDB, backend, frontend
4. File .env.example liệt kê đầy đủ biến môi trường cần thiết
5. Sơ đồ kiến trúc (mô tả bằng text/mermaid) thể hiện luồng dữ liệu giữa 
   4 thành phần và trust boundary giữa chúng
6. README.md khung sườn: mục tiêu, kiến trúc, hướng dẫn cài đặt cơ bản

Chưa cần viết business logic chi tiết ở phase này — chỉ dựng khung project 
chạy được (compile thành công, docker-compose up không lỗi).
```

---

## PHASE 1 — Database Schema & Migration

```
Dựa trên kiến trúc đã thiết lập ở Phase 0, thiết kế và triển khai đầy đủ 
schema database cho hệ thống Network Security Analytics.

YÊU CẦU CÁC BẢNG (dùng TimescaleDB hypertable cho bảng time-series):

- users (id, username, email, password_hash, role[admin/analyst/viewer], 
  created_at)
- devices/network_nodes (id, ip_address, mac_address, hostname, device_type, 
  first_seen, last_seen, is_trusted)
- traffic_events (HYPERTABLE — time, src_ip, dst_ip, src_port, dst_port, 
  protocol, bytes_transferred, packet_count, flags, interface_name)
- alerts (id, rule_id, severity[low/medium/high/critical], title, 
  description, src_ip, dst_ip, detected_at, status[open/acknowledged/resolved], 
  acknowledged_by, resolved_at)
- detection_rules (id, name, rule_type[threshold/pattern/anomaly], 
  condition_json, severity, is_enabled, threshold_value, time_window_seconds)
- notification_channels (id, type[email/webhook/telegram/slack], 
  config_json, is_enabled)
- audit_logs (id, user_id, action, target, timestamp, ip_address)
- blocked_ips/blacklist (id, ip_address, reason, blocked_at, blocked_until)

YÊU CẦU OUTPUT:
1. Toàn bộ SQL migration file (dùng sqlx-cli hoặc refinery), đánh số thứ tự 
   theo timeline logic
2. Câu lệnh SELECT create_hypertable() cho bảng traffic_events, cấu hình 
   chunk_time_interval hợp lý
3. Đầy đủ indexes cho các cột hay query (src_ip, dst_ip, detected_at, 
   status, severity...)
4. Foreign keys và CHECK constraints (VD: severity chỉ nhận 4 giá trị 
   cố định)
5. Seed data mẫu (vài user, vài detection_rules mặc định, vài traffic_events 
   giả để test)
6. Struct Rust tương ứng (dùng sqlx::FromRow + serde) cho từng bảng, đặt 
   trong module models/

Tất cả query sau này PHẢI dùng SQLx compile-time checked query hoặc 
parameterized query — tuyệt đối không nối chuỗi SQL thủ công (yêu cầu 
bảo mật, chống SQL Injection theo OWASP).
```

---

## PHASE 2 — Detection Engine (Capture + Phát hiện bất thường)

```
Dựa trên schema đã có ở Phase 1, xây dựng Packet Capture & Detection Engine.

YÊU CẦU CHỨC NĂNG:

1. Capture module: đọc packet từ network interface (dùng pnet/pcap), 
   parse thông tin cơ bản (src/dst IP, port, protocol, flags, size), đẩy 
   vào internal channel (tokio::sync::mpsc hoặc broadcast) để các consumer 
   khác xử lý song song, không block

2. Simulation mode (BẮT BUỘC): vì capture thật cần quyền root/raw socket 
   và có thể không chạy được trong môi trường demo/sandbox, viết 1 module 
   sinh traffic giả lập:
   - Random traffic bình thường (background noise)
   - Kịch bản tấn công mẫu có thể bật theo lệnh: port scan, SYN flood, 
     brute-force, ARP spoof, DNS tunneling
   - Output cùng format với capture module thật, để detection engine dùng 
     chung logic

3. Detection rules (mỗi rule 1 module riêng, đọc từ event stream):
   - Port Scan Detection: 1 src_ip kết nối > N port khác nhau của cùng 
     dst_ip trong T giây
   - SYN Flood/DDoS Detection: số SYN packet đến 1 dst_ip vượt ngưỡng 
     trong khoảng thời gian ngắn
   - Brute-force Detection: nhiều failed connection tới cùng port 
     (22/SSH, 3389/RDP, 21/FTP) từ 1 IP
   - ARP Spoofing Detection: MAC address thay đổi bất thường cho cùng 1 IP
   - DNS Tunneling Detection: query DNS bất thường (độ dài, entropy cao, 
     tần suất cao)
   - Traffic Volume Anomaly: dùng z-score (moving average + standard 
     deviation) phát hiện spike bất thường

4. Mỗi rule: đọc threshold/config từ bảng detection_rules (bật/tắt được, 
   không hardcode), khi trigger → insert vào bảng alerts + gửi event qua 
   internal channel để module Alerting (Phase 4) xử lý tiếp

YÊU CẦU OUTPUT:
1. Module capture-engine hoàn chỉnh (capture thật + simulation mode)
2. Module detection/ chứa từng rule riêng biệt, dễ thêm rule mới 
   (dùng trait chung, VD: trait DetectionRule { fn evaluate(&self, event) 
   -> Option<Alert> })
3. Background task orchestration (Tokio) chạy nhiều rule song song, đọc 
   cùng 1 event stream
4. Unit test cho từng detection rule (dùng #[cfg(test)] + mock traffic data, 
   test cả case trigger đúng và không trigger nhầm)
5. Ghi rõ cách chạy demo: bật simulation mode, chọn kịch bản tấn công, 
   quan sát alert được tạo trong DB

Yêu cầu hiệu năng: pipeline phải xử lý được tối thiểu 10,000 packet/giây 
không block main thread.
```

---

## PHASE 3 — Backend API Server

```
Dựa trên schema (Phase 1) và detection engine (Phase 2), xây dựng Backend 
API Server bằng Axum.

YÊU CẦU ENDPOINTS (RESTful, có OpenAPI/Swagger doc qua utoipa crate):

Auth: 
- POST /api/auth/login, /api/auth/register, /api/auth/refresh

Dashboard:
- GET /api/dashboard/summary (tổng traffic, số alert theo severity, 
  top talkers, top ports)

Traffic:
- GET /api/traffic?from=&to=&src_ip=&protocol= (có pagination)

Alerts:
- GET /api/alerts, GET /api/alerts/:id
- PATCH /api/alerts/:id (acknowledge/resolve)

Rules:
- CRUD đầy đủ /api/rules

Devices:
- GET /api/devices, GET /api/devices/:id/history

Blocklist:
- CRUD đầy đủ /api/blocklist

WebSocket:
- GET /ws/alerts (real-time push khi có alert mới từ Phase 2)
- GET /ws/traffic (live traffic feed)

Reports:
- GET /api/reports/export (xuất PDF/CSV báo cáo theo khoảng thời gian)

YÊU CẦU KỸ THUẬT:
1. Middleware authentication (JWT) áp dụng cho toàn bộ route trừ 
   login/register
2. Middleware RBAC (Role-Based Access Control): admin/analyst/viewer có 
   quyền khác nhau, enforce ở server-side, deny by default
3. Rate limiting cho API (đặc biệt endpoint login — chống brute-force)
4. CORS config chặt chẽ (whitelist origin cụ thể, không dùng "*")
5. Error handling chuẩn hóa (dùng thiserror), response envelope thống nhất 
   dạng { success, data, error }
6. Validate/sanitize input (dùng validator crate) cho mọi request body/query
7. WebSocket handler nhận event từ internal channel (Phase 2 bắn ra) và 
   broadcast tới tất cả client đang kết nối

YÊU CẦU OUTPUT:
1. Toàn bộ route handlers theo nhóm ở trên
2. Middleware auth + RBAC + rate limit hoàn chỉnh
3. WebSocket handler cho /ws/alerts và /ws/traffic
4. OpenAPI spec tự sinh (utoipa) truy cập được qua /docs hoặc /swagger-ui
5. Integration test (dùng reqwest) cho tối thiểu: login, tạo/xem alert, 
   CRUD rule, kiểm tra RBAC chặn đúng role
```

---

## PHASE 4 — Alerting System (Đa kênh thông báo)

```
Dựa trên Backend API (Phase 3), xây dựng hệ thống gửi cảnh báo đa kênh.

LUỒNG XỬ LÝ khi 1 alert được tạo (từ Detection Engine - Phase 2):
1. Lưu vào bảng alerts trong DB (đã có từ Phase 1)
2. Broadcast ngay qua WebSocket /ws/alerts tới tất cả client dashboard 
   đang mở (dùng handler đã có ở Phase 3)
3. Kiểm tra severity của alert so với threshold cấu hình trong 
   notification_channels → nếu vượt ngưỡng, gửi qua kênh ngoài:
   - Email (dùng lettre crate, SMTP config qua .env)
   - Webhook (HTTP POST tới URL tùy chỉnh, dùng reqwest, kèm payload JSON 
     chuẩn hóa)
   - Telegram Bot API (dùng reqwest gọi Telegram Bot HTTP API, cần 
     bot_token + chat_id cấu hình trong notification_channels.config_json)
4. Cơ chế deduplication/throttling: không gửi lại cùng 1 loại alert 
   (cùng rule_id + cùng src_ip) liên tục trong khoảng X giây cấu hình được

YÊU CẦU OUTPUT:
1. Module alerting/ với trait chung cho các channel (VD: trait 
   NotificationChannel { async fn send(&self, alert: &Alert) -> Result<()> })
2. Implementation cho từng channel: EmailChannel, WebhookChannel, 
   TelegramChannel
3. Dispatcher đọc danh sách channel đang bật (is_enabled) từ DB, gửi song 
   song (tokio::join! hoặc spawn nhiều task), không để 1 channel lỗi làm 
   chậm channel khác
4. Cơ chế throttling dùng in-memory cache (VD: dashmap hoặc HashMap + 
   Mutex) lưu lần gửi gần nhất theo key (rule_id, src_ip)
5. Ghi audit_logs mỗi lần gửi alert thành công/thất bại
6. Test giả lập: trigger 1 alert mẫu, xác nhận cả 3 kênh (mock/log ra 
   console nếu không có SMTP/Telegram thật) đều nhận được đúng nội dung
```

---

## PHASE 5 — Frontend Dashboard (Leptos/WASM)

```
Dựa trên Backend API + WebSocket (Phase 3), xây dựng Frontend Dashboard 
bằng Leptos (biên dịch WASM, build bằng Trunk).

CÁC TRANG CẦN CÓ:

1. Login/Register page — gọi API auth, lưu JWT (in-memory hoặc cookie 
   HttpOnly nếu SSR), redirect sau khi login

2. Dashboard tổng quan:
   - Card thống kê: tổng packet, tổng alert 24h, top 5 IP nguồn traffic
   - Biểu đồ traffic theo thời gian (line chart, update real-time qua 
     WebSocket /ws/traffic)
   - Biểu đồ phân bố alert theo severity (pie/donut chart)

3. Trang Alerts:
   - Bảng danh sách alert, filter theo severity/status/thời gian
   - Nút acknowledge/resolve gọi PATCH /api/alerts/:id
   - Toast/notification popup real-time khi có alert mới từ /ws/alerts
   - Sound/badge notification riêng cho alert severity = critical

4. Trang Traffic Explorer:
   - Bảng traffic log chi tiết, filter nâng cao (src_ip, protocol, 
     khoảng thời gian)
   - Nút export CSV

5. Trang Detection Rules:
   - CRUD rule (form tạo/sửa, toggle bật/tắt, chỉnh threshold_value và 
     time_window_seconds)

6. Trang Devices: danh sách thiết bị/IP đã phát hiện, click xem history

7. Trang Settings: cấu hình notification_channels (email/webhook/telegram), 
   test gửi thử

YÊU CẦU KỸ THUẬT:
- Dùng Leptos signals cho reactive state
- WebSocket client (web-sys/wasm-bindgen) kết nối /ws/alerts và /ws/traffic, 
  tự động reconnect khi mất kết nối
- Biểu đồ: plotters-rs render canvas hoặc bind Chart.js qua wasm-bindgen
- Styling: Tailwind CSS, có dark mode toggle
- Responsive (mobile-friendly tối thiểu ở mức xem được, không cần tối ưu 
  hoàn hảo)
- Route guard: redirect về login nếu chưa auth, ẩn/hiện menu theo role

YÊU CẦU OUTPUT:
1. Cấu trúc component Leptos đầy đủ cho 7 trang trên
2. WebSocket client module dùng chung, tự parse event và cập nhật signal
3. API client module (wrapper gọi REST API, tự đính kèm JWT vào header)
4. 1 component mẫu đầy đủ code (khuyến nghị: AlertsTable component) để 
   làm chuẩn cho các component còn lại
5. trunk.toml + cấu hình build production
```

---

## PHASE 6 — Bảo mật (OWASP Top 10) & Hoàn thiện

```
Rà soát và bổ sung toàn bộ hệ thống (Phase 0–5) để đối chiếu với OWASP 
Top 10 (2026) - Web Application Security Risks, đồng thời hoàn thiện các 
yêu cầu phi chức năng còn lại.

YÊU CẦU BẢO MẬT THEO OWASP TOP 10 (2026):

1. Broken Access Control (rủi ro cao nhất theo OWASP 2026)
   - Rà soát lại middleware RBAC ở Phase 3: đảm bảo enforce server-side 
     cho MỌI endpoint
   - Kiểm tra ownership tài nguyên (VD: analyst A không sửa được alert 
     do analyst B xử lý nếu không đủ quyền)
   - Xác nhận nguyên tắc deny by default

2. Security Misconfiguration
   - Không hardcode secret/API key — rà lại toàn bộ .env usage
   - Tắt debug mode/verbose error ở production build
   - CORS whitelist origin cụ thể
   - Thêm security headers (CSP, X-Frame-Options, X-Content-Type-Options) 
     qua middleware
   - Docker container chạy non-root user, dùng minimal base image 
     (distroless hoặc alpine)

3. Supply Chain Security
   - Chạy `cargo audit` và `cargo deny`, xử lý các cảnh báo
   - Pin version cụ thể toàn bộ crate trong Cargo.toml (không dùng "*")

4. Injection
   - Xác nhận toàn bộ query đã qua SQLx parameterized (rà lại Phase 1-3)
   - Validate input toàn bộ endpoint (rà lại Phase 3)

5. Cryptographic Failures
   - Xác nhận argon2 cho password hash (rà Phase 3)
   - JWT có exp + refresh token riêng
   - Bật HTTPS/TLS cho toàn bộ traffic (kể cả WebSocket → WSS)

6. Authentication Failures
   - Rate limit login/register (rà lại Phase 3)
   - Khóa tài khoản tạm thời sau N lần đăng nhập sai
   - Thông báo lỗi login không lộ chi tiết (sai username/password chung 
     1 message)

7. Security Logging & Monitoring
   - Rà soát audit_logs: đảm bảo ghi đủ các hành động nhạy cảm (login, 
     đổi rule, acknowledge/resolve alert, đổi phân quyền)
   - Structured logging (tracing) có correlation ID xuyên suốt request

8. Secure Design
   - Vẽ lại data flow diagram hoàn chỉnh, đánh dấu trust boundary

CÁC YÊU CẦU PHI CHỨC NĂNG CÒN LẠI:
- Prometheus metrics endpoint /metrics (optional nhưng khuyến khích)
- README.md hoàn chỉnh: kiến trúc, hướng dẫn cài đặt, API doc, hướng dẫn 
  demo bằng simulation mode
- Test coverage tổng kết (unit + integration)

YÊU CẦU OUTPUT CUỐI CÙNG:
1. Danh sách các thay đổi/patch áp dụng lên code Phase 0-5 để đáp ứng từng 
   mục OWASP ở trên
2. Bảng mapping (markdown table) giữa từng mục OWASP Top 10 (2026) và 
   cơ chế/đoạn code cụ thể đã xử lý — dùng để đưa vào báo cáo/bảo vệ đồ án
3. README.md hoàn chỉnh cuối cùng
4. Checklist tổng kết: hệ thống đã đáp ứng đủ các mục ở Phase 0-6 hay chưa
```

---

> **Lưu ý:** Phần OWASP Top 10 (2026) ở Phase 6 được tổng hợp từ nguồn thứ
> cấp, chưa xác minh 100% với trang chính thức owasp.org/Top10. Nên kiểm tra
> lại thứ tự/tên gọi chính xác trước khi đưa vào báo cáo bảo vệ đồ án.
>
> **Cách dùng:** đưa từng Phase cho AI/dev, review kỹ output trước khi sang
> Phase tiếp theo — vì Phase sau phụ thuộc trực tiếp vào code/schema đã có
> ở Phase trước.
