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

    // Parental relationships, -1 for no parent (root)
    let mut parent: Vec<i32>
        = itertools::repeat_n::<i32>(-1, n as usize)
        .collect::<Vec<i32>>();

    let mut graph: Vec<Vec<i32>>
        = itertools::repeat_n::<Vec<i32>>(vec![], n as usize)
        .collect::<Vec<_>>();

    for e in &edges {
        graph[e[0] as usize].push(e[1]);
        parent[e[1] as usize] = e[0];
    }

    println!("checking the graph {:?}", graph);
    println!("and the parent {:?}", parent);

    // Fill in the {leaves}

    let mut q: std::collections::vec_deque::VecDeque::<i32>
        = std::collections::vec_deque::VecDeque::<i32>::new();

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

    println!(" leaves {:?}", leaves);

    // Now start moving from the leaves and fill in the
    // {hm_nodes_in_subtree}.

    for i in &leaves {
        hm_nodes_in_subtree[*i as usize] = 0;
        q.push_back(*i);
    }

    // Now fill in the {hm_nodes_in_subtree}.
    // helper memory
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

    println!(" how many nodes in subtrees {:?}", hm_nodes_in_subtree);

    // Start counting {down} values from the leaves.

    for i in &leaves {
        down[*i as usize] = 0;
        q.push_back(*i);
    }

    seen_node_times
        = itertools::repeat_n::<i32>(0, n as usize)
        .collect::<Vec<i32>>();

    while q.len() > 0 {
        let current_node = q.pop_front().unwrap();
        if parent[current_node as usize] == -1 {
            continue;
        }

        println!("!{}", current_node);

        seen_node_times[current_node as usize] += 1;
        if graph[current_node as usize].len() >
            seen_node_times[current_node as usize] as usize {
            continue;
        }

        println!("!..survived");

        // logic
        for j in &graph[parent[current_node as usize] as usize] {
            println!("!old value {}", down[parent[current_node as usize] as usize]);

            println!("!adding d = {}, c = {}",
                     down[*j as usize],
                    hm_nodes_in_subtree[*j as usize]
            );

            down[parent[current_node as usize] as usize]
                += (down[*j as usize] +
                    hm_nodes_in_subtree[*j as usize]);
            println!("!new value {}", down[parent[current_node as usize] as usize]);
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

        println!("?current node{}", current_node);

        println!("? down of parent {} - down of current node {} - hm nodes in current {}",
            down[parent[current_node as usize] as usize],
            down[current_node as usize],
            hm_nodes_in_subtree[current_node as usize]
        );

        up[current_node as usize]
            =
        // sum of distances to all nodes coming from the parent
        // as if the connection to current_node did not exist
            down[parent[current_node as usize] as usize] -
            down[current_node as usize] -
            hm_nodes_in_subtree[current_node as usize] +

        // now that we shift one node away from all those, as usual
        // we mark that we are one node away from so many nodes
            hm_nodes_in_subtree[parent[current_node as usize] as usize] -
            hm_nodes_in_subtree[current_node as usize] +

        // and finally we are at distance 1 from the old node
            1;

        for i in &graph[current_node as usize] {
            q.push_back(*i);
        }
    }

    println!(" ups {:?}", up);

    vec![]
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