pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>)
                                -> Vec<i32> {

    // The sum of distances from a node to all the nodes in the tree
    // below it.
    let mut down: Vec<i32> = [0].repeat(n as usize);

    // The sum of distances from a node to all the nodes in the tree
    // above it.
    let mut up: Vec<i32> = [0].repeat(n as usize);

    // How many nodes in the subtree for each of the nodes in the
    // tree.
    let mut hm_nodes_in_subtree: Vec<i32> = [0].repeat(n as usize);

    // -1 for no parent (root)
    let mut parent: Vec<i32> = [-1].repeat(n as usize);

    let mut graph: Vec<Vec<i32>> = vec![];
    graph.reserve(n as usize);
    for i in 0..n {
        graph.push(vec![]);
    }

    // Queue reused for traversals
    let mut q: std::collections::vec_deque::VecDeque::<i32>
        = std::collections::vec_deque::VecDeque::<i32>::new();

    // Prepare the graph
    for e in &edges {
        graph[e[0] as usize].push(e[1]);
        parent[e[1] as usize] = e[0];
    }

    // Find the leaves.
    // Let's say 0 is always the root.
    q.push_back(0);
    let mut leaves: Vec<i32> = vec![];
    while q.len() > 0 {
        let current_node = q.pop_front().unwrap();
        if graph[current_node as usize].len() == 0 {
            leaves.push(current_node);
            continue;
        }
        for next in &graph[current_node as usize] {
            q.push_back(*next);
        }
    }
    // Need the lowest nodes to be located before the highest
    // Without this, the algorithm will not work - a higher leave
    // must be processed after a lower one.
    leaves.reverse();

    // How many nodes in subtrees
    for i in &leaves {
        hm_nodes_in_subtree[*i as usize] = 0;
        q.push_back(*i);
    }
    let mut seen_node_times: Vec<i32>
        = [0].repeat(n as usize);

    while q.len() > 0 {
        let current_node = q.pop_front().unwrap();
        if parent[current_node as usize] == -1 {
            continue;
        }
        seen_node_times[current_node as usize] += 1;
        if graph[current_node as usize].len() >
            seen_node_times[current_node as usize] as usize {
            continue;
        }
        hm_nodes_in_subtree[parent[current_node as usize] as usize]
            += (hm_nodes_in_subtree[current_node as usize] + 1);
        q.push_back(parent[current_node as usize]);
    }

    // Fill {down}
    for i in &leaves {
        down[*i as usize] = 0;
        q.push_back(*i);
    }
    seen_node_times = [0].repeat(n as usize);
    while q.len() > 0 {
        let current_node = q.pop_front().unwrap();
        if parent[current_node as usize] == -1 ||
            seen_node_times[parent
                            [current_node as usize] as usize] != 0 {
            continue;
        }
        seen_node_times[parent[current_node as usize] as usize] = 1;
        for j in &graph[parent[current_node as usize] as usize] {
            down[parent[current_node as usize] as usize]
                += down[*j as usize] +
                hm_nodes_in_subtree[*j as usize] + 1;
        }
        q.push_back(parent[current_node as usize]);
    }

    // println!("dows {:?}", down);

    // Fill {up}
    for i in &graph[0] {
        q.push_back(*i);
    }
    while q.len() > 0 {
        let current_node = q.pop_front().unwrap();
        up[current_node as usize]
            =

        // Sum of distances from the parent to every node below it
        // other than {current_node} and nodes from its subtree.
            down[parent[current_node as usize] as usize] -
            down[current_node as usize] -
            hm_nodes_in_subtree[current_node as usize] - 1 +

        // Sum of distances from the parent to every node above it,
        // already calculated.
            up[parent[current_node as usize] as usize] +

        // How many nodes are not reachable from {current_node}
            n - hm_nodes_in_subtree[current_node as usize] - 1;

        for i in &graph[current_node as usize] {
            q.push_back(*i);
        }
    }

    // println!(" ups {:?}", up);

    for (i, v) in up.iter().enumerate() {
        down[i] += v;
    }

    // println!("res: {:?}", down);

    down
}


#[cfg(test)]
mod test {
    use super::*;
    fn chain() {
        let edges = vec![
            vec![
                // always 2 items in an inner array
                0, 1
            ],
            vec![
                1, 2
            ],
        ];
        assert_eq!(vec![2, 1, 0],
            sum_of_distances_in_tree(3, edges));
    }

    #[test]
    fn lc1() {
        let edges = vec![
            vec![
                0, 1
            ],
            vec![
                0, 2
            ],
            vec![
                2, 3
            ],
            vec![
                2, 4
            ],
            vec![
                2, 5
            ],
        ];
        assert_eq!(vec![8,12,6,10,10,10],
                   sum_of_distances_in_tree(6, edges));
    }

    #[test]
    fn cs1() {
        let edges = vec![
            vec![
                0, 1
            ],
            vec![
                0, 2
            ],
            vec![
                1, 3
            ],
            vec![
                1, 4
            ],
            vec![
                3, 5
            ],
            vec![
                2, 6
            ],
        ];

        assert_eq!(vec![11, 10, 14, 13, 15, 18, 19],
                   sum_of_distances_in_tree(7, edges));
    }

    #[test]
    fn lc2() {
        let edges = vec![
            vec![
                1, 0
            ],
        ];
;
        assert_eq!(vec![1,1],
                   sum_of_distances_in_tree(2, edges));
    }

    #[test]
    fn lc3() {
        let edges = vec![
            vec![
                1, 0
            ],
            vec![
                2, 0
            ]
        ];

        assert_eq!(vec![2,3,3],
                   sum_of_distances_in_tree(3, edges));
    }

    #[test]
    fn lc4() {
        let edges = vec![
            vec![
                1, 2
            ],
            vec![
                3, 2
            ],
            vec![
                3, 0
            ]
        ];

        assert_eq!(vec![6,6,4,4],
                   sum_of_distances_in_tree(4, edges));
    }
}


fn main() {

}
