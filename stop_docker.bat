@echo off
chcp 65001 >nul
title Dừng hệ thống SecNet Docker

echo =====================================================================
echo  🛡️ SECNET: Dừng toàn bộ các container Docker
echo =====================================================================
echo.

docker compose down

echo.
echo =====================================================================
echo  ✅ Đã dừng và giải phóng tài nguyên thành công!
echo =====================================================================
pause
