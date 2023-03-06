use app::App;

// start with "trunk serve" for web, or desktop "cargo tauri dev"
fn main() {
    yew::Renderer::<App>::new().render();
}
