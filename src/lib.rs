use std::cmp::{Ord, PartialOrd, Eq, PartialEq, Ordering};
pub struct OrdTag<T> {
    pub id: usize,
    pub data: T,
}
impl<T> PartialEq for OrdTag<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
impl<T> Eq for OrdTag<T> {}
impl<T> PartialOrd for OrdTag<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
impl<T> Ord for OrdTag<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}