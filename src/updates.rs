use crate::app::Diary;
use gpui_kit::*;
use serde::Deserialize;
#[derive(Debug, Default, PartialEq)]
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
  #[serde(default)]
  draft: bool,
  #[serde(default)]
  prerelease: bool,
}
fn check() -> Result<UpdateState, String> {
  check_at(
    "https://api.github.com/repos/khurrambhutto/diary/releases/latest",
    env!("CARGO_PKG_VERSION"),
  )
}
fn check_at(endpoint: &str, current: &str) -> Result<UpdateState, String> {
  let config = ureq::Agent::config_builder()
    .timeout_global(Some(std::time::Duration::from_secs(15)))
    .build();
  let agent: ureq::Agent = config.into();
  let body = agent
    .get(endpoint)
    .header("User-Agent", "Diary")
    .header("Accept", "application/vnd.github+json")
    .call()
    .map_err(|e| e.to_string())?
    .body_mut()
    .read_to_string()
    .map_err(|e| e.to_string())?;
  release_state(&body, current)
}
fn release_state(body: &str, current: &str) -> Result<UpdateState, String> {
  let release: Release = serde_json::from_str(body).map_err(|e| e.to_string())?;
  let version = semver::Version::parse(
    release
      .tag_name
      .strip_prefix('v')
      .unwrap_or(&release.tag_name),
  )
  .map_err(|e| e.to_string())?;
  let current = semver::Version::parse(current).map_err(|e| e.to_string())?;
  if release.draft || release.prerelease || !version.pre.is_empty() {
    return Ok(UpdateState::Current);
  }
  if version.cmp_precedence(&current).is_gt() {
    let expected = format!(
      "https://github.com/khurrambhutto/diary/releases/tag/{}",
      release.tag_name
    );
    if release.html_url != expected {
      return Err("The update server returned an unexpected release link.".into());
    }
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

#[cfg(test)]
mod tests {
  use super::{UpdateState, check, check_at, release_state};
  use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
  };

  fn release(tag: &str) -> serde_json::Value {
    serde_json::json!({
      "tag_name": tag,
      "html_url": format!("https://github.com/khurrambhutto/diary/releases/tag/{tag}"),
      "draft": false,
      "prerelease": false
    })
  }

  #[test]
  fn newer_release_opens_the_matching_download_page() {
    assert_eq!(
      release_state(&release("v0.2.0").to_string(), "0.1.5").unwrap(),
      UpdateState::Available {
        version: "0.2.0".into(),
        url: "https://github.com/khurrambhutto/diary/releases/tag/v0.2.0".into()
      }
    );
    assert!(matches!(
      release_state(&release("0.3.0").to_string(), "0.2.0").unwrap(),
      UpdateState::Available { .. }
    ));
  }

  #[test]
  fn equal_older_and_build_only_versions_do_not_offer_an_update() {
    for tag in ["v0.1.5", "v0.2.0", "v0.2.0+build.2"] {
      assert_eq!(
        release_state(&release(tag).to_string(), "0.2.0").unwrap(),
        UpdateState::Current
      );
    }
  }

  #[test]
  fn drafts_and_prereleases_are_not_stable_updates() {
    for field in ["draft", "prerelease"] {
      let mut release = release("v0.3.0");
      release[field] = true.into();
      assert_eq!(
        release_state(&release.to_string(), "0.2.0").unwrap(),
        UpdateState::Current
      );
    }
    assert_eq!(
      release_state(&release("v0.3.0-beta.1").to_string(), "0.2.0").unwrap(),
      UpdateState::Current
    );
  }

  #[test]
  fn invalid_responses_report_errors() {
    for body in ["not json", "{}", &release("vv0.3.0").to_string()] {
      assert!(release_state(body, "0.2.0").is_err());
    }
    for url in [
      "https://example.com/diary",
      "https://github.com/khurrambhutto/diary/releases/../../other",
      "https://github.com/khurrambhutto/diary/releases/tag/v0.1.5",
    ] {
      let mut release = release("v0.3.0");
      release["html_url"] = url.into();
      assert!(release_state(&release.to_string(), "0.2.0").is_err());
    }
  }

  fn serve(status: &str, body: String) -> (String, thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}/latest", listener.local_addr().unwrap());
    let status = status.to_owned();
    let task = thread::spawn(move || {
      let (mut stream, _) = listener.accept().unwrap();
      stream
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .unwrap();
      let mut request = Vec::new();
      let mut buffer = [0; 1024];
      while !request.windows(4).any(|part| part == b"\r\n\r\n") {
        let count = stream.read(&mut buffer).unwrap();
        assert!(count > 0);
        request.extend_from_slice(&buffer[..count]);
      }
      write!(stream, "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
      String::from_utf8(request).unwrap()
    });
    (endpoint, task)
  }

  #[test]
  fn http_check_sends_github_headers_and_parses_the_response() {
    let (endpoint, task) = serve("200 OK", release("v0.3.0").to_string());
    assert!(matches!(
      check_at(&endpoint, "0.2.0").unwrap(),
      UpdateState::Available { .. }
    ));
    let request = task.join().unwrap().to_ascii_lowercase();
    assert!(request.starts_with("get /latest http/1.1\r\n"));
    assert!(request.contains("user-agent: diary\r\n"));
    assert!(request.contains("accept: application/vnd.github+json\r\n"));
  }

  #[test]
  fn http_errors_and_invalid_json_are_reported() {
    for (status, body) in [("403 Forbidden", "rate limited"), ("200 OK", "not json")] {
      let (endpoint, task) = serve(status, body.into());
      assert!(check_at(&endpoint, "0.2.0").is_err());
      task.join().unwrap();
    }
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    drop(listener);
    assert!(check_at(&endpoint, "0.2.0").is_err());
  }

  #[test]
  #[ignore = "contacts GitHub only when explicitly requested"]
  fn live_update_check() {
    assert!(matches!(
      check().unwrap(),
      UpdateState::Current | UpdateState::Available { .. }
    ));
  }
}
