use i18nrs::yew::use_translation;
use yew::prelude::*;

#[function_component(PrivacyPolicy)]
pub fn privacy_policy() -> Html {
    let (i18n, _) = use_translation();

    html! {
        <div style="max-width: 900px; margin: auto;">
            <h1>{ i18n.t("privacy.title") }</h1>
            <p style="color: var(--color-text-secondary); margin-bottom: var(--spacing-xl);">
                { i18n.t("privacy.last_updated") }
            </p>

            <div class="card" style="margin-bottom: var(--spacing-lg); background-color: rgba(74, 144, 226, 0.1); border-color: var(--color-primary-accent);">
                <p style="font-size: 1.05rem;">{ i18n.t("privacy.intro") }</p>
            </div>

            <section class="card">
                <h2>{ i18n.t("privacy.h2_1") }</h2>
                <p>{ i18n.t("privacy.p1") }</p>
            </section>

            <section class="card">
                <h2>{ i18n.t("privacy.h2_2") }</h2>
                <p>{ i18n.t("privacy.p2_intro") }</p>
                <ul>
                    <li>{ i18n.t("privacy.p2_id") }</li>
                    <li>{ i18n.t("privacy.p2_tech") }</li>
                    <li>{ i18n.t("privacy.p2_service") }</li>
                </ul>
                <p>{ i18n.t("privacy.p2_note") }</p>
            </section>

            <section class="card">
                <h2>{ i18n.t("privacy.h2_3") }</h2>
                <ul>
                    <li>{ i18n.t("privacy.p3_1") }</li>
                    <li>{ i18n.t("privacy.p3_2") }</li>
                    <li>{ i18n.t("privacy.p3_3") }</li>
                    <li>{ i18n.t("privacy.p3_4") }</li>
                    <li>{ i18n.t("privacy.p3_5") }</li>
                    <li>{ i18n.t("privacy.p3_6") }</li>
                </ul>
            </section>

            <section class="card">
                <h2>{ i18n.t("privacy.h2_4") }</h2>
                <ul>
                    <li>{ i18n.t("privacy.p4_1") }</li>
                    <li>{ i18n.t("privacy.p4_2") }</li>
                    <li>{ i18n.t("privacy.p4_3") }</li>
                </ul>
            </section>

            <section class="card"><h2>{ i18n.t("privacy.h2_5") }</h2><p>{ i18n.t("privacy.p5") }</p></section>
            <section class="card"><h2>{ i18n.t("privacy.h2_6") }</h2><p>{ i18n.t("privacy.p6") }</p></section>
            <section class="card"><h2>{ i18n.t("privacy.h2_7") }</h2><p>{ i18n.t("privacy.p7") }</p></section>
            <section class="card"><h2>{ i18n.t("privacy.h2_8") }</h2><p>{ i18n.t("privacy.p8") }</p></section>
            <section class="card"><h2>{ i18n.t("privacy.h2_9") }</h2><p>{ i18n.t("privacy.p9") }</p></section>
            <section class="card"><h2>{ i18n.t("privacy.h2_10") }</h2><p>{ i18n.t("privacy.p10") }</p></section>
            <section class="card"><h2>{ i18n.t("privacy.h2_11") }</h2><p>{ i18n.t("privacy.p11") }</p></section>
            <section class="card"><h2>{ i18n.t("privacy.h2_12") }</h2><p>{ i18n.t("privacy.p12") }</p></section>
            <section class="card"><h2>{ i18n.t("privacy.h2_13") }</h2><p>{ i18n.t("privacy.p13") }</p></section>

            <div class="card" style="background-color: rgba(74, 144, 226, 0.1); border-color: var(--color-primary-accent); text-align: center;">
                <p style="font-weight: 600;">{ i18n.t("privacy.contact") }</p>
            </div>
        </div>
    }
}
