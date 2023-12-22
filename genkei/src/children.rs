
/// A trait for elements that can have children.
pub trait Children where Self: Sized {
    type Output;
    type Child;

    /// Add a child.
    fn child(self, child: impl Into<Self::Child>) -> Self::Output;

    /// Add multiple children.
    fn children(self, children: impl IntoIterator<Item = impl Into<Self::Child>>) -> Self::Output;
}
