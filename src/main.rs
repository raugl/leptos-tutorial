use leptos::{ev::SubmitEvent, html::Input, *};

#[component]
fn App() -> impl IntoView {
    view! {
        <h2>"Controlled Component"</h2>
        <ControlledComponent/>
        <h2>"Uncontrolled Component"</h2>
        <UncontrolledComponent/>
    }
}

#[component]
fn ControlledComponent() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
            on:input=move |event| {
                set_name(event_target_value(&event));
            }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn UncontrolledComponent() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<Input> = create_node_ref();

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        let value = input_element().expect("Expected <input> to exist!").value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
