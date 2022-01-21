use super::eval;
use crate::syntax::parser::AST;
use plotters::prelude::*;

pub fn draw(
  file: &String,
  statement: Vec<(AST, String)>,
  title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let root = SVGBackend::new(file, (750, 750)).into_drawing_area();
  root.fill(&RGBColor(102, 102, 102))?;
  let mut chart = ChartBuilder::on(&root)
    .caption(title, ("Arial", 50).into_font())
    .margin(5)
    .x_label_area_size(30)
    .y_label_area_size(30)
    .build_cartesian_2d(-20f64..20f64, -20f64..20f64)?;

  chart.configure_mesh().draw()?;

  let colours: Vec<f64> = (0..statement.len())
    .map(|v| v as f64 / statement.len() as f64)
    .collect();
  for i in 0..statement.len() {
    match statement[i].0.clone() {
      AST::Identity(identity) => {
        let cs = colours.clone();
        chart
          .draw_series(LineSeries::new(
            {
              let mut po = Vec::<(f64, f64)>::new();
              let mut no = Vec::<(f64, f64)>::new();
              for x in (-200..=200).map(|x| x as f64 / 10.0) {
                let a = eval(*identity.identity[1].clone(), x);
                if !a[0].is_nan() {
                  po.push((x, a[0]));
                }
                if a.len() > 1 && !a[1].is_nan() {
                  no.push((x, a[1]));
                }
              }
              no.reverse();
              po.extend(no);
              po
            },
            HSLColor(colours[i], 1f64, 0.5f64),
          ))?
          .label(statement[i].1.clone())
          .legend(move |(x, y)| {
            PathElement::new(
              vec![(x, y), (x + 20, y)],
              HSLColor(cs[i], 1f64, 0.5f64).filled(),
            )
          });
      }
      _ => {}
    }
  }
  chart
    .configure_series_labels()
    .background_style(&WHITE.mix(0.8))
    .border_style(&BLACK)
    .draw()?;

  Ok(())
}
