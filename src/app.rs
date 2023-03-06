use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use log::info;
use yewdux::prelude::*;


use crate::state::AppState;
use crate::entities::Thing;

fn log_message(message: &str) {
    let object = JsValue::from(message);
    info!("{}", object.as_string().unwrap());
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


#[derive(Properties, PartialEq)]
struct Props {
    pub children: Children,
    pub to: Route,
}
#[function_component(RouteLink)]
fn route_link(props: &Props) -> Html {
    let route = use_route::<Route>().unwrap_or_default();
    let classes = if route == props.to {
        classes!("active")
    } else {
        classes!("")
    };
    html! {
        <Link<Route> classes={classes} to={props.to.clone()}>{for props.children.iter() }</Link<Route>>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]Home,
    #[at("/thing")]Things,
    #[at("/template")]Templates,
    #[at("/category")]Categories,
    #[at("/tag")]Tags,
    #[at("/character")]Character,

    #[not_found] #[at("/404")] NotFound,
}

#[function_component]
fn NavBar() -> Html {
    html! {
        <nav>
            <ul>
            //<li><RouteLink to={Route::Home}>{"HOME"}</RouteLink></li>
            <li><RouteLink to={Route::Things}>{"THING"}</RouteLink></li>
            <li><RouteLink to={Route::Templates}>{"TEMPLATE"}</RouteLink></li>
            <li><RouteLink to={Route::Categories}>{"CATEGORY"}</RouteLink></li>
            <li><RouteLink to={Route::Tags}>{"TAG"}</RouteLink></li>
            <li><RouteLink to={Route::Character}>{"CHARACTER"}</RouteLink></li>
            </ul>
        </nav>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/> },
        Route::Things => html! { <ThingPage/> },
        Route::Templates => html! { <HomePage/> },
        Route::Tags => html! { <HomePage/> },
        Route::Character => html! { <HomePage/> },
        Route::Categories => html! { <HomePage/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
pub fn my_routes() -> Html {
    html! {
        <BrowserRouter>
            <NavBar />
            <main><Switch<Route> render={switch} /></main>
            <menu>{"menu text here"}</menu>
            <footer>{"footer"}</footer>
        </BrowserRouter>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    //log_message("Hello world");
    html! { <Main/> }
}


#[function_component]
fn ThingPage() -> Html {
    html! {
        "thing page"
    }
}

#[function_component]
fn HomePage() -> Html {
    //let (counter, dispatcher) = use_store::<Counter>();
    //let onclick = dispatcher.apply_callback(|_| Msg::AddOne);
    //let negate = dispatcher.apply_callback(|_| Msg::Minus);
    //let clear = dispatcher.apply_callback(|_| Msg::Clear);

    html! {
        <div>
            {"This is the home page here..."}
            <p></p>
            <button class="primary">{"+1"}</button>
            <button class="accent">{"-1"}</button>
            <button class="warn">{"=0"}</button>
        </div>
    }
}
