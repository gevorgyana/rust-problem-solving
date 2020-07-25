pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>)
                                -> Vec<i32> {

    let mut down: Vec<i32>
        = itertools::repeat_n::<i32>(0, n as usize)
        .collect::<Vec<i32>>();
    let mut up: Vec<i32>
            = itertools::repeat_n::<i32>(0, n as usize)
        .collect::<Vec<i32>>();
    let mut graph: Vec<Vec<i32>>
        = itertools::repeat_n::<Vec<i32>>(vec![], n as usize)
        .collect::<Vec<_>>();

    for e in &edges {
        graph[e[0] as usize].push(e[1]);
    }

    println!("{:?}", graph);

    vec![]
}

fn main() {
    let edges = vec![
        vec![
            // always 2 items in an inner array
            0, 0
        ],
        vec![
            0, 0
        ]
    ];
    sum_of_distances_in_tree(1, edges);
}
