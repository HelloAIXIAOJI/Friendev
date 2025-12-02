use notify_rust::Notification;
use anyhow::Result;
use ui::get_i18n;

/// 发送AI完成输出的系统通知
pub async fn notify_ai_completed() -> Result<()> {
    let i18n = get_i18n();
    let body = i18n.get("notify_ai_completed_body");

    tokio::task::spawn_blocking(move || {
        let _ = Notification::new()
            .summary("Friendev")
            .body(&body)
            .timeout(notify_rust::Timeout::Milliseconds(5000))
            .show();
    })
    .await?;

    play_error_sound().await;
    
    Ok(())
}

/// 播放系统Err音效
async fn play_error_sound() {
    tokio::task::spawn_blocking(|| {
        play_native_sound();
    })
    .await.ok();
}

#[cfg(target_os = "windows")]
fn play_native_sound() {
    use std::process::Command;
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let _ = Command::new("powershell")
        .args(["-c", "[System.Media.SystemSounds]::Hand.Play()"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
}

#[cfg(not(target_os = "windows"))]
fn play_native_sound() {
    // 简单的终端响铃作为跨平台回退
    // Simple terminal bell as cross-platform fallback
    use std::io::Write;
    let _ = std::io::stdout().write_all(b"\x07");
    let _ = std::io::stdout().flush();
}

/// 发送通用通知
pub async fn send_notification(title: &str, body: &str) -> Result<()> {
    let title = title.to_string();
    let body = body.to_string();
    
    tokio::task::spawn_blocking(move || {
        let _ = Notification::new()
            .summary(&title)
            .body(&body)
            .timeout(notify_rust::Timeout::Milliseconds(5000))
            .show();
    })
    .await?;
    
    Ok(())
}
