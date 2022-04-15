use yew::prelude::*;

//enum Msg {
//    AddOne,
//}
//
//struct Model {
//    value: i64,
//}
//
//impl Component for Model {
//    type Message = Msg;
//    type Properties = ();
//
//    fn create(ctx: &Context<Self>) -> Self {
//        Self { value: 0 }
//    }
//
//    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//        match msg {
//            Msg::AddOne => {
//                self.value += 1;
//                true
//            }
//        }
//    }
//
//    fn view(&self, ctx: &Context<Self>) -> Html {
//        let link = ctx.link();
//        html! {
//            <div>
//                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1 "} </button>
//                { self.value }
//            </div>
//        }
//    }
//}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| 0);

    let incr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    let decr_counter = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| state.set(*state - 1))
    };

    html!(
        <>
        <p> {"Current Count: "} {*state} </p>
        <button onclick={incr_counter}> {"+"} </button>
        <button onclick={decr_counter}> {"-"} </button>
        </>
    )
}

fn main() {
    yew::start_app::<App>();
}
