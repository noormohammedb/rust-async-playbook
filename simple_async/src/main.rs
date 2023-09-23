use std::{
  future::{Future, Pending},
  task::Poll,
};

fn main() {
  println!("Hello, world!");

  let koo = foo1();
}

async fn foo1() -> usize {
  println!("foo1");
  0
}

fn foo2() -> impl Future<Output = usize> {
  async {
    println!("foo2");
    0
  }
}

struct bar {
  d: usize,
}
impl Future for bar {
  type Output = usize;
  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    Poll::Pending
    //
  }
}

fn foo3() -> impl Future<Output = usize> {
  let foo = bar { d: 0 };
  foo
}
