use dioxus::prelude::*;

#[derive(Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    None,
}

#[derive(Clone)]
struct CalculatorState {
    display: String,
    current_memory: f64,
    operation: Operation,
    clear_on_next: bool,
    memory: f64,
    has_memory: bool,
}

impl Default for CalculatorState {
    fn default() -> Self {
        CalculatorState {
            display: "0".to_string(),
            current_memory: 0.0,
            operation: Operation::None,
            clear_on_next: false,
            memory: 0.0,
            has_memory: false,
        }
    }
}

fn main() {
    dioxus_desktop::launch(calc);
}

fn calc(cx: Scope) -> Element {
    let state = use_state(&cx, CalculatorState::default);

    let on_digit = move |digit: char| {
        state.set({
            let mut new_state = state.get().clone();
            if new_state.clear_on_next {
                new_state.display = digit.to_string();
                new_state.clear_on_next = false;
            } else if new_state.display == "0" {
                new_state.display = digit.to_string();
            } else {
                new_state.display.push(digit);
            }
            new_state
        });
    };

    let on_operation = move |op: Operation| {
        state.set({
            let mut new_state = state.get().clone();
            match new_state.operation {
                Operation::None => {
                    new_state.current_memory = new_state.display.parse().unwrap_or(0.0);
                }
                _ => {
                    let result = calculate(new_state.current_memory, new_state.display.parse().unwrap_or(0.0), new_state.operation.clone());
                    new_state.display = format!("{}", result);
                    new_state.current_memory = result;
                }
            }
            new_state.operation = op;
            new_state.clear_on_next = true;
            new_state
        });
    };

    let on_equals = move |_| {
        state.set({
            let mut new_state = state.get().clone();
            let result = calculate(new_state.current_memory, new_state.display.parse().unwrap_or(0.0), new_state.operation.clone());
            new_state.display = format!("{}", result);
            new_state.current_memory = result;
            new_state.operation = Operation::None;
            new_state.clear_on_next = true;
            new_state
        });
    };

    let on_clear = move |_| {
        state.set({
            let mut new_state = CalculatorState::default();
            new_state.memory = state.memory;
            new_state.has_memory = state.has_memory;
            new_state
        });
    };

    let on_memory_store = move |_| {
        state.set({
            let mut new_state = state.get().clone();
            new_state.memory = new_state.display.parse().unwrap_or(0.0);
            new_state.has_memory = true;
            new_state.clear_on_next = true;
            new_state
        });
    };

    let on_memory_recall = move |_| {
        state.set({
            let mut new_state = state.get().clone();
            if new_state.has_memory {
                new_state.display = format!("{}", new_state.memory);
                new_state.clear_on_next = true;
            }
            new_state
        });
    };

    let on_memory_clear = move |_| {
        state.set({
            let mut new_state = state.get().clone();
            new_state.memory = 0.0;
            new_state.has_memory = false;
            new_state
        });
    };

    cx.render(rsx! {
        link {
            rel: "stylesheet",
            href: "resources/style.css",
        }
        div {
            class: "calculator",
            div { 
                class: "display-container",
                div { class: "memory-indicator", 
                    display: if state.has_memory { "block" } else { "none" },
                    "M"
                }
                div { class: "display", "{state.display}" }
            }

            div { class: "button-row",
                button { onclick: on_clear, "C" }
                button { onclick: on_memory_clear, "MC" }
                button { onclick: on_memory_store, "MS" }
                button { onclick: on_memory_recall, "MR" }
            }
            div { class: "button-row",
                button { onclick: move |_| on_digit('7'), "7" }
                button { onclick: move |_| on_digit('8'), "8" }
                button { onclick: move |_| on_digit('9'), "9" }
                button { onclick: move |_| on_operation(Operation::Divide), "/" }
            }
            div { class: "button-row",
                button { onclick: move |_| on_digit('4'), "4" }
                button { onclick: move |_| on_digit('5'), "5" }
                button { onclick: move |_| on_digit('6'), "6" }
                button { onclick: move |_| on_operation(Operation::Multiply), "*" }
            }
            div { class: "button-row",
                button { onclick: move |_| on_digit('1'), "1" }
                button { onclick: move |_| on_digit('2'), "2" }
                button { onclick: move |_| on_digit('3'), "3" }
                button { onclick: move |_| on_operation(Operation::Subtract), "-" }
            }
            div { class: "button-row",
                button { onclick: move |_| on_digit('.'), "." }
                button { onclick: move |_| on_digit('0'), "0" }
                button { onclick: on_equals, "=" }
                button { onclick: move |_| on_operation(Operation::Add), "+" }
            }
        }
    })
}

fn calculate(a: f64, b: f64, operation: Operation) -> f64 {
    match operation {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
        Operation::Divide => {
            if b != 0.0 {
                a / b
            } else {
                f64::NAN
            }
        }
        Operation::None => b,
    }
}
