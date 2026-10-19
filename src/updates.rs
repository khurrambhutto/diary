use crate::app::Diary;
use gpui_kit::*;
use serde::Deserialize;
#[derive(Default)]
pub enum UpdateState {
  #[default]
  Idle,
  Checking,
  Current,
  Available {
    version: String,
    url: String,
  },
  Error(String),
}
#[derive(Deserialize)]
struct Release {
  tag_name: String,
  html_url: String,
}
fn check() -> Result<UpdateState, String> {
  let config = ureq::Agent::config_builder()
    .timeout_global(Some(std::time::Duration::from_secs(15)))
    .build();
  let agent: ureq::Agent = config.into();
  let body = agent
    .get("https://api.github.com/repos/khurrambhutto/diary/releases/latest")
    .header("User-Agent", "Diary")
    .header("Accept", "application/vnd.github+json")
    .call()
    .map_err(|e| e.to_string())?
    .body_mut()
    .read_to_string()
    .map_err(|e| e.to_string())?;
  let release: Release = serde_json::from_str(&body).map_err(|e| e.to_string())?;
  let version =
    semver::Version::parse(release.tag_name.trim_start_matches('v')).map_err(|e| e.to_string())?;
  let current = semver::Version::parse(env!("CARGO_PKG_VERSION")).map_err(|e| e.to_string())?;
  if version > current
    && release
      .html_url
      .starts_with("https://github.com/khurrambhutto/diary/releases/")
  {
    Ok(UpdateState::Available {
      version: version.to_string(),
      url: release.html_url,
    })
  } else {
    Ok(UpdateState::Current)
  }
}
impl Diary {
  pub fn check_updates(&mut self, cx: &mut Context<Self>) {
    if matches!(self.update, UpdateState::Checking) {
      return;
    }
    self.update = UpdateState::Checking;
    cx.notify();
    self.update_task = Some(cx.spawn(async |weak, cx| {
      let result = cx.background_executor().spawn(async { check() }).await;
      let _ = weak.update(cx, |this, cx| {
        this.update = result.unwrap_or_else(UpdateState::Error);
        cx.notify();
      });
    }));
  }
}
