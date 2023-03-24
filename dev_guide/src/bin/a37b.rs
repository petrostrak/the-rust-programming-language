enum KeyPress {
    Down,
    Up,
}

struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}

impl From<KeyEvent> for InputEvent {
    fn from(value: KeyEvent) -> Self {
        InputEvent::Key(value.keycode, value.state)
    }
}

enum InputEvent {
    Key(u16, KeyPress),
    Mouse,
}

fn main() {
    let key_ev = KeyEvent {
        keycode: 5,
        state: KeyPress::Down,
    };

    // let input_event = InputEvent::from(key_ev);
    let input_event: InputEvent = key_ev.into();
}
