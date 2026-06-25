use crate::content::warframe::index::NameIndex;

pub trait Listable {
    fn list(&self, index: &NameIndex) -> String;
}