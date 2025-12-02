use notify_rust::Notification;
use anyhow::Result;
use std::process::Command;

/// 发送AI完成输出的系统通知
pub async fn notify_ai_completed() -> Result<()> {
    tokio::task::spawn_blocking(|| {
        let _ = Notification::new()
            .summary("Friendev")
            .body("已完成输出，请返回查看。")
            .timeout(notify_rust::Timeout::Milliseconds(5000))
            .show();
    })
    .await?;

    play_completion_sound().await;
    
    Ok(())
}

/// 播放完成提示音
async fn play_completion_sound() {
    let _ = tokio::task::spawn_blocking(|| {
        let sound_path = get_sound_path();
        
        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("powershell")
                .args(&[
                    "-NoProfile",
                    "-Command",
                    &format!("(New-Object System.Media.SoundPlayer '{}').PlaySync()", sound_path),
                ])
                .output();
        }
        
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("afplay")
                .arg(&sound_path)
                .output();
        }
        
        #[cfg(target_os = "linux")]
        {
            let _ = Command::new("ffplay")
                .args(&["-nodisp", "-autoexit", &sound_path])
                .output();
        }
    })
    .await;
}

/// 获取声音文件路径
fn get_sound_path() -> String {
    #[cfg(debug_assertions)]
    {
        "app/resources/sounds/completion.mp3".to_string()
    }
    #[cfg(not(debug_assertions))]
    {
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_default();
        exe_dir.join("../resources/sounds/completion.mp3")
            .to_string_lossy()
            .to_string()
    }
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
