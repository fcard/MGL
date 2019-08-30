#![allow(dead_code)]
use crate::ast::ResourceName;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
  Create,
  Destroy,
  Step(StepKind),
  Alarm(Alarm),
  Keyboard(KeyCode),
  KeyPress(KeyCode),
  KeyRelease(KeyCode),
  Mouse(MouseAction),
  Collision(ResourceName),
  Other(OtherEvent),
  Draw(DrawKind)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepKind {
  Normal = 0,
  Begin,
  End
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawKind {
  Begin,
  End,
  Pre,
  Post,
  Gui,
  GuiBegin,
  GuiEnd,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alarm {
  Alarm0 = 0,
  Alarm1,
  Alarm2,
  Alarm3,
  Alarm4,
  Alarm5,
  Alarm6,
  Alarm7,
  Alarm8,
  Alarm9,
  Alarm10,
  Alarm11,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OtherEvent {
  RoomOutside,
  RoomBoundary,
  GameStart,
  GameEnd,
  RoomStart,
  RoomEnd,
  NoMoreLives,
  NoMoreHealth,
  AnimationEnd,
  EndOfPath,
  CloseButton,
  User0,
  User1,
  User2,
  User3,
  User4,
  User5,
  User6,
  User7,
  User8,
  User9,
  User10,
  User11,
  User12,
  User13,
  User14,
  User15,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseAction {
  NoButton,
  LeftButton,
  RightButton,
  MiddleButton,
  LeftPress,
  RightPress,
  MiddlePress,
  LeftRelease,
  RightRelease,
  MiddleRelease,
  MouseEnter,
  MouseLeave,
  MouseWheelUp,
  MouseWheelDown,
  GlobalLeftButton,
  GlobalRightButton,
  GlobalMiddleButton,
  GlobalLeftPress,
  GlobalRightPress,
  GlobalMiddlePress,
  GlobalLeftRelease,
  GlobalRightRelease,
  GlobalMiddleRelease,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyCode {
  NoKey,
  AnyKey,
  Character(char),
  Left,
  Right,
  Up,
  Down,
  Enter,
  Escape,
  Space,
  Shift,
  Control,
  Alt,
  Backspace,
  Tab,
  Home,
  End,
  Delete,
  Insert,
  PageUp,
  PageDown,
  Pause,
  PrintScreen,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  NumPad0,
  NumPad1,
  NumPad2,
  NumPad3,
  NumPad4,
  NumPad5,
  NumPad6,
  NumPad7,
  NumPad8,
  NumPad9,
  Multiply,
  Divide,
  Add,
  Subtract,
  Decimal,
  LeftShift,
  LeftControl,
  LeftAlt,
  RightShift,
  RightControl,
  RightAlt,
}

