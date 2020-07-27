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

fn f(index: usize, fuel: i32,
     // cached answer for given amount of fuel,
     c_ans: &Vec<i32>, c_fuel: &Vec<i32>,
     // (distance from the start, amount of fuel)
     stations: &Vec<(usize, i32)>) -> i32 {

    if fuel < c_fuel[station] {
        if c_ans[station] == -1 {
            c_and[station] = i32::max_value();
        }

        for (i, val) in stations.iter().enumerate() {
            let fuel_cost = val.0 - stations[index];
            if c_
            if fuel_cost > fuel {
                continue;
            }
            c_and[station] = std::cmp::min(
                c_and[station],
                f(c_and[station],
                  fuel - fuel_cost,
                  c_fuel,
                  c_ans,
                  stations
            ));
        }
    } else {
        // also need to recalculate
    }

}

fn main() {
    println!("Hello, world!");
}
