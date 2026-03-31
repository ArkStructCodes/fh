use plotters::prelude::*;

use crate::telemetry::EngineData;

const MINT: ShapeStyle = ShapeStyle {
    color: RGBAColor(74, 238, 180, 1.0),
    filled: false,
    stroke_width: 4,
};

const HOT_PINK: ShapeStyle = ShapeStyle {
    color: RGBAColor(247, 1, 137, 1.0),
    filled: false,
    stroke_width: 4,
};

type Coord = (i32, i32);

fn render_legend_line((x, y): Coord, style: ShapeStyle) -> PathElement<Coord> {
    PathElement::new(vec![(x, y), (x + 20, y)], style)
}

fn pad_axis(component: f32, value: f32) -> f32 {
    (f32::ceil(component / 100.0) * 100.0) + value
}

pub(crate) fn write_svg_to(
    path: &str,
    size: (u32, u32),
    data: EngineData,
) -> Result<(), Box<dyn std::error::Error>> {
    // create a drawing area for the chart
    let area = SVGBackend::new(path, size).into_drawing_area();
    area.fill(&WHITE)?;

    // set up font scaling according to the area
    let series_label_font = ("sans-serif", 18).into_text_style(&area).font;
    let label_style = ("sans-serif", 20)
        .into_text_style(&area)
        .font
        .style(FontStyle::Bold);

    let mut chart = ChartBuilder::on(&area)
        .set_label_area_size(LabelAreaPosition::Left, 48)
        .set_label_area_size(LabelAreaPosition::Bottom, 72)
        .margin(36)
        .build_cartesian_2d(
            data.idle_rpm..data.max_rpm,
            0.0..pad_axis(data.peak, 200.0),
        )?;

    chart
        .configure_mesh()
        .disable_mesh()
        .label_style(label_style)
        .x_desc("RPM x1000")
        .x_label_formatter(&|x| format!("{:.0}", x / 1000.0))
        .y_label_formatter(&|y| format!("{:.0}", y))
        .y_labels(4)
        .draw()?;

    chart
        .draw_series(LineSeries::new(data.power, MINT))?
        .label("Power - kW")
        .legend(move |coord| render_legend_line(coord, MINT));

    chart
        .draw_series(LineSeries::new(data.torque, HOT_PINK))?
        .label("Torque - Nm")
        .legend(move |coord| render_legend_line(coord, HOT_PINK));

    // configure the legend further
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .border_style(&BLACK)
        .margin(12)
        .label_font(series_label_font)
        .draw()?;

    area.present()?;
    Ok(())
}
