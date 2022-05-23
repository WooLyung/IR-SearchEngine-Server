pub struct ListNode {
    pub link: Option<Box<ListNode>>,
    pub term: String,
    pub frequency: u32,
    pub ptr: ()
}