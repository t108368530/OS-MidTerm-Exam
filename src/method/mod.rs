use crate::method::thread::JoinHandle;
use std::{
    sync::{mpsc::sync_channel, Arc, Mutex, MutexGuard},
    thread,
    time::Instant,
};

pub fn cal_cell(vec_1: &Vec<f64>, vec_2: &Vec<f64>) -> f64 {
    let num:usize=vec_1.len();
    let mut temp:f64=0.0;
    for i in 0..num{
        temp += vec_1[i] * vec_2[i];
    }
    return temp;
}

pub fn single_thread(m: &usize, a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) {
    let mut res: Vec<Vec<f64>> = vec![vec![0.0; *m]; *m];
    let a: Box<Vec<Vec<f64>>> = Box::new(a.clone());
    let b: Box<Vec<Vec<f64>>> = Box::new(b.clone());

    let _start: Instant = Instant::now();
    for i in 0..*m {
        for j in 0..*m {
            res[i][j] = cal_cell(&a[i], &b[j]);
        }
    }
    let end: f64 = _start.elapsed().as_micros() as f64;
    println!("Single Thread (迴圈) : {:?} us", end);
    // println!("{:?}",res);
}

pub fn multi_thread_1(m: &usize, a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) {
    let mut jobs: Vec<JoinHandle<()>> = Vec::with_capacity(*m * *m);
    let res: Vec<Vec<f64>> = vec![vec![0.0; *m]; *m];
    let mut res: Box<Vec<Vec<f64>>> = Box::new(res);
    let (sender, receiver) = sync_channel(*m * *m);
    let _start: Instant = Instant::now();
    let a: Arc<Vec<Vec<f64>>> = Arc::new(a.clone());
    let b: Arc<Vec<Vec<f64>>> = Arc::new(b.clone());

    for i in 0..*m {
        for j in 0..*m {
            // let data = Arc::clone(&res);
            let a_vec: Arc<Vec<Vec<f64>>> = Arc::clone(&a);
            let b_vec: Arc<Vec<Vec<f64>>> = Arc::clone(&b);
            let sender = sender.clone();
            jobs.push(thread::spawn(move || {
                sender.send((i, j, cal_cell(&a_vec[i], &b_vec[j]))).unwrap();
            }));
        }
    }

    for _ in 0..&*m * &*m {
        let temp: (usize, usize, f64) = receiver.recv().unwrap();
        res[temp.0][temp.1] = temp.2;
    }

    for job in jobs{
        job.join().unwrap()
    }
    let end: f64 = _start.elapsed().as_micros() as f64;

    println!("Multi Thread (每個元素開一個Thread) : {:?} us", end);
    // println!("{:?}",res);
}

pub fn multi_thread_2(m: &usize, a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) {
    let m: usize = m.clone();
    let mut jobs: Vec<JoinHandle<()>> = Vec::with_capacity(m);
    let res: Vec<Vec<f64>> = vec![vec![0.0; m]; m];

    let res: Arc<Mutex<Vec<Vec<f64>>>> = Arc::new(Mutex::new(res));
    let a: Arc<Vec<Vec<f64>>> = Arc::new(a.clone());
    let b: Arc<Vec<Vec<f64>>> = Arc::new(b.clone());

    let _start: Instant = Instant::now();
    for i in 0..m {
        let data: Arc<Mutex<Vec<Vec<f64>>>> = Arc::clone(&res);
        let a_vec: Arc<Vec<Vec<f64>>> = Arc::clone(&a);
        let b_vec: Arc<Vec<Vec<f64>>> = Arc::clone(&b);

        jobs.push(thread::spawn(move || {
            let mut data: MutexGuard<Vec<Vec<f64>>> = data.lock().unwrap();
            for j in 0..m {
                data[i][j] = cal_cell(&a_vec[i], &b_vec[j]);
            }
        }));
    }

    for job in jobs{
        job.join().unwrap()
    }
    let end: f64 = _start.elapsed().as_micros() as f64;
    println!("Multi Thread (每個row開一個線程) : {:?} us", end);
    // println!("{:?}",&res);
}

pub fn multi_thread_3(chuck_size: &usize, m: &usize, a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) {
    let row_size: usize = m.clone();

    let block_size: usize = chuck_size.clone();
    let interval: usize = &row_size / &block_size;

    let mut jobs: Vec<JoinHandle<()>> = Vec::with_capacity(&block_size * &block_size);
    let res: Vec<Vec<f64>> = vec![vec![0.0; row_size]; row_size];
    let res: Arc<Mutex<Vec<Vec<f64>>>> = Arc::new(Mutex::new(res));
    let a: Arc<Vec<Vec<f64>>> = Arc::new(a.clone());
    let b: Arc<Vec<Vec<f64>>> = Arc::new(b.clone());

    let _start = Instant::now();
    for i in 0..&block_size * &block_size {
        let row_low_bound = i / &block_size * interval;
        let row_up_bound = row_low_bound + interval;
        let col_low_bound = i % &block_size * interval;
        let col_up_bound = col_low_bound + interval;
        // println!("row_low_bound{},row_up_bound{},col_low_bound{},col_up_bound{}",&row_low_bound,&row_up_bound,&col_low_bound,&col_up_bound );
        let data: Arc<Mutex<Vec<Vec<f64>>>> = Arc::clone(&res);
        let a_vec: Arc<Vec<Vec<f64>>> = Arc::clone(&a);
        let b_vec: Arc<Vec<Vec<f64>>> = Arc::clone(&b);

        jobs.push(thread::spawn(move || {
            let mut data: MutexGuard<Vec<Vec<f64>>> = data.lock().unwrap();
            for g in row_low_bound..row_up_bound {
                for k in col_low_bound..col_up_bound {
                    data[g][k] = cal_cell(&a_vec[g], &b_vec[k]);
                }
            }
        }));
    }

    for job in jobs{
        job.join().unwrap()
    }
    let end: f64 = _start.elapsed().as_micros() as f64;

    println!("Multi Thread (每個block開一個線程) : {:?} us", end);
    // println!("{:?}",res);
}
