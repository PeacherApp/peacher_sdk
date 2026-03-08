pub trait ProcessNode<V> {
    type Output;
    fn process(self, visitor: &mut V) -> Self::Output;
}

pub trait NodeKind {
    /// Iterate through text.
    ///
    /// The function will return a false when it is time to stop iterating.
    ///
    /// As a text node, only return the fn value if you actually have text to search. otherwise, return true.
    fn iter_text<'slf, F>(&'slf self, func: F) -> bool
    where
        F: FnMut(&'slf str) -> bool;

    /// Determined if this contains a string. Note that the `Pattern` trait is unsable, so we use a string here.
    fn contains(&self, pattern: &str) -> bool {
        self.iter_text(|text| text.contains(pattern))
    }
}

pub fn iter_node_children_text<'n, N, F>(content: impl Iterator<Item = &'n N>, mut func: F) -> bool
where
    N: NodeKind + 'n,
    F: FnMut(&'n str) -> bool,
{
    for node in content {
        if !node.iter_text(&mut func) {
            return false;
        }
    }
    true
}
