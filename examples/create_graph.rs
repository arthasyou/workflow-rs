use workflow_rs::graph::Graph;

fn main() {
    // 创建一个新的图
    let graph = Graph::new_with_default_nodes().unwrap();
    println!("Graph created: {:#?}", graph);
}
