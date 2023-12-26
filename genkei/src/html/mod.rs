use crate::{tag_def, tag_def_custom};
mod attributes;
mod define;
mod renderer;
mod tag;

#[cfg(feature = "deprecated")]
pub mod deprecated;
#[cfg(feature = "htmx")]
pub mod htmx;
#[cfg(feature = "macros")]
pub mod macros;

pub use attributes::HtmlAttribute;
#[cfg(feature = "deprecated")]
pub use deprecated::*;
#[cfg(feature = "macros")]
pub use macros::*;
pub use renderer::RenderError;
pub use renderer::RenderResult;
pub use renderer::Renderer;
pub use tag::Tag;

// Root element
tag_def!(html);

// Document metadata
tag_def_custom!(base, true);
tag_def!(head);
tag_def_custom!(link, true);
tag_def_custom!(meta, true);
tag_def!(style);
tag_def!(title);

// Sectioning root
tag_def!(body);

// Content sectioning
tag_def!(address);
tag_def!(article);
tag_def!(aside);
tag_def!(footer);
tag_def!(header);
tag_def!(h1);
tag_def!(h2);
tag_def!(h3);
tag_def!(h4);
tag_def!(h5);
tag_def!(h6);
tag_def!(hgroup);
tag_def!(main);
tag_def!(nav);
tag_def!(section);
tag_def!(search);

// Text content
tag_def!(blockquote);
tag_def!(dd);
tag_def!(div);
tag_def!(dl);
tag_def!(dt);
tag_def!(figcaption);
tag_def!(figure);
tag_def_custom!(hr, true);
tag_def!(li);
tag_def!(menu);
tag_def!(ol);
tag_def!(p);
tag_def!(pre);
tag_def!(ul);

// Inline text semantics
tag_def!(a);
tag_def!(abbr);
tag_def!(b);
tag_def!(bdi);
tag_def!(bdo);
tag_def_custom!(br, true);
tag_def!(cite);
tag_def!(code);
tag_def!(data);
tag_def!(dfn);
tag_def!(em);
tag_def!(i);
tag_def!(kbd);
tag_def!(mark);
tag_def!(q);
tag_def!(rp);
tag_def!(rt);
tag_def!(ruby);
tag_def!(s);
tag_def!(samp);
tag_def!(small);
tag_def!(span);
tag_def!(strong);
tag_def!(sub);
tag_def!(sup);
tag_def!(time);
tag_def!(u);
tag_def!(var);
tag_def_custom!(wbr, true);

// Image and multimedia
tag_def_custom!(area, true);
tag_def!(audio);
tag_def_custom!(img, true);
tag_def!(map);
tag_def!(track);
tag_def!(video);

// Embedded content
tag_def_custom!(embed, true);
tag_def!(iframe);
tag_def!(object);
tag_def!(picture);
tag_def!(portal);
tag_def_custom!(source, true);

// SVG and MathML
tag_def!(svg);
tag_def!(math);

// Scripting
tag_def!(canvas);
tag_def!(noscript);
tag_def!(script);

// Demarcating edits
tag_def!(del);
tag_def!(ins);

// Table content
tag_def!(caption);
tag_def_custom!(col, true);
tag_def!(colgroup);
tag_def!(table);
tag_def!(tbody);
tag_def!(td);
tag_def!(tfoot);
tag_def!(th);
tag_def!(thead);
tag_def!(tr);

// Forms
tag_def!(button);
tag_def!(datalist);
tag_def!(fieldset);
tag_def!(form);
tag_def!(label);
tag_def_custom!(input, true);
tag_def!(legend);
tag_def!(meter);
tag_def!(optgroup);
tag_def!(option);
tag_def!(output);
tag_def!(progress);
tag_def!(textarea);
tag_def!(select);

// Interactive elements
tag_def!(details);
tag_def!(dialog);
tag_def!(summary);

// Web Components
tag_def!(slot);
tag_def!(template);

#[cfg(feature = "deprecated")]
pub use deprecated::*;
