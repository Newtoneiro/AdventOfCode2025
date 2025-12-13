use day_11::utils::get_input;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;

fn hashmap_to_graph(map: HashMap<String, Vec<String>>) -> Graph<String, ()> {
    let mut graph = Graph::<String, ()>::new();
    let mut nodes = HashMap::<String, NodeIndex>::new();

    for key in map.keys() {
        let idx = graph.add_node(key.clone());
        nodes.insert(key.clone(), idx);
    }

    for (src, neighbors) in map {
        let src_idx = nodes[&src];

        for dst in neighbors {
            let dst_idx = *nodes
                .entry(dst.clone())
                .or_insert_with(|| graph.add_node(dst));

            graph.add_edge(src_idx, dst_idx, ());
        }
    }

    graph
}

fn count_paths(graph: &Graph<String, ()>, start: NodeIndex, end: NodeIndex, memo: &mut HashMap<NodeIndex, usize>) -> usize {
    if start == end { return 1; }
    if let Some(&count) = memo.get(&start) { return count; }

    let mut total = 0;
    for neighbor in graph.neighbors(start) {
        total += count_paths(graph, neighbor, end, memo);
    }
    memo.insert(start, total);
    total
}

fn main() {
    let routes = get_input("src/input.txt");
    
    let graph = hashmap_to_graph(routes);

    let fft: NodeIndex = graph
        .node_indices()
        .find(|&idx| graph[idx] == "fft")
        .unwrap();
    let dac: NodeIndex = graph
        .node_indices()
        .find(|&idx| graph[idx] == "dac")
        .unwrap();
    let svr: NodeIndex = graph
        .node_indices()
        .find(|&idx| graph[idx] == "svr")
        .unwrap();
    let out: NodeIndex = graph
        .node_indices()
        .find(|&idx| graph[idx] == "out")
        .unwrap();

    let mut memo_svr_fft = HashMap::new();
    let mut memo_fft_dac = HashMap::new();
    let mut memo_dac_out = HashMap::new();

    let svr_to_fft = count_paths(&graph, svr, fft, &mut memo_svr_fft);
    let fft_to_dac = count_paths(&graph, fft, dac, &mut memo_fft_dac);
    let dac_to_out = count_paths(&graph, dac, out, &mut memo_dac_out);

    let total_paths = svr_to_fft * fft_to_dac * dac_to_out;
    println!("{}", total_paths);
}