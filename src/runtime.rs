use bevy::prelude::*;
use bevy::tasks::{TaskPool, TaskPoolBuilder};
use std::{future::Future, ops::DerefMut, sync::{Arc, RwLock}};

pub struct Runtime {
  pub task_pool: TaskPool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ExecState<T> {
  None,
  Executing,
  Available(T),
  Done,
}

impl <T> Default for ExecState<T> {
  fn default() -> Self { ExecState::None }
}

#[allow(dead_code)]
impl <T> ExecState<T> {
  pub fn is_executing(&self) -> bool {
    match self {
      &ExecState::Executing => true,
      _ => false
    }
  }
  pub fn is_done(&self) -> bool {
    match self {
      &ExecState::Done => true,
      _ => false
    }
  }
  pub fn is_available(&self) -> bool {
    match self {
      &ExecState::Available(_) => true,
      _ => false
    }
  }
}

#[derive(Debug, Default)]
pub struct Handle <T> {
  state: RwLock<ExecState<T>>,
}

#[allow(dead_code)]
impl <T> Handle<T> {
  pub fn take(&self) -> Option<T> {
    let mut state = self.state.write().unwrap();
    if state.is_executing() {
      return None;
    }
    let mut result: ExecState<T> = ExecState::Done;
    std::mem::swap(&mut result, state.deref_mut());
    if let ExecState::<T>::Available(item) = result {
      Some(item)
    } else {
      None
    }
 }
}

impl Runtime {
  pub fn new() -> Self {
    Self {
      task_pool: TaskPoolBuilder::new().build(),
    }
  }
  pub fn spawn<T:'static + Sync + Send>(&self, future: impl Future<Output = T> + Send + 'static) -> Arc<Handle<T>>
  {
    let handle = Arc::new(Handle{
      state: RwLock::new(ExecState::Executing),
    });
    let copied_handle = handle.clone();
    self.task_pool.spawn(async move {
      let r = future.await;
      {
        let mut state = copied_handle.state.write().unwrap();
        *state = ExecState::Available(r);
      }
    }).detach();
    handle
  }
}

impl FromWorld for Runtime {
  fn from_world(_world: &mut World) -> Self {
    Runtime::new()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test() {
    let rt = Runtime::new();
    let task = rt.spawn(async { "".to_string() });
    let result = {
      let mut r: Option<String> = None;
      while r.is_none() {
        r = task.take();
      }
      r
    };
    assert_eq!(Some("".to_string()), result);
    assert_eq!(None, task.take());
  }
}