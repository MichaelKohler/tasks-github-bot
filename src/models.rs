pub trait Taskable {
  fn get_title(&self) -> String;
  fn get_id(&self) -> String;
  fn format_body(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    pub html_url: String,
    pub title: String,
}

impl Taskable for Issue {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_id(&self) -> String {
        self.html_url.clone()
    }

    fn format_body(&self) -> String {
        self.html_url.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bug {
    pub summary: String,
    pub id: u32,
}

impl Taskable for Bug {
    fn get_title(&self) -> String {
        self.summary.clone()
    }

    fn get_id(&self) -> String {
        self.id.to_string()
    }

    fn format_body(&self) -> String {
        format!("https://bugzilla.mozilla.org/show_bug.cgi?id={}", self.id)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatedTaskIssue {
    pub id: u32,
    pub body: String,
    pub html_url: String,
}
