use full_palette::GREY;
use plotters::prelude::*;

use super::math;

pub fn draw_graph_on_svg(path: String,width: u32, height: u32, raw_series: Vec<f64> ,histogram: math::Histogram) -> Result<(), Box<dyn std::error::Error>> {
    SVGBackend::new(&path, (width, height)).into_drawing_area();
    // ==============================ГРАФИКА==============================
    // На чём рисовать
    let drawing_area =
        SVGBackend::new(&path, (width, height)).into_drawing_area();
    drawing_area.fill(&WHITE)?;
    let (left, right) = drawing_area.split_horizontally(width / 2);

    // ######################Левый график######################
    // Параметры графика
    let mut chart_builder = ChartBuilder::on(&left);
    chart_builder
        .margin(5)
        .set_left_and_bottom_label_area_size(20);
    // Границы графика
    let left_boundary = unsafe { histogram.occurrences.first().unwrap_unchecked().x - histogram.step };
    let right_boundary = unsafe { histogram.occurrences.last().unwrap_unchecked().x + 0.01 + histogram.step };

    let mut chart_context_left =
    chart_builder
        .build_cartesian_2d(
            (left_boundary..right_boundary)
                .step(histogram.step)
                .use_round()
                .into_segmented(),
            (0.0..1.01).step(0.01)
        )?
        .set_secondary_coord(
            (left_boundary - histogram.step / 2.0)..(right_boundary + histogram.step / 2.0),
            0.0..1.01
        );
    // Отрисовка сетки и системы координат
    chart_context_left.configure_mesh().draw()?;

    let graph_histogram = Histogram::vertical(&chart_context_left)
        .style(BLUE)
        .margin(0)
        .data(
            histogram
                .occurrences
                .iter()
                .map(|row| (row.x, row.occurrences as f64 / histogram.sample_size as f64)),
        );

    chart_context_left.draw_series(graph_histogram)?;

    chart_context_left.draw_secondary_series(PointSeries::of_element(
        raw_series.iter().map(|x| (*x, 0.05f64)),
        3.0f64,
        ShapeStyle::from(&GREY).filled(),
        &|coord, size, style| EmptyElement::at(coord) + Circle::new((0, 0), size, style),
    ))?;
    chart_context_left.configure_series_labels().draw()?;

    // ######################Правый график######################
    chart_builder = ChartBuilder::on(&right);
    chart_builder
        .margin(5)
        .set_left_and_bottom_label_area_size(20);

    let mut chart_context_right =
    // Границы графика
    chart_builder.build_cartesian_2d(left_boundary..right_boundary - 0.001, (0.0..1.01).step(0.01))?;

    chart_context_right.configure_mesh().draw()?;
    let mut lines = Vec::<(f64,f64)>::with_capacity(histogram.occurrences.len() + 1);
    lines.push((left_boundary,0.0));
    for occurrency in histogram.occurrences.iter() {
        lines.push((occurrency.x - histogram.step/2.0,unsafe { lines.last().unwrap_unchecked().1 }));
        lines.push((occurrency.x - histogram.step/2.0,unsafe { lines.last().unwrap_unchecked().1} + occurrency.occurrences as f64 / raw_series.len() as f64 ));
    }
    lines.push((right_boundary,1.0));

    let cum_distribution_function =
        LineSeries::new(lines, &BLACK);
    chart_context_right.draw_series(cum_distribution_function)?;
    // ==============================ГРАФИКА==============================
    Ok(())
}
