pub const ADDRESS_COLUMN_WIDTH: i32 = 6;
pub const CHARACTERS_PER_COLUMN: i32 = ROWS * WORD_COLUMN_WIDTH;
pub const COLUMNS: i32 = 2;
pub const COLUMN_END_ROW: i32 = COLUMN_START_ROW + ROWS;
pub const COLUMN_PADDING: i32 = 2;
pub const COLUMN_START_ROW: i32 = MARGIN + 5;
pub const COLUMN_WIDTH: i32 = ADDRESS_COLUMN_WIDTH + INNER_COLUMN_PADDING + WORD_COLUMN_WIDTH;
pub const CONSOLE_PADDING: i32 = 1;
pub const CONSOLE_WIDTH: i32 = 13;
pub const INNER_COLUMN_PADDING: i32 = 1;
pub const MARGIN: i32 = 1;
pub const ROWS: i32 = 17;
pub const STARTING_ATTEMPTS: i32 = 4;
pub const TERMINAL_WIDTH: i32 = COLUMN_WIDTH + COLUMN_PADDING + COLUMN_WIDTH + CONSOLE_PADDING + CONSOLE_WIDTH;
pub const WINDOW_WIDTH: i32 = MARGIN + TERMINAL_WIDTH + MARGIN;
pub const WORD_COLUMN_WIDTH: i32 = 12;