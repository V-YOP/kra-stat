@echo off
SETLOCAL EnableExtensions

:: 编译项目
cargo build --release

:: 计算目标目录
for /f "tokens=*" %%i in ('echo %USERPROFILE%') do set HOME_DIR=%%i
set TARGET_DIR=%HOME_DIR%\.kra_history

:: 确保目标目录存在
if not exist "%TARGET_DIR%" (
    mkdir "%TARGET_DIR%"
)

:: 复制可执行文件到目标目录
:: 注意：根据实际的项目名称和目标系统调整下面的路径和文件名
copy "target\release\kra-stat.exe" "%TARGET_DIR%\"

echo Done.
ENDLOCAL
