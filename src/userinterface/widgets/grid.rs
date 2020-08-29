extern crate std;
use std::cmp::{min, max};
use std::convert::TryFrom;

extern crate tui;
use tui::buffer::Buffer;
use tui::layout::{Direction, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Widget};

use crate::userinterface::widgets::{BORDERS, RGB_SUM_DIVIDED_BY_2};
use crate::userinterface::widgets::scroller::{self, SCROLLER};

use crate::log;
use crate::utils::FullPixel;

pub const TAB_WIDTH: u16 = 17;

const LIGHT_GREY: Color = Color::Rgb(200, 200, 200);

/// Widget to render a colorizable grid
pub struct GridWidget<'a> {
  log: &'a mut log::Log,
}

impl<'a> GridWidget<'a> {
  pub fn new(log: &'a mut log::Log) -> GridWidget<'a> {
    GridWidget {
      log: log,
    }
  }

  fn render_inner(&mut self, buf: &mut Buffer,
    (left, top, width, height): &(u16, u16, u16, u16)) {

      let inner = Rect::new(left + 1, top + 1, width - BORDERS,
        height - BORDERS);

      let (inner_left, inner_right, inner_top, inner_bottom) =
        (inner.left(), inner.right(), inner.top(), inner.bottom());

      buf.set_style(inner, Style::default().bg(Color::Black));

      let (scroll_x, scroll_y) = self.log.gridscroll_getscroll();
      self.log.check_last_action(
        (inner_left, inner_right + scroll_x,
          inner_top, inner_bottom + scroll_y));
      let current_grid = self.log.grids_getcurrentgrid();

      let mut row;
      let mut col;
      for (pixel, x, y) in current_grid {
        if (x + inner_left < scroll_x) ||
          (x + inner_left - scroll_x >= inner_right) {
            row = *left;
        } else {
          row = x + inner_left - scroll_x;
        }
        if (y + inner_top < scroll_y) ||
          (y + inner_top - scroll_y >= inner_bottom) {
          col = *top;
        } else {
          col = y + inner_top - scroll_y;
        }
        if (row > *left) && (col > *top) {
          match pixel {
            FullPixel::Body => buf.get_mut(row, col).set_bg(Color::Yellow),
            FullPixel::Border => buf.get_mut(row, col).set_bg(Color::Red),
            FullPixel::BodyBorder => buf.get_mut(row, col).set_bg(Color::Blue),
            FullPixel::BodyEmpty => buf.get_mut(row, col).set_bg(Color::Green),
            FullPixel::SpecificColor(red, green, blue) => {
              let rgb_sum =
                [red, green, blue].iter().map(|&x| x as u16).sum::<u16>();
              let fg_color = if rgb_sum > RGB_SUM_DIVIDED_BY_2 {
                  Color::Black
                } else {
                  Color::White
              };
              buf.get_mut(row, col).set_symbol("C")
                .set_bg(Color::Rgb(red, green, blue)).set_fg(fg_color)
            },
          };
        }
      }
  }

  fn render_scrollers(&mut self, buf: &mut Buffer,
    (borders_left, borders_top, borders_width, borders_height):
    &(u16, u16, u16, u16)) {

      let (scroll_x, scroll_y) = self.log.gridscroll_getscroll();
      let (grid_width, grid_height) =
        (self.log.grids_getwidth(), self.log.grids_getheight());
      let (inner_width, inner_height) =
        (borders_width - BORDERS, borders_height - BORDERS);

      if grid_width > inner_width {
        let hozizontal_scroller = scroller::ScrollerWidget::new(
          Direction::Horizontal, scroll_x, grid_width);
        let hscroller_area = Rect::new(
          *borders_left, borders_top + borders_height, inner_width, SCROLLER);
        hozizontal_scroller.render(hscroller_area, buf);
      }

      if grid_height > inner_height {
        let vertical_scroller = scroller::ScrollerWidget::new(
          Direction::Vertical, scroll_y, grid_height);
        let vscroller_area =
          Rect::new(borders_left - 1, *borders_top, SCROLLER, inner_height);
        vertical_scroller.render(vscroller_area, buf);
      }
  }

  fn render_tabs(&mut self, buf: &mut Buffer,
    (left, top, width, height): (u16, u16, u16, u16)) {

      let inside_workspace = Block::default()
        .style(Style::default().bg(LIGHT_GREY));
      let inside_workspace_area = Rect::new(left + 1, top + 1,
        width - TAB_WIDTH + 1, height);
      inside_workspace.render(inside_workspace_area, buf);

      let (names, current_grid) =
        (self.log.grids_getnames(), self.log.grids_getcurrentgridid());
      let tabs_height = u16::try_from(names.len()).unwrap();
      let tabs_area = Rect::new(left + width + BORDERS - TAB_WIDTH, top + 1,
        TAB_WIDTH - 1, tabs_height);
      let names = names.iter().enumerate().map(|(index, key)| {
        if index == current_grid {
          Spans::from(vec![Span::styled(key,
            Style::default().fg(LIGHT_GREY)
              .add_modifier(Modifier::REVERSED))])
        } else {
          Spans::from(vec![Span::styled(key,
            Style::default().add_modifier(Modifier::UNDERLINED))])
        }
      }).collect::<Vec<Spans>>();
      Paragraph::new(names).render(tabs_area, buf);
    }
}

impl<'a> Widget for GridWidget<'a> {

