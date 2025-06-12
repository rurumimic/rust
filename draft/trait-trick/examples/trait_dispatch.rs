use std::future::Future;

struct ValueDispatcher<T>(std::marker::PhantomData<T>);

trait Get<F: Future> {
    type Output;

    fn get(futures: Vec<F>) -> Self::Output;
}

impl<F> Get<F> for ValueDispatcher<()>
where
    F: Future<Output = ()>,
{
    type Output = ();

    fn get(futures: Vec<F>) -> Self::Output {
        smol::block_on(async {
            for fut in futures {
                fut.await;
            }
        })
    }
}

trait NotUnit {}

impl NotUnit for u32 {}
impl NotUnit for f32 {}
impl NotUnit for String {}
impl<T> NotUnit for Vec<T> {}

impl<T, F> Get<F> for ValueDispatcher<T>
where
    T: NotUnit,
    F: Future<Output = T>,
{
    type Output = Vec<T>;

    fn get(futures: Vec<F>) -> Self::Output {
        smol::block_on(async {
            let mut out = Vec::with_capacity(futures.len());
            for fut in futures {
                out.push(fut.await);
            }
            out
        })
    }
}

fn main() {
    let futs_void: Vec<_> = (0..3)
        .map(|i| async move {
            println!("Void future {}", i);
        })
        .collect();
    ValueDispatcher::get(futs_void);

    let futs_u32: Vec<_> = (0..3)
        .map(|i| async move {
            println!("u32 future {}", i);
            i * 10
        })
        .collect();
    let results = ValueDispatcher::get(futs_u32);
    println!("u32 futures: {:?}", results);

    let futs_f32: Vec<_> = (0..3)
        .map(|i| async move {
            println!("f32 future {}", i as f32);
            (i * 10) as f32
        })
        .collect();
    let results = ValueDispatcher::get(futs_f32);
    println!("f32 futures: {:?}", results);
}
