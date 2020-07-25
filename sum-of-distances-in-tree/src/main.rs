pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>)
                                -> Vec<i32> {

    // The sum of distances from a node to all the nodes in the tree
    // below it.
    let mut down: Vec<i32>
        = itertools::repeat_n::<i32>(0, n as usize)
        .collect::<Vec<i32>>();

    // The sum of distances from a node to all the nodes in the tree
    // above it.
    let mut up: Vec<i32>
            = itertools::repeat_n::<i32>(0, n as usize)
        .collect::<Vec<i32>>();

    // How many nodes in the subtree for each of the nodes in the
    // tree.
    let mut hm_nodes_in_subtree: Vec<i32>
        = itertools::repeat_n::<i32>(0, n as usize)
        .collect::<Vec<i32>>();

    // -1 for no parent (root)
    let mut parent: Vec<i32>
        = itertools::repeat_n::<i32>(-1, n as usize)
        .collect::<Vec<i32>>();

    let mut graph: Vec<Vec<i32>>
        = itertools::repeat_n::<Vec<i32>>(vec![], n as usize)
        .collect::<Vec<_>>();

    // Queue reused for traversals
    let mut q: std::collections::vec_deque::VecDeque::<i32>
        = std::collections::vec_deque::VecDeque::<i32>::new();

    // Convert to a more useful representation

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
        = itertools::repeat_n::<i32>(0, n as usize)
        .collect::<Vec<i32>>();
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
    seen_node_times
        = itertools::repeat_n::<i32>(0, n as usize)
        .collect::<Vec<i32>>();
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

    println!("dows {:?}", down);

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

    println!(" ups {:?}", up);

    for (i, v) in up.iter().enumerate() {
        down[i] += v;
    }

    println!("res: {:?}", down);

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
        sum_of_distances_in_tree(3, edges);
    }

    // leetcode example 1

    fn ex1() {
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
        sum_of_distances_in_tree(6, edges);
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
        sum_of_distances_in_tree(7, edges);
    }
}


fn main() {

}
