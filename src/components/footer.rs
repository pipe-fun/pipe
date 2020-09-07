use yew::{
    html,
    Component,
    ComponentLink,
    Html,
};

pub struct Footer;

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self { Self }

    fn update(&mut self, _msg: Self::Message) -> bool { true }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {
            <footer>
                <div class="container">
                    <p class="text-center">
                        <a href="https://github.com/pipe-fun" target="_blank">
                        { "Spxg 's project" }
                        </a>
                    </p>
                </div>
            </footer>
        }
    }
}