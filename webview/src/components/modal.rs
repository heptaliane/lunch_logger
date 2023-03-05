use yew::function_component;
use yew::prelude::*;
use yew::Properties;


#[derive(Properties, PartialEq)]
pub struct ModalProps
{
    #[prop_or(true)]
    pub show: bool,
    #[prop_or(AttrValue::from(""))]
    pub title: AttrValue,
    pub children: Children,

    pub oncancel: Callback<()>,
    pub onsubmit: Callback<()>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html
{
    let oncancel = props.oncancel.clone();
    let onsubmit = props.onsubmit.clone();
    html! {
        <div
            class={match props.show {
                true => "modal fade show",
                false => "modal fade",
            }}
        >
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title">
                            {props.title.clone()}
                        </h5>
                    </div>
                    <div class="modal-body">
                        {props.children.clone()}
                    </div>
                    <div class="modal-footer">
                        <button
                            class="btn btn-secondary"
                            onclick={Callback::from(move |_| {
                                oncancel.emit(());
                            })}
                        >
                            {"Cancel"}
                        </button>
                        <button
                            class="btn btn-primary"
                            onclick={Callback::from(move |_| {
                                onsubmit.emit(());
                            })}
                        >
                            {"OK"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
