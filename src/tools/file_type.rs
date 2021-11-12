pub fn file_content_type(path: &str) -> Option<&'static str> {
    let (_, value) = path.rsplit_once('.')?;
    static CONTENT_TYPES: [(&str, &str); 40] = [
        ("aac", "audio/aac"),
        ("abw", "application/x-abiword"),
        ("arc", "application/x-freearc"),
        ("avi", "video/x-msvideo"),
        ("azw", "application/vnd.amazon.ebook"),
        ("bin", "application/octet-stream"),
        ("bmp", "image/bmp"),
        ("bz", "application/x-bzip"),
        ("bz2", "application/x-bzip2"),
        ("cda", "application/x-cdf"),
        ("csh", "application/x-csh"),
        ("css", "text/css"),
        ("csv", "text/csv"),
        ("doc", "application/msword"),
        (
            "docx",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        ),
        ("eot", "application/vnd.ms-fontobject"),
        ("epub", "application/epub+zip"),
        ("gz", "application/gzip"),
        ("gif", "image/gif"),
        ("htm", "text/html"),
        ("html", "text/html"),
        ("ico", "image/vnd.microsoft.icon"),
        ("ics", "text/calendar"),
        ("jar", "application/java-archive"),
        ("jpg", "image/jpeg"),
        ("jpeg", "image/jpeg"),
        ("js", "text/javascript"),
        ("json", "application/json"),
        ("mjs", "text/javascript"),
        ("mp3", "audio/mpeg"),
        ("mp4", "video/mp4"),
        ("otf", "font/otf"),
        ("png", "image/png"),
        ("pdf", "application/pdf"),
        ("php", "application/x-httpd-php"),
        ("svg", "image/svg+xml"),
        ("ttf", "font/ttf"),
        ("txt", "text/plain"),
        ("woff", "font/woff"),
        ("woff2", "font/woff2"),
    ];
    CONTENT_TYPES
        .iter()
        .fold(None, |v, (extension, content_type)| match (v, extension) {
            (Some(v), _) => Some(v),
            (None, _) if *extension == value => Some(content_type),
            _ => None,
        })
}

#[cfg(test)]
mod file_content_type {
    use crate::tools::file_type::file_content_type;

    #[test]
    fn file_type_check() {
        assert_eq!(file_content_type("123.js"), Some("text/javascript"));
        assert_eq!(file_content_type("main.css"), Some("text/css"));
        assert_eq!(file_content_type("index.html"), Some("text/html"));
        assert_eq!(file_content_type("file.json"), Some("application/json"));
    }
}
