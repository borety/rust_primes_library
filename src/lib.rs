#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod basic {

    use std::cmp;

    pub fn is_prime(n : u64) -> bool {

        for p in 2..(1+ root_lim(n)) {
            if n % p == 0 {
                return false;
            }
        }

        return true;
    }

    pub fn hcf(n: u64, m: u64) -> u64 {

        let mut a = cmp::max(n, m);
        let mut b = cmp::min(n, m);

        if a % b == 0 { return b }

        let mut p: u64 = 2;
        let mut ans: u64 = 1;

        while p <= (1 + cmp::min(a, b)) {

            if a % p == 0 && b % p == 0 {
                ans *= p;
                a /= p;
                b /= p;
            } else {
                p += 1;
            }

        }

        return ans;
    }

    fn root_lim(a : u64) -> u64 {
        return (a as f64 + 0.5_f64).sqrt() as u64;
    }

    pub fn lcm(a: u64, b:u64) -> u64 {
        return a * b / hcf(a, b);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod is_prime_tests {
            use super::*;

            #[test]
            fn test_is_prime() {

                let prime_set: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 23];

                for p in prime_set.iter() {
                    assert!(is_prime(*p));
                }
            }
            #[test]
            fn test_is_not_prime() {
                let not_prime_set: Vec<u64> = vec![4, 6, 8, 9, 12, 15, 16, 18, 20];

                for p in not_prime_set.iter() {
                    assert!(!is_prime(*p));
                }
            }
        }

        mod hcf_tests {
            use super::hcf;
            #[test]
            fn test_hcf_coprimes() {
                assert_eq!(1, hcf(2, 3));
                assert_eq!(1, hcf(2, 9));
                assert_eq!(1, hcf(35, 11));

            }
            #[test]
            fn test_hcf_examples() {
                assert_eq!(2, hcf(4, 10));
                assert_eq!(10, hcf(10, 10));
                assert_eq!(35, hcf(35, 70));

            }
            #[test]
            fn hcf_test_vs_slow_function() {

                use std::cmp;

                fn slow_hcf(x: u64, y: u64) -> u64 {
                    let mut answer: u64 = 1;
                    for i in 1..(1 + cmp::max(x, y)) {
                        if x % i == 0 && y % i ==0 {
                            answer = i;
                        }
                    }
                    return answer;
                }

                for i in 2..100 {
                    for j in 2..100 {
                        assert_eq!(slow_hcf(i,j), hcf(i,j));
                        assert_eq!(slow_hcf(i,j), hcf(j,i));
                    }
                }
                
            }

        }

        mod lcm_tests {
            use super::lcm;

            #[test]
            fn simple_tests() {
                assert_eq!(6, lcm(2, 3));
                assert_eq!(6, lcm(3, 2));
                assert_eq!(30, lcm(10, 15));
                assert_eq!(12, lcm(6, 12));
            
            }
        }
    }

    


}

pub mod prime_sieves {

