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
pub struct ViewFooterProps {
    pub view: AppView,
    pub onchange: Callback<AppView>,
}

#[function_component(ViewFooter)]
pub fn view_header(props: &ViewFooterProps) -> Html {
    html! {
        <ul class="nav nav-tabs nav-fill nav-pills bg-light fixed-bottom">
            {
                AppView::iter().map(|view: AppView| {
                    let callback = props.onchange.clone();
                    let nextview = view.clone();
                    html!{
                        <li class="nav-item">
                            <div
                                class={match props.view == view {
                                    true => "btn nav-link active",
                                    false => "btn nav-link",
                                }}
                                onclick={Callback::from(move |_| {
                                    callback.emit(nextview.clone())
                                })}
                            >
                                {view.icon()}
                            </div>
                        </li>
                    }
                }).collect::<Html>()
            }
        </ul>
    }
}
