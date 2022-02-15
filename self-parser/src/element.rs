#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element {
    name: String,
    attribute: Vec<(String, String)>,
    children: Vec<Element>,
}
