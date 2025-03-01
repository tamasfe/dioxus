use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};
use dioxus_core::VNode;
use dioxus_core::*;
use dioxus_core_macro::*;
use dioxus_hooks::*;
use dioxus_html as dioxus_elements;
use dioxus_tui::TuiContext;

#[test]
fn key_down() {
    dioxus_tui::launch_cfg(app, dioxus_tui::Config::new().with_headless());

    fn app(cx: Scope) -> Element {
        let render_count = use_state(&cx, || 0);
        let tui_ctx: TuiContext = cx.consume_context().unwrap();
        let render_count_handle = render_count.clone();
        cx.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            render_count_handle.modify(|x| *x + 1);
        });
        if *render_count.get() > 2 {
            panic!("Event was not received");
        }
        tui_ctx.inject_event(Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
        }));
        cx.render(rsx! {
            div {
                width: "100%",
                height: "100%",
                onkeydown: move |evt| {
                    assert_eq!(evt.data.key_code, dioxus_html::KeyCode::A);
                    tui_ctx.quit();
                },
            }
        })
    }
}

#[test]
fn mouse_down() {
    dioxus_tui::launch_cfg(app, dioxus_tui::Config::new().with_headless());

    fn app(cx: Scope) -> Element {
        let render_count = use_state(&cx, || 0);
        let tui_ctx: TuiContext = cx.consume_context().unwrap();
        let render_count_handle = render_count.clone();
        cx.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            render_count_handle.modify(|x| *x + 1);
        });
        if *render_count.get() > 2 {
            panic!("Event was not received");
        }
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 0,
            row: 0,
            kind: crossterm::event::MouseEventKind::Down(MouseButton::Left),
            modifiers: KeyModifiers::NONE,
        }));
        cx.render(rsx! {
            div {
                width: "100%",
                height: "100%",
                onmousedown: move |evt| {
                    assert!(evt.data.held_buttons().contains(dioxus_html::input_data::MouseButton::Primary));
                    tui_ctx.quit();
                },
            }
        })
    }
}

#[test]
fn mouse_up() {
    dioxus_tui::launch_cfg(app, dioxus_tui::Config::new().with_headless());

    fn app(cx: Scope) -> Element {
        let render_count = use_state(&cx, || 0);
        let tui_ctx: TuiContext = cx.consume_context().unwrap();
        let render_count_handle = render_count.clone();
        cx.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            render_count_handle.modify(|x| *x + 1);
        });
        if *render_count.get() > 2 {
            panic!("Event was not received");
        }
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 0,
            row: 0,
            kind: crossterm::event::MouseEventKind::Down(MouseButton::Left),
            modifiers: KeyModifiers::NONE,
        }));
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 0,
            row: 0,
            kind: crossterm::event::MouseEventKind::Up(MouseButton::Left),
            modifiers: KeyModifiers::NONE,
        }));
        cx.render(rsx! {
            div {
                width: "100%",
                height: "100%",
                onmouseup: move |_| {
                    tui_ctx.quit();
                },
            }
        })
    }
}

#[test]
fn mouse_enter() {
    dioxus_tui::launch_cfg(app, dioxus_tui::Config::new().with_headless());

    fn app(cx: Scope) -> Element {
        let render_count = use_state(&cx, || 0);
        let tui_ctx: TuiContext = cx.consume_context().unwrap();
        let render_count_handle = render_count.clone();
        cx.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            render_count_handle.modify(|x| *x + 1);
        });
        if *render_count.get() > 2 {
            panic!("Event was not received");
        }
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 100,
            row: 100,
            kind: crossterm::event::MouseEventKind::Moved,
            modifiers: KeyModifiers::NONE,
        }));
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 0,
            row: 0,
            kind: crossterm::event::MouseEventKind::Moved,
            modifiers: KeyModifiers::NONE,
        }));
        cx.render(rsx! {
            div {
                width: "50%",
                height: "50%",
                onmouseenter: move |_| {
                    tui_ctx.quit();
                },
            }
        })
    }
}

#[test]
fn mouse_exit() {
    dioxus_tui::launch_cfg(app, dioxus_tui::Config::new().with_headless());

    fn app(cx: Scope) -> Element {
        let render_count = use_state(&cx, || 0);
        let tui_ctx: TuiContext = cx.consume_context().unwrap();
        let render_count_handle = render_count.clone();
        cx.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            render_count_handle.modify(|x| *x + 1);
        });
        if *render_count.get() > 2 {
            panic!("Event was not received");
        }
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 0,
            row: 0,
            kind: crossterm::event::MouseEventKind::Moved,
            modifiers: KeyModifiers::NONE,
        }));
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 100,
            row: 100,
            kind: crossterm::event::MouseEventKind::Moved,
            modifiers: KeyModifiers::NONE,
        }));
        cx.render(rsx! {
            div {
                width: "50%",
                height: "50%",
                onmouseenter: move |_| {
                    tui_ctx.quit();
                },
            }
        })
    }
}

