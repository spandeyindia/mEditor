use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferKind {
    Text,
    Source,
    Binary,
    LargeFile,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorBuffer {
    pub path: String,
    pub kind: BufferKind,
    pub text: Option<String>,
    pub dirty: bool,
}

impl EditorBuffer {
    pub fn open_text(path: impl Into<String>, text: impl Into<String>) -> Self {
        let path = path.into();
        let text = text.into();
        let kind = classify_path(&path, text.len() as u64);
        Self {
            path,
            kind,
            text: Some(text),
            dirty: false,
        }
    }

    pub fn route(path: impl AsRef<Path>, size_bytes: u64) -> BufferKind {
        classify_path(path.as_ref().to_string_lossy().as_ref(), size_bytes)
    }
}

pub fn classify_path(path: &str, size_bytes: u64) -> BufferKind {
    if size_bytes > 20 * 1024 * 1024 {
        return BufferKind::LargeFile;
    }
    let ext = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match ext.as_str() {
        "rs" | "java" | "js" | "ts" | "tsx" | "jsx" | "py" | "php" | "c" | "cpp" | "h" | "hpp"
        | "sql" | "html" | "css" | "md" | "toml" | "yaml" | "yml" | "json" | "xml" => {
            BufferKind::Source
        }
        "png" | "jpg" | "jpeg" | "gif" | "pdf" | "zip" | "jar" | "class" => BufferKind::Binary,
        _ => BufferKind::Text,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorTab {
    pub id: String,
    pub path: String,
    pub pinned: bool,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct EditorTabSet {
    tabs: Vec<EditorTab>,
}

impl EditorTabSet {
    pub fn open_or_focus(&mut self, path: impl Into<String>) -> &EditorTab {
        let path = path.into();
        if let Some(index) = self.tabs.iter().position(|tab| tab.path == path) {
            return &self.tabs[index];
        }
        let id = format!("editor.tab.{}", self.tabs.len() + 1);
        self.tabs.push(EditorTab {
            id,
            path,
            pinned: false,
        });
        self.tabs.last().expect("tab was just pushed")
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_source_and_large_files() {
        assert_eq!(EditorBuffer::route("main.rs", 100), BufferKind::Source);
        assert_eq!(
            EditorBuffer::route("video.bin", 25 * 1024 * 1024),
            BufferKind::LargeFile
        );
    }

    #[test]
    fn open_or_focus_reuses_existing_tab() {
        let mut tabs = EditorTabSet::default();
        tabs.open_or_focus("src/main.rs");
        tabs.open_or_focus("src/main.rs");
        assert_eq!(tabs.len(), 1);
    }

    #[test]
    fn open_text_preserves_contents() {
        let buffer = EditorBuffer::open_text("src/main.rs", "fn main() {}");
        assert_eq!(buffer.text.as_deref(), Some("fn main() {}"));
        assert_eq!(buffer.kind, BufferKind::Source);
    }
}
