use crate::event::enums::*;
use crate::ast::*;
use crate::error::EventErrorKind;
use std::convert::TryFrom;

use EventErrorKind::*;

type Result<T> = std::result::Result<T, EventErrorKind>;

impl TryFrom<Key> for Event {
  type Error = EventErrorKind;

  fn try_from(value: Key) -> Result<Self> {
    if value.is_dot() {
      return Err(Dot);
    }

    match &*value.name_of() {
      "create"     => Ok(Event::Create),
      "destroy"    => Ok(Event::Destroy),
      "step"       => Ok(Event::Step(parse_step_kind(value)?)),
      "alarm"      => Ok(Event::Alarm(parse_alarm_kind(value)?)),
      "keyboard"   => Ok(Event::Keyboard(parse_key_code(value)?)),
      "keypress"   => Ok(Event::KeyPress(parse_key_code(value)?)),
      "keyrelease" => Ok(Event::KeyRelease(parse_key_code(value)?)),
      "mouse"      => Ok(Event::Mouse(parse_mouse_action(value)?)),
      "collision"  => Ok(Event::Collision(parse_object_name(value)?)),
      "other"      => Ok(Event::Other(parse_other_event(value)?)),
      "draw"       => Ok(Event::Draw(parse_draw_kind(value)?)),
      _ => Err(InvalidName)
    }
  }
}

fn parse_step_kind(value: Key) -> Result<StepKind> {
  match value.index_of().unwrap() {
    Expression::Str(name) => {
      match &*name {
        "normal" => Ok(StepKind::Normal),
        "begin"  => Ok(StepKind::Begin),
        "end"    => Ok(StepKind::End),
        _ => Err(UnknownStepKind)
      }
    }
    _ => Err(InvalidIndexType(String::from("string")))
  }
}

fn parse_alarm_kind(value: Key) -> Result<Alarm> {
  match value.index_of().unwrap() {
    Expression::Num(n) => {
      match &*n {
         "0" => Ok(Alarm::Alarm0),
         "1" => Ok(Alarm::Alarm1),
         "2" => Ok(Alarm::Alarm2),
         "3" => Ok(Alarm::Alarm3),
         "4" => Ok(Alarm::Alarm4),
         "5" => Ok(Alarm::Alarm5),
         "6" => Ok(Alarm::Alarm6),
         "7" => Ok(Alarm::Alarm7),
         "8" => Ok(Alarm::Alarm8),
         "9" => Ok(Alarm::Alarm9),
        "10" => Ok(Alarm::Alarm10),
        "11" => Ok(Alarm::Alarm11),
        _ => Err(UnknownAlarmKind)
      }
    }
    _ => Err(InvalidIndexType(String::from("number")))
  }
}

fn parse_key_code(value: Key) -> Result<KeyCode> {
  match value.index_of().unwrap() {
    Expression::Str(name) => {
      if name.len() == 1 && name.chars().all(char::is_alphabetic) {
        Ok(KeyCode::Character(name.chars().next().unwrap()))

      } else {
        match &*name {
          "no_key"       => Ok(KeyCode::NoKey),
          "any_key"      => Ok(KeyCode::AnyKey),
          "left"         => Ok(KeyCode::Left),
          "right"        => Ok(KeyCode::Right),
          "down"         => Ok(KeyCode::Down),
          "up"           => Ok(KeyCode::Up),
          "enter"        => Ok(KeyCode::Enter),
          "escape"       => Ok(KeyCode::Escape),
          "space"        => Ok(KeyCode::Space),
          "shift"        => Ok(KeyCode::Shift),
          "control"      => Ok(KeyCode::Control),
          "alt"          => Ok(KeyCode::Alt),
          "backspace"    => Ok(KeyCode::Backspace),
          "tab"          => Ok(KeyCode::Tab),
          "home"         => Ok(KeyCode::Home),
          "end"          => Ok(KeyCode::End),
          "delete"       => Ok(KeyCode::Delete),
          "insert"       => Ok(KeyCode::Insert),
          "pageup"       => Ok(KeyCode::PageUp),
          "pagedown"     => Ok(KeyCode::PageDown),
          "pause"        => Ok(KeyCode::Pause),
          "printscreen"  => Ok(KeyCode::PrintScreen),
          "f1"           => Ok(KeyCode::F1),
          "f2"           => Ok(KeyCode::F2),
          "f3"           => Ok(KeyCode::F3),
          "f4"           => Ok(KeyCode::F4),
          "f5"           => Ok(KeyCode::F5),
          "f6"           => Ok(KeyCode::F6),
          "f7"           => Ok(KeyCode::F7),
          "f8"           => Ok(KeyCode::F8),
          "f9"           => Ok(KeyCode::F9),
          "f10"          => Ok(KeyCode::F10),
          "f11"          => Ok(KeyCode::F11),
          "f12"          => Ok(KeyCode::F12),
          "numpad0"      => Ok(KeyCode::NumPad0),
          "numpad1"      => Ok(KeyCode::NumPad1),
          "numpad2"      => Ok(KeyCode::NumPad2),
          "numpad3"      => Ok(KeyCode::NumPad3),
          "numpad4"      => Ok(KeyCode::NumPad4),
          "numpad5"      => Ok(KeyCode::NumPad5),
          "numpad6"      => Ok(KeyCode::NumPad6),
          "numpad7"      => Ok(KeyCode::NumPad7),
          "numpad8"      => Ok(KeyCode::NumPad8),
          "numpad9"      => Ok(KeyCode::NumPad9),
          "multiply"     => Ok(KeyCode::Multiply),
          "divide"       => Ok(KeyCode::Divide),
          "add"          => Ok(KeyCode::Add),
          "subtract"     => Ok(KeyCode::Subtract),
          "decimal"      => Ok(KeyCode::Decimal),
          "lshift"       => Ok(KeyCode::LeftShift),
          "lcontrol"     => Ok(KeyCode::LeftControl),
          "lalt"         => Ok(KeyCode::LeftAlt),
          "rshift"       => Ok(KeyCode::RightShift),
          "rcontrol"     => Ok(KeyCode::RightControl),
          "ralt"         => Ok(KeyCode::RightAlt),
          _ => Err(UnknownKeyCode)
        }
      }
    }
    _ => Err(InvalidIndexType(String::from("string")))
  }
}

