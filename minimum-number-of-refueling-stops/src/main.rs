/*
 * Say f(station, gas), where {station} is from [0; 500], and {gas} is
 * from [0; 10^9], is the answer for a given station with given amount
 * of fuel.
 *
 * How much time does it take to calculate f1(*)? If we do not cache
 * intermediate results, each decision function f{i}(*) has O(N) options
 * to move to the next station (a rough estimation, but it's okay) and,
 * what is interesting, there are O(M) options to be in a station {i}
 * with different amounts of gas in the car tank, where M is from
 * [0; 10^9]. Therefore we simiply cannot caluclate every
 * f(station, gas) for every argument.
 *
 * A slighly optimized calculation strategy. Say we need f(i, j), we
 * either 1) have already visited station {i}, or 2) not. In case 2) we
 * need to calculate the answer for the furthest k stations, where
 * k <= 500, and cache the result like this (i, k, a), which is
 * affordable memory cost. In case 1) we also calculate the furthest
 * reachable station _k. Say we previously calculated the answer for a
 * range of values of k, k_. Use _k and k_.
 *
 * Takes O(500 * 500) memory and O(500 * 500) time.
 * Here is why the greedy approach does not work. See the picture.
 */

fn f(

    // station id
    i: usize,
    // amoung of fuel available
    j: usize,
    // mem[i][j] = answer for i-th node with the amount of fuel that
    // can reach the j-th node and no other node to the right from it.
    // -1 for non-existing values.
    mem: &mut Vec<Vec<i32>>,
    // (distance from the start, amount of fuel) for a given station
    data: &Vec<(usize, usize)>)

    -> i32 {

    // what is the furthest station that we might reach?
    let mut max_reach: usize = i;
    for n in i + 1..data.len() {
        if j >= (data[n].0 - data[i].0) {
            max_reach += 1;
        } else {
            break;
        }
    }
    if max_reach == i {
        // there is no answer - return a giant value so it is never
        // picked by any min()
        return i32::max_value();
    }

    let mut answer = i32::max_value();
    if mem[i][max_reach] != -1 {
        answer = mem[i][max_reach];
    } else {
        let mut first_missing: usize = 0;
        while mem[i][first_missing] != -1 { first_missing += 1; }
        if first_missing > 0 {
            answer = mem[i][first_missing - 1];
        }
        for n in first_missing..max_reach + 1 {
            answer = std::cmp::min(
                answer,
                // the function call should cache the values in {ans}
                f(n,
                  j - (data[n].0 - data[i].0) + data[i].1,
                  mem, data)
            );
        }
    }
    if answer == i32::max_value() {
        mem[i][j] = 0;
        0
    } else {
        mem[i][j] = answer;
        answer
    }
}

fn main() {
    let mut v = vec![];
    let mut m = vec![];
    f(0,0,&mut v, &mut m);
    println!("Hello, world!");
}
