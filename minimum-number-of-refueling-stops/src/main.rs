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


 * Optimized search:
 * Fix cluster, each cluster takes at most N calculations. O(N^3) as
 * there are at most O(N^2) clusters. Rough estimation.
 * Optimized with memory. If we calculate up to some value, we
 * calculate for every intermediate value, so cache the results. Other
 * wise we will calculate those values more than necessary. Every
 * answer is enumerated by two indices, each of which is 500 at most.
 * So this is quadratic complexity thanks to caching.

 * Can be optimized further by using cubic dp. If we want to cache the
 * last calculated value.
 */

struct Solution {}

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {

        stations.push(vec![target, 0]);

        let mut mem = vec![];
        for i in 0..500 {
            mem.push([-1].repeat(500));
        }

        println!(" stations {:?}", stations);
        println!(" target {}", target);
        println!(" start fuel {}", start_fuel);

        if stations[0][0] > start_fuel {
            -1
        } else {
            let mut best: i32 = i32::max_value();
            for i in 0..stations.len() {
                if (start_fuel as i32) < stations[i][0] {
                    break;
                }
                let f_i = Self::f(
                    i,
                    (start_fuel - stations[i][0]) as usize,
                    &mut mem, &stations);
                if f_i == -1 {
                    continue;
                }
                best = std::cmp::min(best, f_i);
            }
            if best == i32::max_value() {
                -1
            } else {
                best
            }
        }
    }

    fn f(

        // station id
        i: usize,
        // amoung of fuel available
        j: usize,
        // mem[i][j] = answer for i-th node with enough fuel
        // to reach j-th node and no other node to the right from it.
        // -1 for non-existing values.
        mem: &mut Vec<Vec<i32>>,
        // (distance from the start, amount of fuel) for a given station
        data: &Vec<Vec<i32>>)

        -> i32 {

        /*
        unsafe {
        static mut stop: i32 = 0;
        if stop == 30 {
        panic!("max num of calls");
    } else {
        stop += 1;
    }
    }
         */

        println!("station|fuel: {} {}", i, j);
        // what is the furthest station that we might reach?
        let mut max_reach: usize = i;
        for n in i + 1..data.len() {
            println!("distance to {} is {}, and we can use {} liters of
fuel + what we currently have {}",
                     n,
                     data[n][0] - data[i][0],
                     data[i][1],
                     j
            );
            if (j as i32 + data[i][1]) as i32
                >= (data[n][0] - data[i][0]) {
                    max_reach += 1;
                } else {
                    break;
                }
        }
        println!("station|max reach: {} {}", i, max_reach);
        if max_reach == i {
            if i == data.len() - 1 {

                println!("early 0");

                return 0;
            } else {

                println!("early -1");

                return -1;
            }
        }

        if mem[i][max_reach] != -1 {
            return mem[i][max_reach];
        } else {

            println!("trying to do smth");

            let mut answer: i32 = i32::max_value();
            for n in i+1..max_reach + 1 {

                /*
                println!("can use so much fuel {} to get {} kilometers
                from here", j + data[i][1] as usize, (data[n][0] - data[i][0]));
                println!("so the left fuel is {}",
                j + (data[i][1] as usize) - (data[n][0] - data[i][0]) as usize);
                 */

                let f_n = Self::f(n,
                            j + (data[i][1] as usize) - (data[n][0] - data[i][0])
                            as usize,
                            mem, data);

                if f_n != -1 {
                    answer = std::cmp::min(
                        answer,
                        f_n + 1
                    );
                }
            }

            if answer == i32::max_value() {

                mem[i][max_reach] = -1;
                -1
            } else {
                mem[i][max_reach] = answer;
                answer
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let target = 1;
        let start_fuel = 1;
        let mut stations : Vec<Vec<i32>> = vec![];
        assert_eq!(0,
                   Solution::min_refuel_stops(target, start_fuel, stations)
        );
    }

    #[test]
    fn ex2() {
        let target = 100;
        let start_fuel = 1;
        let mut stations : Vec<Vec<i32>> = vec![vec![10, 100]];
        assert_eq!(-1,
                   Solution::min_refuel_stops(target, start_fuel, stations)
        );
    }

    #[test]
    fn ex3() {
        let target = 100;
        let start_fuel = 10;
        let mut stations : Vec<Vec<i32>>
            = vec![vec![10, 60], vec![20, 30],
                   vec![30, 30], vec![60, 40]
            ];
        assert_eq!(2,
                   Solution::min_refuel_stops(target, start_fuel, stations)
        );
    }

    #[test]
    fn lc1() {
        let target = 999;
        let start_fuel = 1000;
        let mut stations : Vec<Vec<i32>>
            = vec!
            [vec![5,100],
             vec![997,100],
             vec![998,100]];
        assert_eq!(0,
                   Solution::min_refuel_stops(target, start_fuel, stations)
        );
    }
}

fn main() {
}
