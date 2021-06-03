pub trait Substring {
    fn substring(&self, b: usize) -> String;
}

impl Substring for String {
    fn substring(&self, b: usize) -> String {
        let s: String = self.chars().skip(b).collect();
        s
    }
}

impl Substring for &str {
    fn substring(&self, b: usize) -> String {
        let s: String = self.chars().skip(b).collect();
        s
    }
}