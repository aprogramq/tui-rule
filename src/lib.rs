pub mod presets;
use colorgrad::Gradient;
use derive_builder::Builder;
use getset::{Getters, Setters};
use ratatui::{
    prelude::{

    layout::Margin,
    },
    prelude::{Alignment, Buffer, Rect},
    style::Color,
    text::Line,
    widgets::{Padding, Widget, WidgetRef},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "all")]
/// ## The Rule widget
/// ### Allows:
///     - Vertical alignment
///     - Horizontal alignment
///     - Horizontal and vertical paddings
///     - Center symbols
///     - Horizontal and vertical orientation
///     - Colorgrad gradients
///     - Start and end symbols
pub struct Rule {
    pub gradient: Option<Box<dyn Gradient>>,
    pub symbol_set: Set,
    pub orientation: Orientation,
    pub padding: Padding,
    pub vertical_alignment: VerticalAlignment,
    pub horizontal_alignment: Alignment,
    pub extra_rep_1: usize,
    pub extra_rep_2: usize,
    pub bg: Bg,
    pub area_margin: Margin,
}
pub enum Bg {
    None,
    Solid(Color),
    Gradient,
    GradientCustom(Box<dyn Gradient>),
}
#[macro_export]
macro_rules! create_segment {
    ($set:expr, $p_1:expr, $p_2:expr, $base_area:expr, $orientation:expr, $h_alignment:expr, $v_alignment:expr, $extra_rep_1:expr, $extra_rep_2:expr) => {{
        let rep_count: f32 = ($base_area / 2.0) - 1.0;
        let seg1 = $set.rep_1.to_string().repeat(
            (rep_count.floor() as usize)
                .saturating_sub($p_1)
                .saturating_add($extra_rep_1),
        );
        let seg2 = $set.rep_2.to_string().repeat(
            (rep_count.round() as usize)
                .saturating_sub($p_2 + 1)
                .saturating_add($extra_rep_2),
        );
        let mut ln = String::with_capacity(
            $p_1 + $p_2
                + 1
                + seg1.len()
                + 1
                + seg2.len()
                + 5,
        );
        ln.push_str(&String::from(" ").repeat(
            match $orientation {
                Orientation::Horizontal => {
                    match $h_alignment {
                        Alignment::Left => 0,
                        Alignment::Center => $p_1,

                        Alignment::Right => {
                            $p_1.saturating_add($p_2)
                        }
                    }
                }
                Orientation::Vertical => match $v_alignment
                {
                    VerticalAlignment::Top => 0,
                    VerticalAlignment::Center => $p_1,

                    VerticalAlignment::Bottom => {
                        $p_1.saturating_add($p_2)
                    }
                },
            } as usize,
        ));
        ln.push($set.start);
        ln.push_str(&seg1);
        ln.push($set.center);
        ln.push_str(&seg2);
        ln.push($set.end);
        ln.push_str(&String::from(" ").repeat(
            match $orientation {
                Orientation::Horizontal => {
                    match $h_alignment {
                        Alignment::Left => {
                            $p_2.saturating_add($p_1)
                        }
                        Alignment::Center => $p_2,
                        Alignment::Right => 0,
                    }
                }
                Orientation::Vertical => match $v_alignment
                {
                    VerticalAlignment::Top => {
                        $p_1.saturating_add($p_2)
                    }
                    VerticalAlignment::Center => $p_2,
                    VerticalAlignment::Bottom => 0,
                },
            } as usize,
        ));
        ln
    }};
}
/// ### Symbol set struct
/// ```
/// let set = Set {
///     start: '+',
///     rep_1: '─',
///     center: '+',
///     rep_2: '─',
///     end: '+',
/// };
/// let rule = Rule::from_set(set);
/// // Contents would be "+───+───+"
/// frame.render_widget(rule, frame.area());
/// ```
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize)
)]
#[derive(Builder, Getters, Setters, Debug, Clone)]
pub struct Set {
    #[builder(default = "'─'")]
    pub start: char,
    #[builder(default = "'─'")]
    pub end: char,
    #[builder(default = "'─'")]
    pub rep_1: char,
    #[builder(default = "'─'")]
    pub rep_2: char,
    #[builder(default = "'─'")]
    pub center: char,
}
/// controls rule orientation
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Orientation {
    Vertical,
    Horizontal,
}
/// vertical version of the Alignment enum
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}
/// # Macro for generating gradient text that returns a `Vec<Span>` with the inputted gradient.
/// # Parameters
/// 1. any type that can be converted to Line (String, Line, &str, Vec<Span>)
/// 2. a colorgrad gradient (can be either Box<dyn Gradient> or an owned type)
///
/// ```rust
///     let gradient_text = generate_gradient_text!("Rainbow Text", colorgrad::preset::rainbow());
///     // displays "Rainbow Text" with a rainbow gradient
///     buf.set_line(1, 1, &gradient_text, gradient_text.width())
/// ```
#[macro_export]
macro_rules! generate_gradient_text {
    ($txt:expr, $gr:expr) => {{
        use ratatui::prelude::{Color, Style};
        let mut ln: Line = $txt.into();
        ln.spans = create_raw_spans!(ln.spans[0].content);
        let mut new_text = Vec::new();
        for (s, c) in ln
            .spans
            .clone()
            .into_iter()
            .zip($gr.colors(ln.width()))
        {
            new_text.push(s.style(Style::new().fg(
                Color::Rgb(
                    (c.r * 255.0) as u8,
                    (c.g * 255.0) as u8,
                    (c.b * 255.0) as u8,
                ),
            )));
        }
        new_text
    }};
    ($txt:expr, $gr:expr, $bgtype:expr) => {{
        use ratatui::prelude::{Color, Style};
        let mut ln: Line = $txt.into();
        ln.spans = create_raw_spans!(ln.spans[0].content);
        let mut new_text = Vec::new();
        match $bgtype {
            Bg::GradientCustom(grad) => {
                for (s, (c, c2)) in
                    ln.spans.clone().into_iter().zip(
                        $gr.colors(ln.width())
                            .into_iter()
                            .zip(grad.colors(ln.width())),
                    )
                {
                    new_text.push(
                        s.style(
                            Style::new()
                                .fg(Color::Rgb(
                                    (c.r * 255.0) as u8,
                                    (c.g * 255.0) as u8,
                                    (c.b * 255.0) as u8,
                                ))
                                .bg(Color::Rgb(
                                    (c2.r * 255.0) as u8,
                                    (c2.g * 255.0) as u8,
                                    (c2.b * 255.0) as u8,
                                )),
                        ),
                    );
                }
            }
            _ => {
                for (s, c) in ln
                    .spans
                    .clone()
                    .into_iter()
                    .zip($gr.colors(ln.width()))
                {
                    let c = Color::Rgb(
                        (c.r * 255.0) as u8,
                        (c.g * 255.0) as u8,
                        (c.b * 255.0) as u8,
                    );
                    new_text.push(s.style(
                        Style::new().fg(c).bg(
                            match $bgtype {
                                Bg::Solid(color) => *color,
                                Bg::Gradient => c,
                                _ => c,
                            },
                        ),
                    ));
                }
            }
        }
        new_text
    }};
}
#[cfg(feature = "all")]
impl Default for Rule {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "all")]
impl Rule {
    /// generates a new rule that looks like `─────────────` with no gradient and no padding
    /// centered horizontally and vertically by default
    pub fn new() -> Self {
        Self {
            gradient: None,
            symbol_set: Set {
                start: '─',
                end: '─',
                center: '─',
                rep_1: '─',
                rep_2: '─',
            },
            padding: Padding::new(0, 0, 0, 0),
            orientation: Orientation::Horizontal,
            horizontal_alignment: Alignment::Center,
            vertical_alignment: VerticalAlignment::Center,
            bg: Bg::None,
            area_margin: Margin::new(1, 1),
            extra_rep_1: 0,
            extra_rep_2: 0,
        }
    }
    pub fn area_margin(mut self, margin: Margin) -> Self {
        self.area_margin = margin;
        self
    }
    /// makes the bg solid
    pub fn bg_solid(mut self, c: Color) -> Self {
        self.bg = Bg::Solid(c);
        self
    }
    /// makes the bg use the same gradient as the fg
    pub fn bg_same_gradient(mut self) -> Self {
        self.bg = Bg::Gradient;
        self
    }
    pub fn extra_rep_1(mut self, rep: usize) -> Self {
        self.extra_rep_1 = rep;
        self
    }
    pub fn extra_rep_2(mut self, rep: usize) -> Self {
        self.extra_rep_2 = rep;
        self
    }
    pub fn extra_rep(
        mut self,
        rep_1: usize,
        rep_2: usize,
    ) -> Self {
        self.extra_rep_1 = rep_1;
        self.extra_rep_2 = rep_2;
        self
    }
    /// makes the bg a custom gradient
    pub fn bg_gradient<G: Gradient + 'static>(
        mut self,
        g: G,
    ) -> Self {
        self.bg = Bg::GradientCustom(Box::<G>::new(g));
        self
    }
    pub fn bg(mut self, bg: Bg) -> Self {
        self.bg = bg;
        self
    }
    /// creates a new vertical rule
    pub fn new_vertical() -> Self {
        Self::new().vertical()
    }
    /// Creates a new rule instance from a Set struct
    /// ```
    /// let rule = Rule::from_set(presets::horizontal::ASCII);
    /// // Has the start, end, center, right, and left symbols from the horizontal ASCII preset
    /// frame.render_widget(rule, frame.area())
    /// ```
    pub fn from_set(set: Set) -> Self {
        Self::new().with_set(set)
    }
    /// the new function and the with_gradient function combined
    /// ```rust
    ///     // displays a new rule with a rainbow gradient
    ///     Rule::new_with_gradient(colorgrad::preset::rainbow())
    /// ```
    pub fn new_with_gradient<G: Gradient + 'static>(
        gradient: G,
    ) -> Self {
        Self::new().with_gradient(gradient)
    }
    /// sets gradient for rule. uses colorgrad gradients
    /// ```rust
    ///     // displays `+=====+=====+` with a rainbow gradient
    ///     Rule::default().with_gradient(colorgrad::preset::rainbow())
    /// ```
    pub fn with_gradient<G: Gradient + 'static>(
        mut self,
        gradient: G,
    ) -> Self {
        self.gradient = Some(Box::<G>::new(gradient));
        self
    }
    /// sets the horizontal padding
    pub fn horizontal_padding(
        mut self,
        padding: u16,
    ) -> Self {
        self.padding.right = padding;
        self.padding.left = padding;
        self
    }
    /// sets the vertical padding
    pub fn vertical_padding(
        mut self,
        padding: u16,
    ) -> Self {
        self.padding.bottom = padding;
        self.padding.top = padding;
        self
    }
    /// Sets the right padding
    pub fn right_padding(mut self, padding: u16) -> Self {
        self.padding.right = padding;
        self
    }

    /// Sets the left padding
    pub fn left_padding(mut self, padding: u16) -> Self {
        self.padding.left = padding;
        self
    }

    /// Sets the top padding
    pub fn top_padding(mut self, padding: u16) -> Self {
        self.padding.top = padding;
        self
    }

    /// Sets the bottom padding
    pub fn bottom_padding(mut self, padding: u16) -> Self {
        self.padding.bottom = padding;
        self
    }
    /// Sets the end, start, right, center, and left symbols from the Set struct
    pub fn with_set(mut self, set: Set) -> Self {
        self = self
            .end(set.end)
            .start(set.start)
            .rep_2(set.rep_2)
            .rep_1(set.rep_1)
            .center(set.center);
        self
    }
    /// makes the rule horizontal instead of vertical. Horizontal by default
    pub fn horizontal(mut self) -> Self {
        self.orientation = Orientation::Horizontal;
        self
    }
    /// makes the rule a vertical rule instead of horizontal
    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }
    /// repeated symbol for right side
    /// ```rust
    ///     Rule::default().rep_2('-')
    /// ```
    /// `+=====+-----+`
    pub fn rep_2(mut self, symb: char) -> Self {
        self.symbol_set.rep_2 = symb;
        self
    }
    /// repeated symbol for left side
    /// ```rust
    ///     Rule::default().rep_1('-')
    /// ```
    /// `+-----+=====+`
    pub fn rep_1(mut self, symb: char) -> Self {
        self.symbol_set.rep_1 = symb;
        self
    }
    /// first symbol
    /// ```rust
    ///     Rule::default().start('%')
    /// ```
    /// `%=====+=====+`
    pub fn start(mut self, symb: char) -> Self {
        self.symbol_set.start = symb;
        self
    }
    /// last symbol
    ///```rust
    ///     Rule::default().end('%');
    ///```   
    /// `+=====+=====%`
    pub fn end(mut self, symb: char) -> Self {
        self.symbol_set.end = symb;
        self
    }
    /// center symbol  
    ///```rust
    ///     Rule::default().center('%')
    ///```
    /// `+=====%=====+`
    pub fn center(mut self, symb: char) -> Self {
        self.symbol_set.center = symb;
        self
    }
    /// the rep_1 and the rep_2 functions in one
    pub fn main_symbol(mut self, symb: char) -> Self {
        self = self.rep_1(symb).rep_2(symb);
        self
    }
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
    /// sets rule orientation
    /// ```rust
    ///     // creates a vertical rule
    ///     Rule::default().orientation(Orientation::Vertical)
    /// ```
    /// /// using the `.horizontal()` and `.vertical()` methods instead is recommended for simplicity
    pub fn orientation(
        mut self,
        orientation: Orientation,
    ) -> Self {
        self.orientation = orientation;
        self
    }
    /// sets vertical alignment
    /// centered by default
    /// ```rust
    ///     // creates a rule thats vertically aligned to the top
    ///     Rule::default().vertical_alignment(VerticalAlignment::Top)
    /// ```
    pub fn vertical_alignment(
        mut self,
        alignment: VerticalAlignment,
    ) -> Self {
        self.vertical_alignment = alignment;
        self
    }
    // sets horizontal alignment
    pub fn horizontal_alignment(
        mut self,
        alignment: Alignment,
    ) -> Self {
        self.horizontal_alignment = alignment;
        self
    }
}
#[cfg(feature = "all")]
impl Widget for Rule {
    fn render(self, area_old: Rect, buf: &mut Buffer) {
        self.render_ref(area_old, buf);
    }
}
#[cfg(test)]
mod tests {
    use ratatui::widgets::Block;
    #[test]
    pub fn test_hr() {
        use super::presets::test_sets::HORIZONTAL;
        use super::*;
        let mut buffer =
            Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(
                Line::raw("Horizontal Rule").centered(),
            )
            .title_bottom(
                Line::raw(" Vertical Alignment: Center ")
                    .centered(),
            )
            .render(buffer.area, &mut buffer);
        Rule::from_set(HORIZONTAL)
            .horizontal_padding(1)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal()
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
                "┌────────────────Horizontal Rule────────────────┐",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│ +─────────────────────+─────────────────────+ │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "└───────── Vertical Alignment: Center ──────────┘",
            ]);
        assert_eq!(buffer, expected);
        buffer = Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(
                Line::raw("Horizontal Rule").centered(),
            )
            .title_bottom(
                Line::raw(" Vertical Alignment: Top ")
                    .centered(),
            )
            .render(buffer.area, &mut buffer);
        Rule::from_set(HORIZONTAL)
            .horizontal_padding(1)
            .vertical_alignment(VerticalAlignment::Top)
            .horizontal()
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
                "┌────────────────Horizontal Rule────────────────┐",
                "│ +─────────────────────+─────────────────────+ │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "└─────────── Vertical Alignment: Top ───────────┘",
            ]);
        assert_eq!(buffer, expected);
        buffer = Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(
                Line::raw("Horizontal Rule").centered(),
            )
            .title_bottom(
                Line::raw(" Vertical Alignment: Bottom ")
                    .centered(),
            )
            .render(buffer.area, &mut buffer);
        Rule::from_set(HORIZONTAL)
            .horizontal_padding(1)
            .vertical_alignment(VerticalAlignment::Bottom)
            .horizontal()
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
                "┌────────────────Horizontal Rule────────────────┐",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│ +─────────────────────+─────────────────────+ │",
                "└───────── Vertical Alignment: Bottom ──────────┘",
            ]);
        assert_eq!(buffer, expected);
    }
    #[test]
    pub fn test_vr() {
        use super::presets::test_sets::VERTICAL;
        use super::*;
        let mut buffer =
            Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(
                Line::raw("Vertical Rule").centered(),
            )
            .title_bottom(
                Line::raw(" Horizontal Alignment: Center ")
                    .centered(),
            )
            .render(buffer.area, &mut buffer);
        Rule::from_set(VERTICAL)
            .vertical()
            .vertical_padding(1)
            .horizontal_alignment(Alignment::Center)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌─────────────────Vertical Rule─────────────────┐",
            "│                                               │",
            "│                       +                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       +                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       +                       │",
            "│                                               │",
            "└──────── Horizontal Alignment: Center ─────────┘",
        ]);
        assert_eq!(buffer, expected);
        buffer = Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(
                Line::raw("Vertical Rule").centered(),
            )
            .title_bottom(
                Line::raw(" Horizontal Alignment: Left ")
                    .centered(),
            )
            .render(buffer.area, &mut buffer);
        Rule::from_set(VERTICAL)
            .vertical()
            .vertical_padding(1)
            .horizontal_alignment(Alignment::Left)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌─────────────────Vertical Rule─────────────────┐",
            "│                                               │",
            "│+                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "│+                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "│+                                              │",
            "│                                               │",
            "└───────── Horizontal Alignment: Left ──────────┘",
        ]);
        assert_eq!(buffer, expected);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌─────────────────Vertical Rule─────────────────┐",
            "│                                               │",
            "│                                              +│",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              +│",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              +│",
            "│                                               │",
            "└───────── Horizontal Alignment: Right ─────────┘",
        ]);
        buffer = Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(
                Line::raw("Vertical Rule").centered(),
            )
            .title_bottom(
                Line::raw(" Horizontal Alignment: Right ")
                    .centered(),
            )
            .render(buffer.area, &mut buffer);
        Rule::new()
            .with_set(VERTICAL)
            .vertical()
            .vertical_padding(1)
            .main_symbol('│')
            .horizontal_alignment(Alignment::Right)
            .render(buffer.area, &mut buffer);
        assert_eq!(buffer, expected);
    }
}
pub mod macros {
    #[cfg(feature = "utils")]
    #[macro_export]
    macro_rules! gen_main {
        () => {
            fn main() -> io::Result<()> {
                let mut terminal = ratatui::init();
                let app_result = run(&mut terminal);
                ratatui::restore();
                app_result
            }
        };
    }
    #[cfg(feature = "utils")]
    #[macro_export]
    macro_rules! gen_example_code {
        ($fun:item) => {
            tui_rule::gen_use!();
            tui_rule::gen_run!($fun);
            tui_rule::gen_main!();
        };
    }
    #[cfg(feature = "utils")]
    #[macro_export]
    macro_rules! gen_run {
        ($fun:item) => {
            $fun
        };
    }
    #[cfg(feature = "utils")]
    #[macro_export]
    macro_rules! gen_use {
        () => {
            use colorgrad::Gradient;
            use crossterm::event::{
                self, Event, KeyCode, KeyEvent,
                KeyEventKind,
            };
            use ratatui::{
                buffer::Buffer,
                layout::Rect,
                prelude::{Alignment, Color, Style},
                text::Line,
                widgets::{Block, Widget},
                DefaultTerminal, Frame,
            };
            use std::{io, rc::Rc};
            use tui_rule::*;
        };
    }
    #[macro_export]
    macro_rules! create_raw_spans {
        ($string:expr) => {
            $string
                .chars()
                .map(String::from)
                .map(ratatui::text::Span::from)
                .collect::<Vec<ratatui::text::Span>>()
        };
    }
}

