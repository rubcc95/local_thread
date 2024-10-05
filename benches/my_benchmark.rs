use criterion::{black_box, criterion_group, criterion_main, Criterion};
use local_thread::LocalThread;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    fn thread() -> u64 {
        black_box({
            let mut a = 0;
            let mut b = 1;
            for _ in 0..1000 {
                let temp = a;
                a = b;
                b = temp + b;
            }
            b
        })
    }

    let mut group = c.benchmark_group("Thread Speed");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("Static", |b| {
        b.iter(|| {
            std::thread::spawn(thread).join().unwrap();
        })
    });
    group.bench_function("Local", |b| {
        b.iter(|| {
            LocalThread::new(thread).spawn().join().unwrap();            
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
