use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    tasks: Vec<String>, // Holds the list of tasks
    input: String,      // Holds the current value of the input field
}

enum Msg {
    Add,                 // Adds a new task
    UpdateInput(String), // Updates the input field
    Remove(usize),       // Removes a task
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            tasks: Vec::new(),
            input: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                if !self.input.is_empty() {
                    self.tasks.push(self.input.clone());
                    self.input.clear();
                }
            }
            Msg::UpdateInput(value) => {
                self.input = value;
            }
            Msg::Remove(index) => {
                self.tasks.remove(index);
            }
        }
        true // Indicate that the Component should re-render
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "Todo List" }</h1>
                <ul>
                    { for self.tasks.iter().enumerate().map(|(i, task)| {
                        html!{ <li>{ task } <button onclick=self.link.callback(move |_| Msg::Remove(i))>{ "Remove" }</button></li> }
                    }) }
                </ul>
                <input
                    value=self.input.clone()
                    oninput=self.link.callback(|e: InputData| Msg::UpdateInput(e.value))
                    onkeypress=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::Add } else { Msg::UpdateInput(String::new()) }
                    })

                />
                <button onclick=self.link.callback(|_| Msg::Add)>{ "Add Task" }</button>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
