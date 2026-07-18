use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GaugeProps 
{
    pub label: String,
    pub value: f64,
    pub max_value: f64,
    pub unit: String,
}

#[function_component(Gauge)]
pub fn gauge(props: &GaugeProps) -> Html 
{
    let percentage = if props.max_value > 0.0 
    {
        (props.value / props.max_value).clamp(0.0, 1.0)
    } 
    else 
    {
        0.0
    };

    let color_class = if percentage > 90.0 
    {
        "gauge-progress-danger"
    } 
    else if percentage > 70.0 
    {
        "gauge-progress-warning"
    } 
    else 
    {
        "gauge-progress-normal"
    };

    let radius = 52.0;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let offset = circumference - (percentage / 100.0 * circumference);

    let display_info = if props.unit == "%" 
    {
        html! {}
    } 
    else 
    {
        let value_mib = props.value / (1024.0 * 1024.0);
        let max_value_mib = props.max_value / (1024.0 * 1024.0);
        html! { format!("{:.0} / {:.0} {}", value_mib, max_value_mib, props.unit) }
    };

    html! 
    {
        <div class="gauge-container">
            <svg class="gauge-svg" viewBox="0 0 120 120">
                <circle class="gauge-background" cx="60" cy="60" r={radius.to_string()} />
                <circle
                    class={classes!("gauge-progress", color_class)}
                    cx="60"
                    cy="60"
                    r={radius.to_string()}
                    style={format!("stroke-dasharray: {}; stroke-dashoffset: {};", circumference, offset)}
                />

                <text
                    x="50%"
                    y="50%"
                    dominant-baseline="middle"
                    text-anchor="middle"
                    dy="-0.5em"
                    class="gauge-value"
                >
                    { format!("{:.1}", percentage) }
                </text>
                <text
                    x="50%"
                    y="50%"
                    dominant-baseline="middle"
                    text-anchor="middle"
                    dy="1.0em"
                    class="gauge-label"
                >
                    { &props.label }
                </text>
            </svg>
            <div class="gauge-info">
                { display_info }
            </div>
        </div>
    }
}