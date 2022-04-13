use std::num::ParseFloatError;

/* Wrap each error with info */
#[derive(Debug)]
pub enum Error {
    VersionError,
    ArgumentError,
    ParseFloatError,
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Self::ParseFloatError
    }
}

#[derive(Debug)]
pub struct JavascriptVersion(f32);

impl JavascriptVersion {
    pub fn from_string(s: &str) -> Result<Self, Error> {
        let split = s.trim().split("javascript").collect::<Vec<&str>>();

        match split.len() {
            1 => Ok(Self(f32::default())),
            2 => {
                if let version @ ("1.0" | "1.1" | "1.2" | "1.3" | "1.4" | "1.5") = split[1] {
                    Ok(Self(version.parse()?))
                } else {
                    Err(Error::VersionError)
                }
            }
            _ => Err(Error::ArgumentError),
        }
    }

    pub fn into_string(self) -> String {
        let mut base = "javascript".to_owned();
        if self.0 != f32::default() {
            base.push_str(&self.0.to_string())
        }
        base
    }
}

pub enum Application {
    Javascript,
    Ecmascript,
    XEcmascript,
    XJavascript,
}

pub enum Text {
    Javascript,
    Ecmascript,
    // Javascript(JavascriptVersion)
}
