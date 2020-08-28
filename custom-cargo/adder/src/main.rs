use rand;
use reducer;
use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(0, 10);
    println!("hello, world! {} reduce one is {}", num, reducer::reduce(num));
}
