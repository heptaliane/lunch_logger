use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use yew::callback::Callback;
use yew::function_component;
use yew::prelude::*;
use yew::Properties;
use yew_icons::{Icon, IconId};

#[derive(Clone, PartialEq, EnumIter)]
pub enum AppView {
    MapView,
    StoreView,
    MenuView,
}

impl AppView {
    pub fn title(&self) -> String {
        match &self {
            Self::MapView => "Map List".to_string(),
            Self::StoreView => "Store List".to_string(),
            Self::MenuView => "Menu List".to_string(),
        }
    }

    fn icon(&self) -> Html {
        html! {
            <Icon icon_id={match &self {
                Self::MapView => IconId::BootstrapMap,
                Self::StoreView => IconId::HeroiconsSolidBuildingStorefront,
                Self::MenuView => IconId::FontAwesomeSolidUtensils,
            }} />
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ViewHeaderProps {
    pub view: AppView,
    pub onchange: Callback<AppView>,
}

#[function_component(ViewHeader)]
pub fn view_header(props: &ViewHeaderProps) -> Html {
    html! {
        <nav class="navbar navbar-dark bg-primary">
            <div class="container-fluid">
                <div class="navbar-brand">
                    <button
                        class="btn btn-primary"
                    >
                        <Icon icon_id={IconId::FontAwesomeSolidArrowLeft} />
                    </button>
                </div>
                <div class="nav-item">
                    {props.view.title()}
                </div>
                <div class="btn-group">
                {
                    AppView::iter().map(|view: AppView| {
                        let callback = props.onchange.clone();
                        let nextview = view.clone();
                        html!{
                            <button
                                class={match props.view == view {
                                    true => "btn btn-primary active",
                                    false => "btn btn-primary",
                                }}
                                onclick={Callback::from(move |_| {
                                    callback.emit(nextview.clone())
                                })}
                            >
                                {view.icon()}
                            </button>
                        }
                    }).collect::<Html>()
                }
                </div>
            </div>
        </nav>
    }
}