  fn render(mut self, area: Rect, buf: &mut Buffer) {

    let workspace_borders = Block::default().title(" Grids ")
      .borders(Borders::ALL);

    workspace_borders.render(area, buf);

    let workspace_left = area.left();
    let workspace_right = area.right();
    let workspace_top = area.top();
    let workspace_bottom = area.bottom();
    let workspace_width = workspace_right - workspace_left - BORDERS;
    let workspace_height = workspace_bottom - workspace_top - BORDERS;

    let (grid_width, grid_height) =
      (self.log.grids_getwidth::<u16>(), self.log.grids_getheight::<u16>());

    let left_borders;
    if (buf.area.right() - buf.area.left()) % 2 == 0 {
      if workspace_left + SCROLLER + workspace_width/2 >
        grid_width/2 + TAB_WIDTH/2 {
          left_borders =
            max(workspace_left + SCROLLER + workspace_width/2 -
              grid_width/2 - TAB_WIDTH/2,
            workspace_left + SCROLLER + 1);
      } else {
        left_borders = workspace_left + SCROLLER + 1;
      }
    } else {
      if workspace_left + SCROLLER + workspace_width/2 >
        grid_width/2 + TAB_WIDTH/2 + 1 {
          left_borders =
            max(workspace_left + SCROLLER + workspace_width/2 -
              grid_width/2 - TAB_WIDTH/2 - 1,
            workspace_left + SCROLLER + 1);
      } else {
        left_borders = workspace_left + SCROLLER + 1;
      }
    }

    let top_borders;
    if (buf.area.bottom() - buf.area.top()) % 2 == 0 {
      if workspace_top + workspace_height/2 > grid_height/2 + 1 {
        top_borders =
          max(workspace_top + workspace_height/2 - grid_height/2 - 1,
          workspace_top + 1);
      } else {
        top_borders = workspace_top + 1;
      }
    } else {
      if workspace_top + workspace_height/2 > grid_height/2 {
        top_borders = max(workspace_top + workspace_height/2 - grid_height/2,
          workspace_top + 1);
      } else {
        top_borders = workspace_top + 1;
      }
    }

    let max_width =
      workspace_right - workspace_left - BORDERS - SCROLLER - TAB_WIDTH;
    let max_height = workspace_bottom - workspace_top - BORDERS - SCROLLER;
    let width = min(self.log.grids_getwidth::<u16>() + BORDERS, max_width);
    let height =
      min(self.log.grids_getheight::<u16>() + BORDERS, max_height);
    let borders_area = (left_borders, top_borders, width, height);

    self.log.gridscroll_checkscroll(
      &(max_width - BORDERS, max_height - BORDERS));

    self.render_tabs(buf, (workspace_left, workspace_top, workspace_width,
      workspace_height));
    self.render_scrollers(buf, &borders_area);
    self.render_inner(buf, &borders_area);
  }
}
