pub trait ProcessNode<V> {
    type Output;
    fn process(self, visitor: &mut V) -> Self::Output;
}

pub trait NodeKind {
    /// Iterate through text.
    ///
    /// The function will return a false when it is time to stop iterating.
    ///
    /// Note that the [`Pattern`](std::str::pattern::Pattern) trait is not stabilized
    fn iter_text<'slf, F>(&'slf self, func: &mut F) -> bool
    where
        F: FnMut(&'slf str) -> bool;

    /// Determined if this contains a string. Note that the `Pattern` trait is unsable, so we use a string here.
    fn contains(&self, pattern: &str) -> bool {
        let mut closure = |text: &str| text.contains(pattern);
        self.iter_text(&mut closure)
    }
}

pub fn iter_node_children_text<'n, N, F>(content: impl Iterator<Item = &'n N>, func: &mut F) -> bool
where
    N: NodeKind + 'n,
    F: FnMut(&'n str) -> bool,
{
    for node in content {
        if !node.iter_text(func) {
            return false;
        }
    }
    true
}
