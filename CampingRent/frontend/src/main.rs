use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct App {
    link: ComponentLink<Self>,
    gears: Vec<CampingGear>,
    transactions: Vec<RentalTransaction>,
}

struct CampingGear {
    id: u64,
    name: String,
    description: String,
    daily_rate: u64,
    quantity: u64,
}

struct RentalTransaction {
    user: String,
    gear_id: u64,
    days: u64,
    total_cost: u64,
    is_returned: bool,
}

enum Msg {
    FetchGears,
    FetchTransactions,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            gears: Vec::new(),
            transactions: Vec::new(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchGears => {
                // Lakukan fetch gears dari backend
                self.gears = vec![]; // Placeholder untuk data
                true
            }
            Msg::FetchTransactions => {
                // Lakukan fetch transaksi dari backend
                self.transactions = vec![]; // Placeholder untuk data
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Persewaan Alat Camping" }</h1>
                <button onclick={ctx.link().callback(|_| Msg::FetchGears)}>{ "Lihat Alat Camping" }</button>
                <ul>
                    { for self.gears.iter().map(|gear| html! {
                        <li>{ format!("{} - {} - Rp{}/hari - {} unit", gear.name, gear.description, gear.daily_rate, gear.quantity) }</li>
                    }) }
                </ul>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<App>::new().mount_to_body();
}
