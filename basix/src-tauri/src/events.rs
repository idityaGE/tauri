use tauri::{AppHandle, Emitter};
use tokio::time::{sleep, Duration};

#[tauri::command(async)]
pub async fn download(app: AppHandle, url: String) {
    app.emit("download-started", &url).unwrap();
    for progress in [5, 10, 20, 40, 70, 90, 100] {
        sleep(Duration::from_secs(1)).await;
        app.emit("download-progress", progress).unwrap();
    }
    app.emit("download-complete", &url).unwrap();
}
