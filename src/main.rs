use yew::prelude::*;
use stylist::css;


mod header;
use header::Header;

mod textinput;
use textinput::TextInput;

#[derive(Properties, PartialEq)]
struct StatisticsViewProps {
    #[prop_or_default]
    pub name: String,
    pub value: i32
}

#[function_component(StatisticsView)]
fn statistics_view(props: &StatisticsViewProps) -> Html {
    html! {
        <div class={css!("
            border: 1px solid rgba(255, 255, 255, 0.3);
            border-radius: 0 0 4px 4px;
            border-top: none;
            padding: 0.5rem 1rem;
            display: flex;
            font-size: 1rem;
            line-height: 1.5rem;

            transition: padding 300ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
            @media only screen and (max-width: 450px) {
                padding: 0.5rem;
            }
        ")}>
            <p class={css!("
                font-family: 'Epilogue', sans-serif;
                font-weight: 800;
                margin: auto 1rem auto 0;
                text-transform: uppercase;
            ")}>{&props.name}</p>
            <p class={css!("
                font-family: 'Epilogue', sans-serif;
                margin: auto 0 auto auto;
                font-weight: 800;
                color: #00acc1;
            ")}>{&props.value}</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ContentWrapperProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ContentWrapper)]
fn content_wrapper(props: &ContentWrapperProps) -> Html {
    html!(
        <div class={css!("
            position: relative;
            padding: 2rem 1rem;
            display: flex;
            flex-direction: column;
            height: 100%;
        ")}>
            { for props.children.iter() }
        </div>
    )
}


#[derive(Properties, PartialEq)]
struct InfoOverlayProps {
    #[prop_or_default]
    pub show: bool,
}


#[function_component(InfoOverlay)]
fn info_overlay(props: &InfoOverlayProps) -> Html {
    if props.show {
    html!(
        <div class={css!("
            position: absolute;
            width: calc(100% - 2rem);
            height: calc(100% - 4rem);
            @keyframes appear {
                from {
                    background: transparent;
                }
                to {
                    background: #121212;
                }
            }
            animation-name: appear;
            animation-duration: 300ms;
            animation-fill-mode: forwards;
            
            a {
                color: #00acc1;
                text-decoration: none;
                font-weight: 600;
            }
            display: flex;
        ")}>
            <div class={css!("margin: auto; max-width: 600px; width: 100%;")}>
                <span class={css!("font-family: 'Epilogue', sans-serif; font-weight: 600; font-size: 1.75rem;")}>
                    {"Hello Yew world!"}
                </span>
                <br />
                <span class={css!("font-size: 1rem; margin: 1rem 0 0 0;")}>
                    {"This is a small sample project to get my bearings with "}
                    <a href="https://yew.rs/">{"yew"}</a>
                    {" and in general with web-development in Rust."}
                    <br />
                    {"Usage should be fairly self-explanatory - but if you're stuck, try putting some text into the big text-field ;)"}
                    <br/>
                    {"Enjoy and take a look at the sources on "}
                    <a href="https://github.com/meinlebenswerk/CharacterCounter.rs">{"github"}</a>
                    {" if you want."}
                </span>
            </div>
        </div>
    ) } else { html!() }
}



#[function_component(App)]
fn app() -> Html {
    let n_chars_without_spaces = use_state(|| 0);
    let n_chars_with_spaces = use_state(|| 0);
    let n_words = use_state(|| 0);

    let show_info = use_state(|| false);

    let input_change_cb = {
        let n_chars_without_spaces = n_chars_without_spaces.clone();
        let n_chars_with_spaces = n_chars_with_spaces.clone();
        let n_words = n_words.clone();

        Callback::from(move | text: String | {
            let mut char_count = 0;
            let mut word_count = 0;
            let mut char_count_wo_spaces = 0;

            let mut processing_word = false;
            for char in text.chars() {
                match char {
                    ' ' => {
                        char_count += 1;
                        if processing_word { 
                            word_count += 1;
                            processing_word = false;
                        }
                    },
                    _ => {
                        char_count += 1;
                        char_count_wo_spaces += 1;
                        processing_word = true;
                    }
                }
            }
            if processing_word {
                word_count += 1;
            }

            n_chars_without_spaces.set(char_count_wo_spaces);
            n_chars_with_spaces.set(char_count);
            n_words.set(word_count);
        })
    };

    let handle_info_toggle = {
        let show_info = show_info.clone();
        Callback::from(move | _: () | {
            let show_info = show_info.clone();
            show_info.set(!*show_info);
        })
    };

    html! {
        <div class={css!("
            width: 100vw;
            height: 100vh;
            background: #121212;
            display: flex;
            flex-direction: column;
            font-family: 'Lato', sans-serif;
            color: rgba(255,255,255, 0.9);
        ")}>
            <Header on_info_toggle={handle_info_toggle}/>
            <ContentWrapper>
                <TextInput on_change={Some(input_change_cb)}/>
                <div class={css!("
                    width: 100%;
                    display: flex;
                    flex-direction: row;

                    & > :not(:last-child) {
                        margin-right: 1rem;
                    }
                    @media only screen and (max-width: 450px) {
                        justify-content: space-between;
                        & > :not(:last-child) {
                            margin-right: -1px;
                        }
                    }
                ")}>
                    <StatisticsView name={"chars"} value={*n_chars_with_spaces}/>
                    <StatisticsView name={"without spaces"} value={*n_chars_without_spaces}/>
                    <StatisticsView name={"words"} value={*n_words}/>
                </div>
                <InfoOverlay show={*show_info}/>
            </ContentWrapper>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}