use yew::callback::Callback;
use yew::function_component;
use yew::prelude::*;
use yew::Properties;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct ViewHeaderProps {
    #[prop_or(AttrValue::from(""))]
    pub title: AttrValue,

    pub onback: Callback<()>,
}

#[function_component(ViewHeader)]
pub fn view_header(props: &ViewHeaderProps) -> Html {
    html! {
        <nav class="navbar navbar-dark bg-primary fixed-top">
            <div class="container-fluid">
                <div class="navbar-brand">
                    <button
                        class="btn btn-primary"
                    >
                        <Icon icon_id={IconId::FontAwesomeSolidArrowLeft} />
                    </button>
                </div>
                <div class="nav-item">
                    {props.title.clone()}
                </div>
                <div>
                    <div
                        class="btn btn-primary"
                    >
                        <Icon icon_id={IconId::OcticonsPlusCircle24} />
                    </div>
                    <div
                        class="btn btn-primary"
                    >
                        <Icon icon_id={IconId::OcticonsNoEntry24} />
                    </div>
                    <div
                        class="btn btn-primary"
                    >
                        <Icon icon_id={IconId::OcticonsPencil24} />
                    </div>
                    <div
                        class="btn btn-primary"
                    >
                        <Icon icon_id={IconId::LucideFilter} />
                    </div>
                </div>
            </div>
        </nav>
    }
}
