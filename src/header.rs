use yew::prelude::*;
use stylist::css;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub on_info_toggle: Option<Callback<()>>
}


#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {

    let handle_ferris_click = {
        let toggle_cb = props.on_info_toggle.clone();
        Callback::from(move | _: MouseEvent | {
            let toggle_cb = toggle_cb.clone();
            if let Some(cb) = toggle_cb {
                cb.emit(());
            }
        })
    };

    html! { 
        <div class={css!("
            background: #1E1E1E;
            padding: 1rem;
            display: flex;
            justify-content: space-between;
            box-shadow: 4px 4px 16px 8px rgba(0, 0, 0, 0.2);
        ")}>
            // Title
            <div class={css!("
                font-family: 'Epilogue', sans-serif;
                font-size: 2rem;
                line-height: 48px;
                font-weight: 700;
                user-select: none;
            ")}>
                {"CharacterCounter.rs"}
            </div>
            <div class={css!("
                position: fixed;
                right: 0.5rem;

                width: 3rem;
                height: 3rem;
                background: #1E1E1E;
                border-radius: 50%;
                display: flex;

                box-shadow: 4px 4px 16px 8px rgba(0, 0, 0, 0.3);
                transition: box-shadow 300ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
                cursor: pointer;

                & > img {
                    width: 80%;
                    height: 80%;
                    margin: auto;
                }

                &:hover {
                    box-shadow: 4px 4px 28px 8px rgba(0, 0, 0, 0.4);
                }
            ")}
                onclick={handle_ferris_click}
            >
                <img src="ferris.svg" alt="About this project" />
            </div>
        </div>
    }
}