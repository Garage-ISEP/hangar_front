use crate::router::AppRoute;
use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Footer)]
pub fn footer() -> Html 
{
    let (i18n, _) = use_translation();

    html! 
    {
        <footer class="site-footer">
            <div class="footer-content">
                <p>{ "© 2025 Hangar - Garage Isep" }</p>
                <ul class="footer-links">
                    <li><Link<AppRoute> to={AppRoute::About}>{ i18n.t("footer.about") }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::Terms}>{ i18n.t("footer.terms") }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::Privacy}>{ i18n.t("footer.privacy") }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::Contact}>{ i18n.t("footer.contact") }</Link<AppRoute>></li>
                </ul>
            </div>
        </footer>
    }
}