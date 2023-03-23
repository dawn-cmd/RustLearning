pub trait Summary {
    fn summary(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summary(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}
