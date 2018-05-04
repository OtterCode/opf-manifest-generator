extern crate mime_guess;

use std::path::{PathBuf};
use std::string::String;
use std::fmt;

pub struct File {
    path: PathBuf,
    mimetype: String,
    filename: String
}


impl File {
    fn mime (path: &PathBuf) -> String {
        let maybe_mimetype: Option<&str> =
          path.extension()
          .and_then(|i| i.to_str())
          .and_then(mime_guess::get_mime_type_str);

        match maybe_mimetype {
            Some(mimetype) => String::from(mimetype),
            None => String::from("application/octet-stream")
        }
    }

    pub fn new (path: PathBuf) -> File {
        let mimetype = File::mime(&path);
        let filename =
            path.file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string()).unwrap();
        File { path: path, mimetype: mimetype, filename: filename }
    }

    pub fn to_string (&self) -> String {
        format!("<item id=\"fileNo{id}\" href=\"{path}\" media-type=\"{mime}\" />",
          id=self.filename,
          path=self.path.to_str().unwrap(),
          mime=self.mimetype)
    }
}
