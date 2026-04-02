use std::collections::VecDeque;



#[unsafe(no_mangle)]
pub extern "C" fn bench_internal(iters: i32){
    let mut queue = VecDeque::new();
    for i in 0..iters {
        queue.push_back(i);
    }

    while let Some(_) = queue.pop_front() {}
}