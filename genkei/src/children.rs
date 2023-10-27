
/// A trait for adding children to a parent.
pub trait Children where Self: Sized {
    type Child;

    /// Add a child.
    fn child(self, child: impl Into<Self::Child>) -> Self;

    /// Add multiple children.
    fn children(self, children: impl IntoIterator<Item = impl Into<Self::Child>>) -> Self {
        children.into_iter().fold(self, |acc, child| acc.child(child))
    }
}
