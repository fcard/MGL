use crate::ast::*;
use crate::event::*;
use crate::error::*;
use crate::tests::utility::*;
use crate::resources::object::*;

#[test]
fn test_resources_object_fields() {
  let o = Object::new(resource(r#"
    object o {
      sprite: s
      persistent: true
    }
  "#)).unwrap();

  assert_eq!(o.sprite, Some(ResourceName::new(&["s"])));
  assert_eq!(o.persistent, true);

  let e1 = Object::new(resource("object e { k: 1\n }"));
  let e2 = Object::new(resource("object e { events: 1\n }"));
  assert_eq!(e1, MglError::invalid_field("k",  InvalidFieldKind::NotFound));
  assert_eq!(e2, MglError::invalid_field("events",  InvalidFieldKind::NotFound));
}


#[test]
fn test_resources_object_events() {
  let o = Object::new(resource(r#"
    object o {
      create:                           c
      destroy:                          d
      step["normal"]:                   sn
      step["begin"]:                    sb
      step["end"]:                      se
      alarm[0]:                         a0
      alarm[1]:                         a1
      alarm[2]:                         a2
      alarm[3]:                         a3
      alarm[4]:                         a4
      alarm[5]:                         a5
      alarm[6]:                         a6
      alarm[7]:                         a7
      alarm[8]:                         a8
      alarm[9]:                         a9
      alarm[10]:                        a10
      alarm[11]:                        a11
      keyboard["a"]:                    kca
      keyboard["b"]:                    kcb
      keyboard["no_key"]:               knk
      keyboard["any_key"]:              kak
      keyboard["left"]:                 kl
      keyboard["right"]:                kr
      keyboard["down"]:                 kd
      keyboard["up"]:                   ku
      keyboard["enter"]:                ken
      keyboard["escape"]:               kesc
      keyboard["space"]:                kspc
      keyboard["shift"]:                ksh
      keyboard["control"]:              kctrl
      keyboard["alt"]:                  kalt
      keyboard["backspace"]:            kbspc
      keyboard["tab"]:                  ktab
      keyboard["home"]:                 khom
      keyboard["end"]:                  kend
      keyboard["delete"]:               kdel
      keyboard["insert"]:               kins
      keyboard["pageup"]:               kpu
      keyboard["pagedown"]:             kpd
      keyboard["pause"]:                kpau
      keyboard["printscreen"]:          kpsc
      keyboard["f1"]:                   kf1
      keyboard["f2"]:                   kf2
      keyboard["f3"]:                   kf3
      keyboard["f4"]:                   kf4
      keyboard["f5"]:                   kf5
      keyboard["f6"]:                   kf6
      keyboard["f7"]:                   kf7
      keyboard["f8"]:                   kf8
      keyboard["f9"]:                   kf9
      keyboard["f10"]:                  kf10
      keyboard["f11"]:                  kf11
      keyboard["f12"]:                  kf12
      keyboard["numpad0"]:              knum0
      keyboard["numpad1"]:              knum1
      keyboard["numpad2"]:              knum2
      keyboard["numpad3"]:              knum3
      keyboard["numpad4"]:              knum4
      keyboard["numpad5"]:              knum5
      keyboard["numpad6"]:              knum6
      keyboard["numpad7"]:              knum7
      keyboard["numpad8"]:              knum8
      keyboard["numpad9"]:              knum9
      keyboard["multiply"]:             kmul
      keyboard["divide"]:               kdiv
      keyboard["add"]:                  kadd
      keyboard["subtract"]:             ksub
      keyboard["decimal"]:              kdec
      keyboard["lshift"]:               klsh
      keyboard["lcontrol"]:             klctrl
      keyboard["lalt"]:                 klalt
      keyboard["rshift"]:               krsh
      keyboard["rcontrol"]:             krctrl
      keyboard["ralt"]:                 kralt
      keypress["c"]:                    kpc
      keyrelease["d"]:                  krd
      mouse["no_button"]:               mnb
      mouse["left_button"]:             mlb
      mouse["right_button"]:            mrb
      mouse["middle_button"]:           mmb
      mouse["left_press"]:              mlp
      mouse["right_press"]:             mrp
      mouse["middle_press"]:            mmp
      mouse["left_release"]:            mlr
      mouse["right_release"]:           mrr
      mouse["middle_release"]:          mmr
      mouse["mouse_enter"]:             mme
      mouse["mouse_leave"]:             mml
      mouse["mouse_wheel_up"]:          mmwu
      mouse["mouse_wheel_down"]:        mmwd
      mouse["global_left_button"]:      mglb
      mouse["global_right_button"]:     mgrb
      mouse["global_middle_button"]:    mgmb
      mouse["global_left_press"]:       mglp
      mouse["global_right_press"]:      mgrp
      mouse["global_middle_press"]:     mgmp
      mouse["global_left_release"]:     mglr
      mouse["global_right_release"]:    mgrr
      mouse["global_middle_release"]:   mgmr
      collision[name]:                  coln
      collision[obj::name]:             colr
      collision[object::name2]:         colo
      other["outside"]:                 oo
      other["boundary"]:                ob
      other["game_start"]:              ogs
      other["game_end"]:                oge
      other["room_start"]:              ors
      other["room_end"]:                ore
      other["no_more_lives"]:           onml
      other["no_more_health"]:          onmh
      other["animation_end"]:           oae
      other["end_of_path"]:             oep
      other["close_button"]:            ocb
      other["user0"]:                   ou0
      other["user1"]:                   ou1
      other["user2"]:                   ou2
      other["user3"]:                   ou3
      other["user4"]:                   ou4
      other["user5"]:                   ou5
      other["user6"]:                   ou6
      other["user7"]:                   ou7
      other["user8"]:                   ou8
      other["user9"]:                   ou9
      other["user10"]:                  ou10
      other["user11"]:                  ou11
      other["user12"]:                  ou12
      other["user13"]:                  ou13
      other["user14"]:                  ou14
      other["user15"]:                  ou15
      draw["begin"]:                    db
      draw["end"]:                      de
      draw["pre"]:                      dpre
      draw["post"]:                     dpost
      draw["gui"]:                      dgui
      draw["gui_begin"]:                dguib
      draw["gui_end"]:                  dguie
    }
  "#)).unwrap();

  let mut i = 0;

  let mut assert_event = |ev, name| {
    assert_eq!(o.events[i], (ev, ResourceName::new(&["script", name])));
    i += 1;
  };

  let res = |name: &str| {
    let full_name = format!("object::{}", name);
    let names: Vec<_> = full_name.split("::").collect();
    ResourceName::new(&names)
  };

  assert_event(Event::Create,                                  "c");
  assert_event(Event::Destroy,                                 "d");
  assert_event(Event::Step(StepKind::Normal),                  "sn");
  assert_event(Event::Step(StepKind::Begin),                   "sb");
  assert_event(Event::Step(StepKind::End),                     "se");
  assert_event(Event::Alarm(Alarm::Alarm0),                    "a0");
  assert_event(Event::Alarm(Alarm::Alarm1),                    "a1");
  assert_event(Event::Alarm(Alarm::Alarm2),                    "a2");
  assert_event(Event::Alarm(Alarm::Alarm3),                    "a3");
  assert_event(Event::Alarm(Alarm::Alarm4),                    "a4");
  assert_event(Event::Alarm(Alarm::Alarm5),                    "a5");
  assert_event(Event::Alarm(Alarm::Alarm6),                    "a6");
  assert_event(Event::Alarm(Alarm::Alarm7),                    "a7");
  assert_event(Event::Alarm(Alarm::Alarm8),                    "a8");
  assert_event(Event::Alarm(Alarm::Alarm9),                    "a9");
  assert_event(Event::Alarm(Alarm::Alarm10),                   "a10");
  assert_event(Event::Alarm(Alarm::Alarm11),                   "a11");
  assert_event(Event::Keyboard(KeyCode::Character('a')),       "kca");
  assert_event(Event::Keyboard(KeyCode::Character('b')),       "kcb");
  assert_event(Event::Keyboard(KeyCode::NoKey),                "knk");
  assert_event(Event::Keyboard(KeyCode::AnyKey),               "kak");
  assert_event(Event::Keyboard(KeyCode::Left),                 "kl");
  assert_event(Event::Keyboard(KeyCode::Right),                "kr");
  assert_event(Event::Keyboard(KeyCode::Down),                 "kd");
  assert_event(Event::Keyboard(KeyCode::Up),                   "ku");
  assert_event(Event::Keyboard(KeyCode::Enter),                "ken");
  assert_event(Event::Keyboard(KeyCode::Escape),               "kesc");
  assert_event(Event::Keyboard(KeyCode::Space),                "kspc");
  assert_event(Event::Keyboard(KeyCode::Shift),                "ksh");
  assert_event(Event::Keyboard(KeyCode::Control),              "kctrl");
  assert_event(Event::Keyboard(KeyCode::Alt),                  "kalt");
  assert_event(Event::Keyboard(KeyCode::Backspace),            "kbspc");
  assert_event(Event::Keyboard(KeyCode::Tab),                  "ktab");
  assert_event(Event::Keyboard(KeyCode::Home),                 "khom");
  assert_event(Event::Keyboard(KeyCode::End),                  "kend");
  assert_event(Event::Keyboard(KeyCode::Delete),               "kdel");
  assert_event(Event::Keyboard(KeyCode::Insert),               "kins");
  assert_event(Event::Keyboard(KeyCode::PageUp),               "kpu");
  assert_event(Event::Keyboard(KeyCode::PageDown),             "kpd");
  assert_event(Event::Keyboard(KeyCode::Pause),                "kpau");
  assert_event(Event::Keyboard(KeyCode::PrintScreen),          "kpsc");
  assert_event(Event::Keyboard(KeyCode::F1),                   "kf1");
  assert_event(Event::Keyboard(KeyCode::F2),                   "kf2");
  assert_event(Event::Keyboard(KeyCode::F3),                   "kf3");
  assert_event(Event::Keyboard(KeyCode::F4),                   "kf4");
  assert_event(Event::Keyboard(KeyCode::F5),                   "kf5");
  assert_event(Event::Keyboard(KeyCode::F6),                   "kf6");
  assert_event(Event::Keyboard(KeyCode::F7),                   "kf7");
  assert_event(Event::Keyboard(KeyCode::F8),                   "kf8");
  assert_event(Event::Keyboard(KeyCode::F9),                   "kf9");
  assert_event(Event::Keyboard(KeyCode::F10),                  "kf10");
  assert_event(Event::Keyboard(KeyCode::F11),                  "kf11");
  assert_event(Event::Keyboard(KeyCode::F12),                  "kf12");
  assert_event(Event::Keyboard(KeyCode::NumPad0),              "knum0");
  assert_event(Event::Keyboard(KeyCode::NumPad1),              "knum1");
  assert_event(Event::Keyboard(KeyCode::NumPad2),              "knum2");
  assert_event(Event::Keyboard(KeyCode::NumPad3),              "knum3");
  assert_event(Event::Keyboard(KeyCode::NumPad4),              "knum4");
  assert_event(Event::Keyboard(KeyCode::NumPad5),              "knum5");
  assert_event(Event::Keyboard(KeyCode::NumPad6),              "knum6");
  assert_event(Event::Keyboard(KeyCode::NumPad7),              "knum7");
  assert_event(Event::Keyboard(KeyCode::NumPad8),              "knum8");
  assert_event(Event::Keyboard(KeyCode::NumPad9),              "knum9");
  assert_event(Event::Keyboard(KeyCode::Multiply),             "kmul");
  assert_event(Event::Keyboard(KeyCode::Divide),               "kdiv");
  assert_event(Event::Keyboard(KeyCode::Add),                  "kadd");
  assert_event(Event::Keyboard(KeyCode::Subtract),             "ksub");
  assert_event(Event::Keyboard(KeyCode::Decimal),              "kdec");
  assert_event(Event::Keyboard(KeyCode::LeftShift),            "klsh");
  assert_event(Event::Keyboard(KeyCode::LeftControl),          "klctrl");
  assert_event(Event::Keyboard(KeyCode::LeftAlt),              "klalt");
  assert_event(Event::Keyboard(KeyCode::RightShift),           "krsh");
  assert_event(Event::Keyboard(KeyCode::RightControl),         "krctrl");
  assert_event(Event::Keyboard(KeyCode::RightAlt),             "kralt");
  assert_event(Event::KeyPress(KeyCode::Character('c')),       "kpc");
  assert_event(Event::KeyRelease(KeyCode::Character('d')),     "krd");
  assert_event(Event::Mouse(MouseAction::NoButton),            "mnb");
  assert_event(Event::Mouse(MouseAction::LeftButton),          "mlb");
  assert_event(Event::Mouse(MouseAction::RightButton),         "mrb");
  assert_event(Event::Mouse(MouseAction::MiddleButton),        "mmb");
  assert_event(Event::Mouse(MouseAction::LeftPress),           "mlp");
  assert_event(Event::Mouse(MouseAction::RightPress),          "mrp");
  assert_event(Event::Mouse(MouseAction::MiddlePress),         "mmp");
  assert_event(Event::Mouse(MouseAction::LeftRelease),         "mlr");
  assert_event(Event::Mouse(MouseAction::RightRelease),        "mrr");
  assert_event(Event::Mouse(MouseAction::MiddleRelease),       "mmr");
  assert_event(Event::Mouse(MouseAction::MouseEnter),          "mme");
  assert_event(Event::Mouse(MouseAction::MouseLeave),          "mml");
  assert_event(Event::Mouse(MouseAction::MouseWheelUp),        "mmwu");
  assert_event(Event::Mouse(MouseAction::MouseWheelDown),      "mmwd");
  assert_event(Event::Mouse(MouseAction::GlobalLeftButton),    "mglb");
  assert_event(Event::Mouse(MouseAction::GlobalRightButton),   "mgrb");
  assert_event(Event::Mouse(MouseAction::GlobalMiddleButton),  "mgmb");
  assert_event(Event::Mouse(MouseAction::GlobalLeftPress),     "mglp");
  assert_event(Event::Mouse(MouseAction::GlobalRightPress),    "mgrp");
  assert_event(Event::Mouse(MouseAction::GlobalMiddlePress),   "mgmp");
  assert_event(Event::Mouse(MouseAction::GlobalLeftRelease),   "mglr");
  assert_event(Event::Mouse(MouseAction::GlobalRightRelease),  "mgrr");
  assert_event(Event::Mouse(MouseAction::GlobalMiddleRelease), "mgmr");
  assert_event(Event::Collision(res("name")),                  "coln");
  assert_event(Event::Collision(res("obj::name")),             "colr");
  assert_event(Event::Collision(res("name2")),                 "colo");
  assert_event(Event::Other(OtherEvent::RoomOutside),          "oo");
  assert_event(Event::Other(OtherEvent::RoomBoundary),         "ob");
  assert_event(Event::Other(OtherEvent::GameStart),            "ogs");
  assert_event(Event::Other(OtherEvent::GameEnd),              "oge");
  assert_event(Event::Other(OtherEvent::RoomStart),            "ors");
  assert_event(Event::Other(OtherEvent::RoomEnd),              "ore");
  assert_event(Event::Other(OtherEvent::NoMoreLives),          "onml");
  assert_event(Event::Other(OtherEvent::NoMoreHealth),         "onmh");
  assert_event(Event::Other(OtherEvent::AnimationEnd),         "oae");
  assert_event(Event::Other(OtherEvent::EndOfPath),            "oep");
  assert_event(Event::Other(OtherEvent::CloseButton),          "ocb");
  assert_event(Event::Other(OtherEvent::User0),                "ou0");
  assert_event(Event::Other(OtherEvent::User1),                "ou1");
  assert_event(Event::Other(OtherEvent::User2),                "ou2");
  assert_event(Event::Other(OtherEvent::User3),                "ou3");
  assert_event(Event::Other(OtherEvent::User4),                "ou4");
  assert_event(Event::Other(OtherEvent::User5),                "ou5");
  assert_event(Event::Other(OtherEvent::User6),                "ou6");
  assert_event(Event::Other(OtherEvent::User7),                "ou7");
  assert_event(Event::Other(OtherEvent::User8),                "ou8");
  assert_event(Event::Other(OtherEvent::User9),                "ou9");
  assert_event(Event::Other(OtherEvent::User10),               "ou10");
  assert_event(Event::Other(OtherEvent::User11),               "ou11");
  assert_event(Event::Other(OtherEvent::User12),               "ou12");
  assert_event(Event::Other(OtherEvent::User13),               "ou13");
  assert_event(Event::Other(OtherEvent::User14),               "ou14");
  assert_event(Event::Other(OtherEvent::User15),               "ou15");
  assert_event(Event::Draw(DrawKind::Begin),                   "db");
  assert_event(Event::Draw(DrawKind::End),                     "de");
  assert_event(Event::Draw(DrawKind::Pre),                     "dpre");
  assert_event(Event::Draw(DrawKind::Post),                    "dpost");
  assert_event(Event::Draw(DrawKind::Gui),                     "dgui");
  assert_event(Event::Draw(DrawKind::GuiBegin),                "dguib");
  assert_event(Event::Draw(DrawKind::GuiEnd),                  "dguie");
}

#[test]
fn test_resources_object_event_errors() {
  use EventErrorKind::*;

  let err = |key: &str| {
    let code = format!("object o {{ {}: _\n }}", key);
    Object::new(resource(&code))
  };

  let invalid_type = |ty: &str| {
    InvalidIndexType(String::from(ty))
  };

  assert_eq!(err("a.b"),                   MglError::event(Dot));
  assert_eq!(err("step[\"unknown\"]"),     MglError::event(UnknownStepKind));
  assert_eq!(err("step[0]"),               MglError::event(invalid_type("string")));
  assert_eq!(err("alarm[13]"),             MglError::event(UnknownAlarmKind));
  assert_eq!(err("alarm[a]")  ,            MglError::event(invalid_type("number")));
  assert_eq!(err("keyboard[\"unknown\"]"), MglError::event(UnknownKeyCode));
  assert_eq!(err("keyboard[0]"),           MglError::event(invalid_type("string")));
  assert_eq!(err("mouse[\"unknown\"]"),    MglError::event(UnknownMouseKind));
  assert_eq!(err("mouse[0]"),              MglError::event(invalid_type("string")));
  assert_eq!(err("collision[0]"),          MglError::event(invalid_type("resource name")));
  assert_eq!(err("other[\"unknown\"]"),    MglError::event(UnknownOtherKind));
  assert_eq!(err("other[0]"),              MglError::event(invalid_type("string")));
  assert_eq!(err("draw[\"unknown\"]"),     MglError::event(UnknownDrawKind));
  assert_eq!(err("draw[0]"),               MglError::event(invalid_type("string")));
}

