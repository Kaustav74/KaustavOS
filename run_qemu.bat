@echo off
setlocal

cd /d "%~dp0"

echo ==========================================================
echo               KaustavOS Windows QEMU Launcher             
echo ==========================================================

REM Check if QEMU is installed on native Windows
where qemu-system-x86_64.exe >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo Native Windows QEMU detected.
    if not exist "build\kaustavos.iso" (
        echo Error: build\kaustavos.iso does not exist yet. Run build first via WSL.
        pause
        exit /b 1
    )
    if not exist "build\usb_persistence.img" (
        echo Running QEMU without secondary persistence image...
        qemu-system-x86_64.exe -cdrom build\kaustavos.iso -m 1024M -serial stdio
    ) else (
        echo Running QEMU with build\usb_persistence.img...
        qemu-system-x86_64.exe -cdrom build\kaustavos.iso -drive file=build\usb_persistence.img,format=raw -m 1024M -serial stdio
    )
    exit /b 0
)

REM If not on Windows PATH, check WSL
where wsl.exe >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo Launching KaustavOS QEMU runner inside WSL...
    wsl.exe -e bash -c "cd /mnt/d/KaustavOS/KaustavOS-main/KaustavOS-main && ./scripts/run_qemu.sh"
    exit /b 0
)

echo ERROR: Neither native QEMU nor WSL was found.
echo Please install QEMU or run through WSL.
pause
exit /b 1
