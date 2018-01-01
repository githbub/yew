#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::html::*;

// Own services implementation
mod gavatar;
use gavatar::GavatarService;
//mod ccxt;
//use ccxt::CcxtService;

struct Context {
    gavatar: GavatarService<Msg>,
    //ccxt: CcxtService,
}

struct Model {
}

enum Msg {
    Exchanges,
}

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Exchanges => {
            //context.ccxt.exchanges();
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <button onclick=|_| Msg::Exchanges,>{ "Get Exchanges" }</button>
        </div>
    }
}

fn main() {
    let mut app = App::new();
    let context = Context {
        gavatar: GavatarService::new(app.sender()),
        //ccxt: CcxtService::new(),
    };
    let model = Model {
    };
    app.run(context, model, update, view);
}
