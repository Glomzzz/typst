use typst_library::layout::{Abs, Axes};
use typst_library::model::Destination;

use crate::SVGRenderer;

const SVG_FOOTNOTE_REF_PREFIX: &str = "#footnote-ref:";

impl SVGRenderer {
    /// Render an image element.
    pub(super) fn render_link(
        &mut self,
        cur_pos: (f64, f64),
        last_pos: (f64, f64),
        dest: &Destination,
        size: &Axes<Abs>,
    ) {
        let mut url = match dest {
            Destination::Url(url) => url.to_string(),
            Destination::Position(_) => return,
            Destination::Location(_) => return,
        };
        let footnote = url.starts_with(SVG_FOOTNOTE_REF_PREFIX);
        let (x, y) = if footnote { last_pos } else { cur_pos };
        self.xml.start_element("g");
        self.xml
            .write_attribute_fmt("transform", format_args!("translate({x} {y})"));
        let id = url.strip_prefix(SVG_FOOTNOTE_REF_PREFIX).map(|it| it.to_string());
        if footnote {
            self.xml.start_element("g");
            self.xml.write_attribute("transform", "scale(1, -0.5)");
            url = format!("#{}", id.as_ref().unwrap());
        }

        self.xml.start_element("a");
        if footnote {
            self.xml.write_attribute("class", "svg-footnote-reference");
            self.xml
                .write_attribute("id", &format!("{}-back", id.as_ref().unwrap()));
        }
        self.xml.write_attribute("href", &url);
        self.xml.write_attribute("width", &size.x.to_pt());
        self.xml.write_attribute("height", &size.y.to_pt());
        self.xml.start_element("rect");
        if footnote {
            self.xml.write_attribute("class", "svg-link target-highlight");
            self.xml.write_attribute("x", &-0.5);
            self.xml.write_attribute("y", &-0.5);
            self.xml.write_attribute("width", &(size.x.to_pt() + 1.0) );
            self.xml.write_attribute("height", &(size.y.to_pt() + 1.0));
        } else {
            self.xml.write_attribute("class", "svg-link");
            self.xml.write_attribute("width", &size.x.to_pt());
            self.xml.write_attribute("height", &size.y.to_pt());
        }
        self.xml.write_attribute("stroke", "none");
        self.xml.write_attribute("fill-rule", "evenodd");
        self.xml.write_attribute("fill-opacity", "0");
        self.xml.end_element();
        if footnote {
            self.xml.end_element();
        }
        self.xml.end_element();
    }
}
