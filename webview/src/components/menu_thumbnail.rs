use chrono::NaiveDateTime;
use yew::callback::Callback;
use yew::function_component;
use yew::prelude::*;
use yew::Properties;

use super::super::data::Menu;

static PRICE_DIGIT: usize = 2;

#[derive(Properties, PartialEq)]
pub struct MenuThumbnailProps {
    pub id: usize,
    pub menu: Menu,
    pub onclick: Callback<usize>,
}

#[function_component(MenuThumbnail)]
pub fn menu_thumbnail(props: &MenuThumbnailProps) -> Html {
    let onclick = props.onclick.clone();
    let id = props.id.clone();
    html! {
        <div
            class="card thumbnail-container"
            onclick={Callback::from(move |_| {
                onclick.emit(id)
            })}
        >
            <img
                src={props.menu.image.clone()}
                class="thumbnail-image"
                loading="lazy"
            />
            <div class="thumbnail-text thumbnail-name">
                {props.menu.name.clone()}
            </div>
            <div class="thumbnail-text thumbnail-rate">
                {props.menu.rate}
            </div>
            <div class="thumbnail-text thumbnail-date">
                {
                    NaiveDateTime::from_timestamp_millis(props.menu.timestamp)
                    .expect("Invalid datetime")
                    .format("%Y-%m-%d").to_string()
                }
            </div>
            <div class="thumbnail-text thumbnail-price">
                {format!(
                    "{price:.*} {}",
                    PRICE_DIGIT,
                    props.menu.price_unit.clone(),
                    price=props.menu.price,
                )}
            </div>
        </div>
    }
}
