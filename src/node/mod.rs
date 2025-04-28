#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub kind: NodeKind,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Prompt,
    Model,
    Retriever,
    // 以后可以加更多类型
}
