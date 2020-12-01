mod method;
mod utils;
use method::{multi_thread_1, multi_thread_2, multi_thread_3, single_thread};
use utils::init_vec;

fn main() {
    const M: usize = 100;
    const N: usize = 60;
    const BLOCK_SIZE: usize = 4;
    let (a, b) = init_vec(M, N);
    single_thread(&M, &a, &b);
    multi_thread_1(&M, &a, &b);
    multi_thread_2(&M, &a, &b);
    multi_thread_3(&BLOCK_SIZE, &M, &a, &b);
}