impl WidgetRef for Rule {
    fn render_ref(
        &self,
        mut area_old: Rect,
        buf: &mut Buffer,
    ) {
        let (p_l, p_r, p_t, p_b) = (
            self.padding.left,
            self.padding.right,
            self.padding.top,
            self.padding.bottom,
        );
        if self.orientation == Orientation::Horizontal {
            area_old.y = match self.vertical_alignment {
                VerticalAlignment::Top => area_old
                    .y
                    .saturating_sub(p_b)
                    .saturating_add(p_t),
                VerticalAlignment::Center => {
                    (area_old.bottom() / 2)
                        .saturating_sub(1 + p_b)
                        .saturating_add(p_t)
                }
                VerticalAlignment::Bottom => area_old
                    .bottom()
                    .saturating_sub(
                        1 + p_b
                            + self.area_margin.vertical * 2,
                    )
                    .saturating_add(p_t),
            }
            .saturating_sub(self.extra_rep_1 as u16);
        };
        if self.orientation == Orientation::Vertical {
            area_old.x = match self.horizontal_alignment {
                Alignment::Left => area_old
                    .x
                    .saturating_sub(p_r)
                    .saturating_add(p_l),
                Alignment::Center => (area_old.right() / 2)
                    .saturating_sub(1 + p_r)
                    .saturating_add(p_l),

                Alignment::Right => {
                    area_old.right().saturating_sub(
                        1 + p_r
                            + self.area_margin.horizontal
                                * 2,
                    )
                }
            }
            .saturating_sub(self.extra_rep_1 as u16);
        };

        let area = area_old.inner(self.area_margin);

        let ln = create_segment!(
            self.symbol_set,
            match self.orientation {
                Orientation::Vertical => p_t,
                Orientation::Horizontal => p_l,
            } as usize,
            match self.orientation {
                Orientation::Vertical => p_b,
                Orientation::Horizontal => p_r,
            } as usize,
            match self.orientation {
                Orientation::Horizontal =>
                    area.width as f32,
                Orientation::Vertical => area.height as f32,
            },
            self.orientation,
            self.horizontal_alignment,
            self.vertical_alignment,
            self.extra_rep_1,
            self.extra_rep_2
        );

        let ln = if let Some(boxed) = &self.gradient {
            match self.bg {
                Bg::None => Line::from(
                    generate_gradient_text!(ln, boxed),
                ),
                _ => Line::from(generate_gradient_text!(
                    ln, boxed, &self.bg
                )),
            }
        } else {
            Line::from(crate::create_raw_spans!(ln))
        };
        match self.orientation {
            Orientation::Horizontal => {
                buf.set_line(
                    area.x,
                    area.y,
                    &ln,
                    ln.spans.len() as u16 + 1,
                );
            }
            Orientation::Vertical => {
                for (y_n, s) in ln.iter().enumerate() {
                    buf.set_span(
                        area.x,
                        area.y + y_n as u16,
                        s,
                        1,
                    );
                }
            }
        }
    }
}
