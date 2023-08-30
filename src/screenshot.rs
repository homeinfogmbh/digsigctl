use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use std::io::Cursor;
#[allow(clippy::module_name_repetitions)]
#[cfg(target_family = "unix")]
pub use unix::take_screenshot;
#[cfg(target_family = "windows")]
pub use windows::take_screenshot;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct ScreenshotResponse(anyhow::Result<Vec<u8>>);

impl<'r, 'o: 'r> Responder<'r, 'o> for ScreenshotResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        match self.0 {
            Ok(image) => Response::build()
                .header(ContentType::PNG)
                .status(Status::Ok)
                .sized_body(image.len(), Cursor::new(image))
                .ok(),
            Err(error) => {
                let msg = error.to_string();
                Response::build()
                    .header(ContentType::Text)
                    .status(Status::InternalServerError)
                    .sized_body(msg.len(), Cursor::new(msg))
                    .ok()
            }
        }
    }
}

impl From<anyhow::Result<Vec<u8>>> for ScreenshotResponse {
    fn from(result: anyhow::Result<Vec<u8>>) -> Self {
        Self(result)
    }
}

#[cfg(target_family = "unix")]
mod unix {
    use crate::systemctl;
    use std::fs::File;
    use std::io::Read;
    use subprocess::ExitStatus;

    const SCREENSHOT_SERVICE: &str = "screenshot.service";
    const SCREENSHOT_FILE: &str = "/tmp/screenshot.png";

    /// Take a screenshot of the running Chromium browser running in Cage
    ///
    /// # Errors
    /// Returns an `[anyhow::Error]` if screenshot could not be taken.
    pub fn take_screenshot() -> anyhow::Result<Vec<u8>> {
        systemctl::start(SCREENSHOT_SERVICE)?;

        while systemctl::is_active(SCREENSHOT_SERVICE)
            .map_or(false, |exit_status| exit_status == ExitStatus::Exited(0))
        {}

        let mut buffer = Vec::new();
        let mut file = File::open(SCREENSHOT_FILE)?;
        let _ = file.read(&mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(target_family = "windows")]
mod windows {
    pub fn take_screenshot() -> anyhow::Result<Vec<u8>> {
        todo!()
    }
}
