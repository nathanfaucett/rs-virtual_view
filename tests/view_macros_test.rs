#[macro_use]
extern crate virtual_view;

use virtual_view::Event;

#[test]
fn test_view() {
    let on_click = |e: &mut Event| {
        let _ = e;
    };
    let value = 0;

    let _ = view! {
        <div
            class="test"
            index=1
            style={{"color": "#fff"}}
            index={[0, 1, 2]}
            onclick=on_click
            ondblclick={move |e: &mut Event| {
                let _ = e;
                let _ = value;
            }}
            ... { props! { "data-id": 1 } }
        >
            <p>{"Hello, world!"}</p>
        </div>
    };
}
