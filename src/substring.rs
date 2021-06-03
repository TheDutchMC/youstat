pub trait Substring {
    fn substring(&self, b: usize) -> &str;
}

impl Substring for String {
    fn substring(&self, b: usize) -> &str {
        let s: String = self.chars().skip(b).collect();
        s.as_str()
    }
}

impl Substring for &str {
    fn substring(&self, b: usize) -> &str {
        let s: String = self.chars().skip(b).collect();
        s.as_str()
    }
}