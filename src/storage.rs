use crate::model::DiaryData;
use std::{
  fs,
  io::{self, Write},
  path::{Path, PathBuf},
  sync::{Arc, Mutex, mpsc},
  thread,
  time::Duration,
};

enum Command {
  Save(DiaryData),
  Flush(mpsc::Sender<Result<(), String>>),
}
pub struct Storage {
  sender: mpsc::Sender<Command>,
  error: Arc<Mutex<Option<String>>>,
}
pub fn store_path() -> Result<PathBuf, String> {
  let dir = if let Some(dir) = std::env::var_os("DIARY_DATA_DIR") {
    PathBuf::from(dir)
  } else {
    dirs::data_dir()
      .ok_or("Cannot find the app data directory")?
      .join("com.khurram.diary")
  };
  Ok(dir.join(".diary-store.json"))
}
pub fn load(path: &Path) -> Result<DiaryData, String> {
  match fs::read(path) {
    Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| {
      format!(
        "Cannot read {}: {e}. The file has been left untouched.",
        path.display()
      )
    }),
    Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(DiaryData::default()),
    Err(e) => Err(format!("Cannot open {}: {e}", path.display())),
  }
}
fn save(path: &Path, data: &DiaryData) -> Result<(), String> {
  let write = || -> Result<(), Box<dyn std::error::Error>> {
    let parent = path.parent().ok_or("Missing data directory")?;
    fs::create_dir_all(parent)?;
    // Keep the pre-rewrite store once, before any GPUI save touches it.
    let backup = path.with_file_name(".diary-store.pre-gpui.json");
    if path.exists() && !backup.exists() {
      fs::copy(path, &backup)?;
    }
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    serde_json::to_writer_pretty(&mut temp, data)?;
    temp.write_all(b"\n")?;
    temp.as_file().sync_all()?;
    temp.persist(path)?;
    #[cfg(unix)]
    fs::File::open(parent)?.sync_all()?;
    Ok(())
  };
  write().map_err(|e| format!("Couldn't save your changes: {e}"))
}
impl Storage {
  pub fn new(path: PathBuf) -> Self {
    let (sender, receiver) = mpsc::channel();
    let error = Arc::new(Mutex::new(None));
    let worker_error = error.clone();
    thread::spawn(move || {
      let mut pending = None;
      loop {
        let message = if pending.is_some() {
          receiver.recv_timeout(Duration::from_millis(400))
        } else {
          receiver
            .recv()
            .map_err(|_| mpsc::RecvTimeoutError::Disconnected)
        };
        match message {
          Ok(Command::Save(data)) => pending = Some(data),
          Ok(Command::Flush(reply)) => {
            let result = pending
              .as_ref()
              .map(|data| save(&path, data))
              .unwrap_or(Ok(()));
            *worker_error.lock().unwrap() = result.clone().err();
            if result.is_ok() {
              pending = None;
            }
            let _ = reply.send(result);
          }
          Err(reason) => {
            if let Some(data) = pending.as_ref() {
              let result = save(&path, data);
              *worker_error.lock().unwrap() = result.clone().err();
              if result.is_ok() {
                pending = None;
              }
            }
            if reason == mpsc::RecvTimeoutError::Disconnected {
              break;
            }
          }
        }
      }
    });
    Self { sender, error }
  }
  pub fn queue(&self, data: &DiaryData) {
    let _ = self.sender.send(Command::Save(data.clone()));
  }
  pub fn flush(&self) -> Result<(), String> {
    let (sender, receiver) = mpsc::channel();
    self
      .sender
      .send(Command::Flush(sender))
      .map_err(|e| e.to_string())?;
    receiver.recv().map_err(|e| e.to_string())?
  }
  pub fn error(&self) -> Option<String> {
    self.error.lock().unwrap().clone()
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn saves_latest_snapshot_and_retains_backup() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store.json");
    fs::write(&path, b"{\"journal\":{\"2026-01-01\":\"original\"}}").unwrap();
    let storage = Storage::new(path.clone());
    let mut data = load(&path).unwrap();
    data.journal.insert("2026-01-02".into(), "first".into());
    storage.queue(&data);
    data.journal.insert("2026-01-02".into(), "latest".into());
    storage.queue(&data);
    storage.flush().unwrap();
    assert_eq!(load(&path).unwrap().journal["2026-01-02"], "latest");
    assert_eq!(
      load(&dir.path().join(".diary-store.pre-gpui.json"))
        .unwrap()
        .journal
        .len(),
      1
    );
  }
  #[test]
  fn malformed_store_is_never_silently_reset() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store.json");
    fs::write(&path, b"broken").unwrap();
    assert!(load(&path).is_err());
    assert_eq!(fs::read(&path).unwrap(), b"broken");
  }
  #[test]
  fn save_failure_is_reported() {
    let dir = tempfile::tempdir().unwrap();
    let parent = dir.path().join("file");
    fs::write(&parent, b"not a directory").unwrap();
    let storage = Storage::new(parent.join("store.json"));
    storage.queue(&DiaryData::default());
    assert!(storage.flush().is_err());
    assert!(storage.error().is_some());
  }
}