#[test]
fn mouse_move() {
    dioxus_tui::launch_cfg(app, dioxus_tui::Config::new().with_headless());

    fn app(cx: Scope) -> Element {
        let render_count = use_state(&cx, || 0);
        let tui_ctx: TuiContext = cx.consume_context().unwrap();
        let render_count_handle = render_count.clone();
        cx.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            render_count_handle.modify(|x| *x + 1);
        });
        if *render_count.get() > 2 {
            panic!("Event was not received");
        }
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 40,
            row: 40,
            kind: crossterm::event::MouseEventKind::Moved,
            modifiers: KeyModifiers::NONE,
        }));
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 60,
            row: 60,
            kind: crossterm::event::MouseEventKind::Moved,
            modifiers: KeyModifiers::NONE,
        }));
        cx.render(rsx! {
            div {
                width: "100%",
                height: "100%",
                onmousemove: move |_|{
                    tui_ctx.quit();
                },
            }
        })
    }
}

#[test]
fn wheel() {
    dioxus_tui::launch_cfg(app, dioxus_tui::Config::new().with_headless());

    fn app(cx: Scope) -> Element {
        let render_count = use_state(&cx, || 0);
        let tui_ctx: TuiContext = cx.consume_context().unwrap();
        let render_count_handle = render_count.clone();
        cx.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            render_count_handle.modify(|x| *x + 1);
        });
        if *render_count.get() > 2 {
            panic!("Event was not received");
        }
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 50,
            row: 50,
            kind: crossterm::event::MouseEventKind::Moved,
            modifiers: KeyModifiers::NONE,
        }));
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 50,
            row: 50,
            kind: crossterm::event::MouseEventKind::ScrollDown,
            modifiers: KeyModifiers::NONE,
        }));
        cx.render(rsx! {
            div {
                width: "100%",
                height: "100%",
                onwheel: move |evt| {
                    assert!(evt.data.delta_y > 0.0);
                    tui_ctx.quit();
                },
            }
        })
    }
}

#[test]
fn click() {
    dioxus_tui::launch_cfg(app, dioxus_tui::Config::new().with_headless());

    fn app(cx: Scope) -> Element {
        let render_count = use_state(&cx, || 0);
        let tui_ctx: TuiContext = cx.consume_context().unwrap();
        let render_count_handle = render_count.clone();
        cx.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            render_count_handle.modify(|x| *x + 1);
        });
        if *render_count.get() > 2 {
            panic!("Event was not received");
        }
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 50,
            row: 50,
            kind: crossterm::event::MouseEventKind::Down(MouseButton::Left),
            modifiers: KeyModifiers::NONE,
        }));
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 50,
            row: 50,
            kind: crossterm::event::MouseEventKind::Up(MouseButton::Left),
            modifiers: KeyModifiers::NONE,
        }));
        cx.render(rsx! {
            div {
                width: "100%",
                height: "100%",
                onclick: move |_|{
                    tui_ctx.quit();
                },
            }
        })
    }
}

#[test]
fn context_menu() {
    dioxus_tui::launch_cfg(app, dioxus_tui::Config::new().with_headless());

    fn app(cx: Scope) -> Element {
        let render_count = use_state(&cx, || 0);
        let tui_ctx: TuiContext = cx.consume_context().unwrap();
        let render_count_handle = render_count.clone();
        cx.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            render_count_handle.modify(|x| *x + 1);
        });
        if *render_count.get() > 2 {
            panic!("Event was not received");
        }
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 50,
            row: 50,
            kind: crossterm::event::MouseEventKind::Down(MouseButton::Right),
            modifiers: KeyModifiers::NONE,
        }));
        tui_ctx.inject_event(Event::Mouse(MouseEvent {
            column: 50,
            row: 50,
            kind: crossterm::event::MouseEventKind::Up(MouseButton::Right),
            modifiers: KeyModifiers::NONE,
        }));
        cx.render(rsx! {
            div {
                width: "100%",
                height: "100%",
                oncontextmenu: move |_|{
                    tui_ctx.quit();
                },
            }
        })
    }
}
