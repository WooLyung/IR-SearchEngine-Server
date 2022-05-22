pub struct VocaNode<'a> {
    left: Option<Box<VocaNode<'a>>>,
    right: Option<Box<VocaNode<'a>>>,
    term: &'a str,
    frequency: u32,
    ptr: ()
}