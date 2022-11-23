use yew::prelude::*;
use stylist::{css};
use web_sys::{HtmlTextAreaElement};
use yew::{html, TargetCast};

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
  #[prop_or_default]
  pub on_change: Option<Callback<String>>
}


#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
  let oninput = {
    let on_change_cb = props.on_change.clone();
    Callback::from(move | event: InputEvent | {
      let cb = on_change_cb.clone();
      if cb.is_none() { return; }
      if let Some(input_element) = event.target_dyn_into::<HtmlTextAreaElement>() {
        cb.unwrap().emit(input_element.value());
      }
    })
  };

  html! {
    <textarea
      class={css!(r#"
        width: 100%;
        height: 100%;
        padding: 0;
        background: #121212;
        box-sizing: border-box;
        padding: 0.5rem;
        border: 1px solid rgba(255, 255, 255, 0.3);
        border-radius: 4px 4px 0 0;
        color: rgba(255,255,255, 0.9);
        resize: none;
        outline: none;
        transition: border 300ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;

        &:focus-visible {
          border: 1px solid rgba(255, 255, 255, 0.5);
        }
      "#)}
      { oninput }
      placeholder="Type your text here..."
    />
  }
}