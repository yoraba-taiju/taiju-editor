#[derive(Debug, Default)]
pub struct KeyboardState {
  pub left: KeyState,
  pub right: KeyState,
  // keys
  pub s: KeyState,
  // Modifier
  pub shift: KeyState,
  pub ctrl: KeyState,
  // actions
  pub delete: KeyState,
}

#[derive(Debug, Default)]
pub struct KeyState {
  pub counter: u32,
}

#[allow(dead_code)]
impl KeyState {
  pub fn is_pressed(&self) -> bool {
    self.counter > 0
  }
  pub fn is_released(&self) -> bool {
    self.counter == 0
  }
  pub fn should_take_action(&self) -> bool {
    self.counter == 1 ||
    (self.counter > 30 && self.counter % 3 == 0)
  }
}
