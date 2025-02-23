#[macro_export]
macro_rules! ERROR_LINE_PREFIX {
    () => {
        "error: "
    };
}
pub use ERROR_LINE_PREFIX;

#[macro_export]
macro_rules! QUESTION_LINE_PREFIX {
    () => {
        "? "
    };
}
pub use QUESTION_LINE_PREFIX;

#[macro_export]
macro_rules! CHOSEN_CHOICE_LINE_PREFIX {
    () => {
        "> "
    };
}
pub use CHOSEN_CHOICE_LINE_PREFIX;

#[macro_export]
macro_rules! UNCHOSEN_CHOICE_LINE_PREFIX {
    () => {
        "  "
    };
}
pub use UNCHOSEN_CHOICE_LINE_PREFIX;

#[macro_export]
macro_rules! HELP_LINE_PREFIX {
    () => {
        "["
    };
}
pub use HELP_LINE_PREFIX;
#[macro_export]
macro_rules! HELP_LINE_POSTFIX {
    () => {
        "]"
    };
}
pub use HELP_LINE_POSTFIX;

// HELP
#[macro_export]
macro_rules! CONTROL_FLOW_HELP {
    () => {
        "Esc back, ⇑Esc quit"
    };
}
pub use CONTROL_FLOW_HELP;

#[macro_export]
macro_rules! SELECT_MOVE_HELP {
    () => {
        "↑↓ move"
    };
}
pub use SELECT_MOVE_HELP;

#[macro_export]
macro_rules! CONTINUE_HELP {
    () => {
        "↵ continue"
    };
}
pub use CONTINUE_HELP;

#[macro_export]
macro_rules! SELECT_HELP {
    () => {
        concat!(
            $crate::SELECT_MOVE_HELP!(),
            ", ",
            $crate::CONTINUE_HELP!(),
            ", ",
            $crate::CONTROL_FLOW_HELP!()
        )
    };
}
pub use SELECT_HELP;

#[macro_export]
macro_rules! MAIN_MENU_SELECT_HELP {
    () => {
        concat!(
            $crate::SELECT_MOVE_HELP!(),
            ", ",
            $crate::CONTINUE_HELP!(),
            ", Esc quit"
        )
    };
}
pub use MAIN_MENU_SELECT_HELP;

#[macro_export]
macro_rules! UNSIGNED_INT_HELP {
    () => {
        concat!(
            "0-9, ",
            $crate::CONTINUE_HELP!(),
            ", ",
            $crate::CONTROL_FLOW_HELP!()
        )
    };
}
pub use UNSIGNED_INT_HELP;

#[macro_export]
macro_rules! INFO_HELP {
    () => {
        concat!("any button to continue, ", $crate::CONTROL_FLOW_HELP!())
    };
}
pub use INFO_HELP;
