pub const DOT: char = '·';
pub const LINE_V: char = '│';
pub const LINE_H: char = '─';

#[derive(Default, Copy, Clone)]
pub struct BoxChar {
    pub value: char,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
}

pub const BOX_CHARS: [BoxChar; 16] = [
    BoxChar {
        value: DOT,
        up: false,
        right: false,
        down: false,
        left: false,
    },
    BoxChar {
        value: '╵',
        up: true,
        right: false,
        down: false,
        left: false,
    },
    BoxChar {
        value: '╶',
        up: false,
        right: true,
        down: false,
        left: false,
    },
    BoxChar {
        value: '╷',
        up: false,
        right: false,
        down: true,
        left: false,
    },
    BoxChar {
        value: '╴',
        up: false,
        right: false,
        down: false,
        left: true,
    },
    BoxChar {
        value: '└',
        up: true,
        right: true,
        down: false,
        left: false,
    },
    BoxChar {
        value: '┌',
        up: false,
        right: true,
        down: true,
        left: false,
    },
    BoxChar {
        value: '┐',
        up: false,
        right: false,
        down: true,
        left: true,
    },
    BoxChar {
        value: '┘',
        up: true,
        right: false,
        down: false,
        left: true,
    },
    BoxChar {
        value: LINE_V,
        up: true,
        right: false,
        down: true,
        left: false,
    },
    BoxChar {
        value: LINE_H,
        up: false,
        right: true,
        down: false,
        left: true,
    },
    BoxChar {
        value: '├',
        up: true,
        right: true,
        down: true,
        left: false,
    },
    BoxChar {
        value: '┬',
        up: false,
        right: true,
        down: true,
        left: true,
    },
    BoxChar {
        value: '┤',
        up: true,
        right: true,
        down: false,
        left: true,
    },
    BoxChar {
        value: '┴',
        up: true,
        right: false,
        down: true,
        left: true,
    },
    BoxChar {
        value: '┼',
        up: true,
        right: true,
        down: true,
        left: true,
    },
];

pub fn lookup(query: BoxChar) -> BoxChar {
    for target in BOX_CHARS.iter() {
        if target.up == query.up
            && target.right == query.right
            && target.down == query.down
            && target.left == query.left
        {
            return *target;
        }
    }
    query
}
// const UP_DOWN: char = '└';
// const UP_: char = '└';
// const UP_RIGHT: char = '┤';
// const UP_RIGHT: char = '└';
// const UP_RIGHT: char = '└';
// const UP_RIGHT: char = '└';
// const UP_RIGHT_DOWN_LEFT: char = '┼';
