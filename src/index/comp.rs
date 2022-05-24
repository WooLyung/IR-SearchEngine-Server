pub struct ListNode {
    pub link: Option<Box<ListNode>>,
    pub term: String,
    pub frequency: u32,
    pub ptr: Box<Posting>
}

pub struct Posting {
    pub link: Option<Box<Posting>>,
    pub id: u32,
    pub frequency: u32
}