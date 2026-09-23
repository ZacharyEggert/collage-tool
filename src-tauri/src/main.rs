#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

// Renders a cols x rows grid (row-major `files`, None = empty slot) with an even `gap`
// between tiles and around the edge: each row is `+smush`ed, rows are `-smush`ed, then bordered
// and extended to the exact canvas size.
#[tauri::command]
fn render(
    files: Vec<Option<String>>,
    cols: usize,
    w: u32,
    h: u32,
    gap: u32,
    canvas_w: u32,
    canvas_h: u32,
    bg: String,
    out: String,
) -> Result<(), String> {
    let size = format!("{w}x{h}");
    let gap = gap.to_string();
    let mut args: Vec<String> = vec!["-background".into(), bg.clone(), "-gravity".into(), "center".into()];
    for row in files.chunks(cols.max(1)) {
        args.push("(".into());
        for file in row {
            args.push("(".into());
            match file {
                // -auto-orient matches the webview's EXIF rotation; resize fits, extent pads to the tile.
                Some(f) => args.extend([f.clone(), "-auto-orient".into(), "-resize".into(), size.clone(), "-extent".into(), size.clone()]),
                None => args.extend(["-size".into(), size.clone(), format!("xc:{bg}")]),
            }
            args.push(")".into());
        }
        args.extend(["+smush".into(), gap.clone(), ")".into()]);
    }
    args.extend(["-smush".into(), gap.clone(), "-bordercolor".into(), bg, "-border".into(), gap, "-extent".into(), format!("{canvas_w}x{canvas_h}"), out]);

    let output = Command::new("magick")
        .args(&args)
        .output()
        .map_err(|e| format!("failed to run magick (is ImageMagick installed?): {e}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![render])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
