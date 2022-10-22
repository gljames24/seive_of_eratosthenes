fn main() {
    let number = 100;
    let prime_list = prime_sieve(number);
    for i in 0..number{
        if prime_list[i] {
            print!("{i} ");
        }
    }
   
}

pub fn prime_sieve(n: usize) -> Vec<bool> {

    let mut prime = vec![true; n+1];
    prime[0] = false;
    prime[1] = false;

    for i in 2..=n{
        prime[i] = true;   
        for p in 2..=n{
            if p*p > n {break;}
            if prime[p] == true {
            let mut i = p*p;
                while i < n {
                    prime[i] = false;
                    i += p;
                }
            }

        }    
    }
    prime
}