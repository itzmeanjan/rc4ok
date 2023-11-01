use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
))]
use criterion_cycles_per_byte::CyclesPerByte;
use rand::{thread_rng, RngCore};

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
))]
type CriterionCPB = Criterion<CyclesPerByte>;

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
)))]
type CriterionCPB = Criterion;

fn rc4ok(c: &mut CriterionCPB) {
    let mut rng = thread_rng();

    const MIN_KEY_SIZE: usize = 8; // bytes
    const MAX_KEY_SIZE: usize = 512; // bytes
    const MIN_OUT_SIZE: usize = 32; // bytes
    const MAX_OUT_SIZE: usize = 8192; // bytes

    let mut klen = MIN_KEY_SIZE;
    while klen <= MAX_KEY_SIZE {
        let mut olen = MIN_OUT_SIZE;
        while olen <= MAX_OUT_SIZE {
            let mut group = c.benchmark_group(format!("rc4ok/{}B key", klen));
            group.throughput(Throughput::Bytes((klen + olen) as u64));

            group.bench_function(format!("{}B out (cached)", olen), |bench| {
                let mut key = vec![0u8; klen];
                let mut out = vec![0u8; olen];

                rng.fill_bytes(&mut key);

                bench.iter(|| {
                    let mut rc4ok_prng = rc4ok::RC4ok::init(black_box(&key));
                    rc4ok_prng.generate(black_box(&mut out));
                })
            });

            group.bench_function(format!("{}B out (random)", olen), |bench| {
                let mut key = vec![0u8; klen];
                let out = vec![0u8; olen];

                rng.fill_bytes(&mut key);

                bench.iter_batched(
                    || (key.clone(), out.clone()),
                    |(key, mut out)| {
                        let mut rc4ok_prng = rc4ok::RC4ok::init(black_box(&key));
                        rc4ok_prng.generate(black_box(&mut out));
                    },
                    BatchSize::SmallInput,
                )
            });

            group.finish();
            olen *= 4;
        }

        klen *= 4;
    }
}

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
))]
criterion_group!(name = rc4ok_prng; config = Criterion::default().with_measurement(CyclesPerByte); targets = rc4ok);

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "loongarch64"
)))]
criterion_group!(rc4ok_prng, rc4ok);

criterion_main!(rc4ok_prng);