fn parse_mouse_action(value: Key) -> Result<MouseAction> {
  match value.index_of().unwrap() {
    Expression::Str(name) => {
      match &*name {
        "no_button"             => Ok(MouseAction::NoButton),
        "left_button"           => Ok(MouseAction::LeftButton),
        "right_button"          => Ok(MouseAction::RightButton),
        "middle_button"         => Ok(MouseAction::MiddleButton),
        "left_press"            => Ok(MouseAction::LeftPress),
        "right_press"           => Ok(MouseAction::RightPress),
        "middle_press"          => Ok(MouseAction::MiddlePress),
        "left_release"          => Ok(MouseAction::LeftRelease),
        "right_release"         => Ok(MouseAction::RightRelease),
        "middle_release"        => Ok(MouseAction::MiddleRelease),
        "mouse_enter"           => Ok(MouseAction::MouseEnter),
        "mouse_leave"           => Ok(MouseAction::MouseLeave),
        "mouse_wheel_up"        => Ok(MouseAction::MouseWheelUp),
        "mouse_wheel_down"      => Ok(MouseAction::MouseWheelDown),
        "global_left_button"    => Ok(MouseAction::GlobalLeftButton),
        "global_right_button"   => Ok(MouseAction::GlobalRightButton),
        "global_middle_button"  => Ok(MouseAction::GlobalMiddleButton),
        "global_left_press"     => Ok(MouseAction::GlobalLeftPress),
        "global_right_press"    => Ok(MouseAction::GlobalRightPress),
        "global_middle_press"   => Ok(MouseAction::GlobalMiddlePress),
        "global_left_release"   => Ok(MouseAction::GlobalLeftRelease),
        "global_right_release"  => Ok(MouseAction::GlobalRightRelease),
        "global_middle_release" => Ok(MouseAction::GlobalMiddleRelease),
        _ => Err(UnknownMouseKind)
      }
    }
    _ => Err(InvalidIndexType(String::from("string")))
  }
}

fn parse_object_name(value: Key) -> Result<ResourceName> {
  match value.index_of().unwrap() {
    Expression::Name(name) => {
      Ok(ResourceName::new(&["object", &name]))
    }

    Expression::Resource(name) => {
      if name.top_module_is("object") {
        Ok(name)
      } else {
        Ok(ResourceName::InModule(String::from("object"), box name.clone()))
      }
    }

    _ => Err(InvalidIndexType(String::from("resource name")))
  }
}

fn parse_other_event(value: Key) -> Result<OtherEvent> {
  match value.index_of().unwrap() {
    Expression::Str(name) => {
      match &*name {
        "outside"        => Ok(OtherEvent::RoomOutside),
        "boundary"       => Ok(OtherEvent::RoomBoundary),
        "game_start"     => Ok(OtherEvent::GameStart),
        "game_end"       => Ok(OtherEvent::GameEnd),
        "room_start"     => Ok(OtherEvent::RoomStart),
        "room_end"       => Ok(OtherEvent::RoomEnd),
        "no_more_lives"  => Ok(OtherEvent::NoMoreLives),
        "no_more_health" => Ok(OtherEvent::NoMoreHealth),
        "animation_end"  => Ok(OtherEvent::AnimationEnd),
        "end_of_path"    => Ok(OtherEvent::EndOfPath),
        "close_button"   => Ok(OtherEvent::CloseButton),
        "user0"          => Ok(OtherEvent::User0),
        "user1"          => Ok(OtherEvent::User1),
        "user2"          => Ok(OtherEvent::User2),
        "user3"          => Ok(OtherEvent::User3),
        "user4"          => Ok(OtherEvent::User4),
        "user5"          => Ok(OtherEvent::User5),
        "user6"          => Ok(OtherEvent::User6),
        "user7"          => Ok(OtherEvent::User7),
        "user8"          => Ok(OtherEvent::User8),
        "user9"          => Ok(OtherEvent::User9),
        "user10"         => Ok(OtherEvent::User10),
        "user11"         => Ok(OtherEvent::User11),
        "user12"         => Ok(OtherEvent::User12),
        "user13"         => Ok(OtherEvent::User13),
        "user14"         => Ok(OtherEvent::User14),
        "user15"         => Ok(OtherEvent::User15),
        _ => Err(UnknownOtherKind)
      }
    }
    _ => Err(InvalidIndexType(String::from("string")))
  }
}

fn parse_draw_kind(value: Key) -> Result<DrawKind> {
  match value.index_of().unwrap() {
    Expression::Str(name) => {
      match &*name {
        "begin"     => Ok(DrawKind::Begin),
        "end"       => Ok(DrawKind::End),
        "pre"       => Ok(DrawKind::Pre),
        "post"      => Ok(DrawKind::Post),
        "gui"       => Ok(DrawKind::Gui),
        "gui_begin" => Ok(DrawKind::GuiBegin),
        "gui_end"   => Ok(DrawKind::GuiEnd),
        _ => Err(UnknownDrawKind)
      }
    }
    _ => Err(InvalidIndexType(String::from("string")))
  }
}

