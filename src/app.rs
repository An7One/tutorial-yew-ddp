use rand::prelude::*;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::dialog::DialogService;

pub struct App{
    items: Vec<i64>,
    link: ComponentLink<Self>,
}

pub enum Msg{
    AddOne,
    RemoveOne,
    About,
}

impl Component for App{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App{
            link,
            items: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::AddOne => {
                let added: i64 = random();
                self.items.push(added);
                ConsoleService::info(format!("Added {}", added).as_str());
            },
            Msg::RemoveOne => {
                let removed = self.items.pop();
                match removed{
                    Some(x) => ConsoleService::info(format!("Removed {}", x).as_str()),
                    None => {
                        ConsoleService::error("No more element to remove!");
                        let user_is_a_monkey = DialogService::confirm("No more elements to remove!");
                        if user_is_a_monkey{
                            DialogService::alert("I knew it!");
                        }else{
                            DialogService::alert("Maybe it was an error.");
                        }
                    }
                }
            },
            Msg::About => DialogService::alert("Purposeless App"),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render_item = |item|{
            html!{
                <>
                    <tr><td>{ item } </td></tr>
                </>
            }
        };
        html!{
            <div class="main">
                <div class="card">
                    <header>
                        <h2>{"Items: "}</h2>
                        <button onclick=self.link.callback(|_| Msg::About)>{"About"}</button>
                    </header>
                    <div class="card-body">
                        <table class="primary">
                            { for self.items.iter().map(render_item) }
                        </table>
                    </div>
                    <footer>
                        <button onclick=self.link.callback(|_| Msg::AddOne)>{"Add 1"}</button>
                        <button onclick=self.link.callback(|_| Msg::RemoveOne)>{"Remove 1"}</button>
                    </footer>
                </div>
            </div>
        }
    }
}