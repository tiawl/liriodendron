extern crate tui;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Gauge, Widget};

pub const GAUGE_HEIGHT: u16 = 3;

/// Color C/C++ style enum
#[non_exhaustive]
pub struct Rgb;

impl Rgb {
  pub const RED: (f64, f64, f64) = (255., 0., 0.);
  pub const GREEN: (f64, f64, f64) = (0., 255., 0.);
  pub const YELLOW: (f64, f64, f64) = (255., 255., 0.);
}

/// Widget to render a loading gauge with a progressive color
pub struct GaugeWidget {
  filled_ratio: f64,
  label: String,
  init_color: (f64, f64, f64),
  ongoing_color: (f64, f64, f64),
  color_ratio: f64,
}

impl GaugeWidget {

  pub fn new(filled_ratio: f64, label: &String,
    (init_color, ongoing_color, color_ratio):
    ((f64, f64, f64), (f64, f64, f64), f64)) -> GaugeWidget {
      GaugeWidget {
        filled_ratio,
        label: label.clone(),
        init_color,
        ongoing_color,
        color_ratio,
      }
  }
}

impl Widget for GaugeWidget {

  fn render(self, area: Rect, buf: &mut Buffer) {
    let gauge_area = Rect::new(area.left(), area.top(),
      area.right() - area.left(), GAUGE_HEIGHT);
    let gauge_color = Color::Rgb(
      (self.init_color.0 * (1. - self.color_ratio) +
        self.ongoing_color.0 * self.color_ratio) as u8,
      (self.init_color.1 * (1. - self.color_ratio) +
        self.ongoing_color.1 * self.color_ratio) as u8,
      (self.init_color.2 * (1. - self.color_ratio) +
        self.ongoing_color.2 * self.color_ratio) as u8);
    let gauge = Gauge::default()
      .block(Block::default().borders(Borders::ALL))
      .gauge_style(Style::default().fg(gauge_color).bg(Color::Black))
      .ratio(self.filled_ratio)
      .label(Span::styled(self.label, Style::default()));
    gauge.render(gauge_area, buf);
  }
}
