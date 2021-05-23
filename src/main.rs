#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::{html::IntoPropValue, web_sys::Url};
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;

mod pages;
use pages::{home::Home, users::Users};
struct App;

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/users"]
    Users,
    #[to = "/projects"]
    Projects,
    #[to = "/"]
    Index,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;

        html! {
            <>
            <div> {{"hello!"}}
              <Anchor classes="navbar-item" route=AppRoute::Users>
              { "Users" }
              </Anchor>
              <Anchor classes="navbar-item" route=AppRoute::Projects>
              { "Projects" }
              </Anchor>
              <Anchor classes="navbar-item" route=AppRoute::Index>
              { "Home" }
              </Anchor>
            </div>
            <main>
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Users => html!{<Users/>},
                        AppRoute::Projects => html!{<Home/>},
                        AppRoute::Index => html!{<Home/>},
                    }
                })
            />
            </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