    pub fn primes_to(n: u32) -> Vec<u32> {
        
        let mut primes: Vec<u32> = vec![2, 3];

        // Work out the maximum value we need to actually do elimination for
        // This is equal to square root of the limit (translated into it's array location)

        // The maximum number that needs to be checked
        let max_num = (n as f64 + 0.5).sqrt() as u32;
        let max_check_val = get_nearest_array_pos(max_num);

        let array_len = 2 + (n / 3) as usize;

        let mut is_prime_array : Vec<u8>= vec![0; array_len];

        for p in 1..=max_check_val {

            if is_prime_array[p] == 0 {
                // If the number is prime then strike out multiples above number**2
                let num = array_pos_to_num(p);
                
                // Add the prime to the overall array
                primes.push(num);

                match p % 2 {
                    0 => {
                        // Case where number has rem of 1 mod 6
                        
                        // Square the number and find start position in the array
                        // POTENTIAL TO MAKE MORE EFFICIENT
                        let mut q = num_to_arr_pos(num.pow(2)).unwrap();

                        // Get the increases for the iteration
                        let d1: usize = (2*((4 * num) / 6) + 1) as usize;
                        let d2: usize = (2*((2 * num) / 6) + 1) as usize;

                        while q < array_len {

                            is_prime_array[q] = 1;
                            q += d1;

                            if q >= array_len { break }

                            is_prime_array[q] = 1;
                            q += d2;
                        }

                    },
                    _ => {
                        // Case where number has rem of 5 mod 6
                        
                        // Square the number and find start position in the array
                        // POTENTIAL TO MAKE MORE EFFICIENT
                        let mut q = num_to_arr_pos(num.pow(2)).unwrap();

                        // Get the increases for the iteration
                        let d2: usize = (2*((4 * num) / 6) + 1) as usize;
                        let d1: usize = (2*((2 * num) / 6) + 1) as usize;

                        while q < array_len {
                            is_prime_array[q] = 1;
                            q += d1;

                            if q >= array_len { break }

                            is_prime_array[q] = 1;
                            q += d2;
                        }
                    }
                }
            }
        }

        for p in (1 + max_check_val)..array_len {
            if is_prime_array[p] == 0 { 
                let x = array_pos_to_num(p);
                if x < n { primes.push(x) }
            }
        }

        // Finally return the primes!
        return primes;
    }


    fn get_nearest_array_pos(n: u32) -> usize {
        // Gets the nearest array position to that number
        let cycle = 2*((n-1) /6);
        match n % 6 {
            1..=4 => return cycle as usize,
            _ => return (cycle + 1) as usize
        }

    }

    fn num_to_arr_pos(x : u32) -> Option<usize> {
        if x % 6 == 1 || x % 6 == 5 {
            return Some((x / 3) as usize);
        } else {
            return None;
        }

    }

    fn array_pos_to_num(x: usize) -> u32 {
        return ((x / 2) * 6 + 1 + (x % 2) * 4) as u32;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod test_array_tools {
            use super::num_to_arr_pos;
            use super::get_nearest_array_pos;
            use super::array_pos_to_num;
            
            #[test]
            fn test_array_pos() {
                assert_eq!(0, num_to_arr_pos(1).unwrap());
                assert_eq!(1, num_to_arr_pos(5).unwrap());
                assert_eq!(2, num_to_arr_pos(7).unwrap());
                assert_eq!(3, num_to_arr_pos(11).unwrap());
                assert_eq!(4, num_to_arr_pos(13).unwrap());
                assert_eq!(5, num_to_arr_pos(17).unwrap());
                assert_eq!(6, num_to_arr_pos(19).unwrap());

            }

            #[test]
            fn test_get_nearest_array_pos() {
                assert_eq!(get_nearest_array_pos(1), 0);
                assert_eq!(get_nearest_array_pos(2), 0);
                assert_eq!(get_nearest_array_pos(3), 0);
                assert_eq!(get_nearest_array_pos(4), 0);
                assert_eq!(get_nearest_array_pos(5), 1);
                assert_eq!(get_nearest_array_pos(6), 1);
                assert_eq!(get_nearest_array_pos(7), 2);

            }
            #[test]
            fn test_array_pos_to_num() {
                assert_eq!(array_pos_to_num(0), 1);
                assert_eq!(array_pos_to_num(1), 5);
                assert_eq!(array_pos_to_num(2), 7);
                assert_eq!(array_pos_to_num(3), 11);
                assert_eq!(array_pos_to_num(4), 13);
                assert_eq!(array_pos_to_num(5), 17);
                assert_eq!(array_pos_to_num(6), 19);
                assert_eq!(array_pos_to_num(7), 23);
                assert_eq!(array_pos_to_num(8), 25);
            }

        }

        mod test_prime_gen {
            use super::primes_to;
            use crate::basic::is_prime;


            #[test]
            fn run_test() {
                let x = primes_to(1000);

                // Checks it only returns primes!
                for p in x.iter() {
                        assert!(is_prime(*p as u64));
                }

                // Checks it returns the right number of primes
                assert_eq!(4, primes_to(10).len());
                assert_eq!(25, primes_to(100).len());
                assert_eq!(168, primes_to(1000).len());

            }
        }
    }

}
