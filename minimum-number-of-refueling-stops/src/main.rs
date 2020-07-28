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

static mut stop: i32 = 0;

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
    data: &Vec<(usize, usize)>)

    -> i32 {

    unsafe {
        if stop == 30 {
            panic!("max num of calls");
        } else {
            stop += 1;
        }
    }

    println!("station|fuel: {} {}", i, j);
    // what is the furthest station that we might reach?
    let mut max_reach: usize = i;
    for n in i + 1..data.len() {
        if j >= (data[n].0 - data[i].0) {
            max_reach += 1;
        } else {
            break;
        }
    }
    println!("station|max reach: {} {}", i, max_reach);
    if max_reach == i {
        if i == data.len() - 1 {
            return 0;
        } else {
            return -1;
        }
    }

    if mem[i][max_reach] != -1 {
        return mem[i][max_reach];
    } else {
        let mut answer: i32 = i32::max_value();
        for n in i+1..max_reach + 1 {
            let f_n = f(n,
                        j - (data[n].0 - data[i].0) + data[i].1,
                        mem, data);
            if f_n != -1 {
                answer = std::cmp::min(
                    answer,
                    f_n + 1
                );
            }
        }

        if answer == i32::max_value() {
            mem[i][j] = -1;
            -1
        } else {
            mem[i][j] = answer;
            answer
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let startFuel = 1;

        let mut stations = vec![];
        let target = 1;
        stations.push((target, 0));

        let mut mem = vec![];
        for i in 0..500 {
            mem.push([-1].repeat(500));
        }

        println!("{:?}", stations);

        if stations[0].0 > startFuel {
            assert_eq!(-1, -1);
        } else {
            assert_eq!(0,
                       f(0,
                         startFuel - stations[0].0,
                         &mut mem, &stations)
            );
        }
    }

    #[test]
    fn ex2() {
        let startFuel = 1;

        let mut stations = vec![(10, 100)];
        let target = 100;
        stations.push((target, 0));

        let mut mem = vec![];
        for i in 0..500 {
            mem.push([-1].repeat(500));
        }

        println!("{:?}", stations);

        if stations[0].0 > startFuel {
            assert_eq!(-1, -1);
        } else {
            assert_eq!(-1,
                       f(0,
                         startFuel - stations[0].0,
                         &mut mem, &stations)
            );
        }
    }

    #[test]
    fn ex3() {
        let startFuel = 10;

        let mut stations = vec![(10, 60), (20, 30), (30, 30),
                                (60, 40)
        ];
        let target = 100;
        stations.push((target, 0));

        let mut mem = vec![];
        for i in 0..500 {
            mem.push([-1].repeat(500));
        }

        println!("{:?}", stations);

        if stations[0].0 > startFuel {
            assert_eq!(-1, -1);
        } else {
            assert_eq!(-1,
                       f(2,
                         startFuel - stations[0].0,
                         &mut mem, &stations)
            );
        }
    }
}

fn main() {
}
