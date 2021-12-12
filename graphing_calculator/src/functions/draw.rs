use super::{eval, pretty};
use crate::syntax::parser::AST;
use plotters::prelude::*;

pub fn draw(file: &String, statement: Vec<AST>) -> Result<(), Box<dyn std::error::Error>> {
  let root = SVGBackend::new(file, (750, 750)).into_drawing_area();
  root.fill(&WHITE)?;
  let mut chart = ChartBuilder::on(&root)
    .caption("y=x^2", ("Arial", 50).into_font())
    .margin(5)
    .x_label_area_size(30)
    .y_label_area_size(30)
    .build_cartesian_2d(-20f64..20f64, -20f64..20f64)?;

  chart.configure_mesh().draw()?;

  for s in statement {
    match s {
      AST::Identity(statement) => {
        chart
          .draw_series(LineSeries::new(
            {
              let mut po = Vec::<(f64, f64)>::new();
              let mut no = Vec::<(f64, f64)>::new();
              for x in (-200..=200).map(|x| x as f64 / 10.0) {
                let a = eval(*statement.identity[1].clone(), x);
                if !a[0].is_nan() {
                  po.push((x, a[0]));
                }
                if !a[1].is_nan() {
                  no.push((x, a[1]));
                }
              }
              no.reverse();
              po.extend(no);
              po
            },
            &RED,
          ))?
          .label(pretty(AST::Identity(statement)))
          .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
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
