//! Custom KaTeX behaviors.

use crate::js_engine::JsValue;
use derive_builder::Builder;
use std::collections::HashMap;

/// Options to be passed to KaTeX.
///
/// Read <https://katex.org/docs/options.html> for more information.
#[non_exhaustive]
#[derive(Clone, Builder, Debug, Default)]
#[builder(default)]
#[builder(setter(into, strip_option))]
pub struct Opts {
    /// Whether to render the math in the display mode.
    display_mode: Option<bool>,
    /// KaTeX output type.
    output_type: Option<OutputType>,
    /// Whether to have `\tags` rendered on the left instead of the right.
    leqno: Option<bool>,
    /// Whether to make display math flush left.
    fleqn: Option<bool>,
    /// Whether to let KaTeX throw a ParseError for invalid LaTeX.
    throw_on_error: Option<bool>,
    /// Color used for invalid LaTeX.
    error_color: Option<String>,
    /// Collection of custom macros.
    /// Read <https://katex.org/docs/options.html> for more information.
    macros: HashMap<String, String>,
    /// Specifies a minimum thickness, in ems.
    /// Read <https://katex.org/docs/options.html> for more information.
    min_rule_thickness: Option<f64>,
    /// Max size for user-specified sizes.
    /// If set to `None`, users can make elements and spaces arbitrarily large.
    /// Read <https://katex.org/docs/options.html> for more information.
    #[allow(clippy::option_option)]
    max_size: Option<Option<f64>>,
    /// Limit the number of macro expansions to the specified number.
    /// If set to `None`, the macro expander will try to fully expand as in LaTeX.
    /// Read <https://katex.org/docs/options.html> for more information.
    #[allow(clippy::option_option)]
    max_expand: Option<Option<i32>>,
    /// Whether to trust users' input.
    /// Read <https://katex.org/docs/options.html> for more information.
    trust: Option<bool>,
}

impl Opts {
    /// Return [`OptsBuilder`].
    pub fn builder() -> OptsBuilder {
        OptsBuilder::default()
    }

    /// Set whether to render the math in the display mode.
    pub fn set_display_mode(&mut self, flag: bool) {
        self.display_mode = Some(flag);
    }

    /// Set KaTeX output type.
    pub fn set_output_type(&mut self, output_type: OutputType) {
        self.output_type = Some(output_type);
    }

    /// Set whether to have `\tags` rendered on the left instead of the right.
    pub fn set_leqno(&mut self, flag: bool) {
        self.leqno = Some(flag);
    }

    /// Set whether to make display math flush left.
    pub fn set_fleqn(&mut self, flag: bool) {
        self.fleqn = Some(flag);
    }

    /// Set whether to let KaTeX throw a ParseError for invalid LaTeX.
    pub fn set_throw_on_error(&mut self, flag: bool) {
        self.throw_on_error = Some(flag);
    }

    /// Set the color used for invalid LaTeX.
    pub fn set_error_color(&mut self, color: String) {
        self.error_color = Some(color);
    }

    /// Add a custom macro.
    /// Read <https://katex.org/docs/options.html> for more information.
    pub fn add_macro(&mut self, entry_name: String, entry_data: String) {
        self.macros.insert(entry_name, entry_data);
    }

    /// Set the minimum thickness, in ems.
    /// Read <https://katex.org/docs/options.html> for more information.
    pub fn set_min_rule_thickness(&mut self, value: f64) {
        self.min_rule_thickness = Some(value);
    }

    /// Set the max size for user-specified sizes.
    /// If set to `None`, users can make elements and spaces arbitrarily large.
    /// Read <https://katex.org/docs/options.html> for more information.
    pub fn set_max_size(&mut self, value: Option<f64>) {
        self.max_size = Some(value);
    }

    /// Set the limit for the number of macro expansions.
    /// If set to `None`, the macro expander will try to fully expand as in LaTeX.
    /// Read <https://katex.org/docs/options.html> for more information.
    pub fn set_max_expand(&mut self, value: Option<i32>) {
        self.max_expand = Some(value);
    }

    /// Set whether to trust users' input.
    /// Read <https://katex.org/docs/options.html> for more information.
    pub fn set_trust(&mut self, flag: bool) {
        self.trust = Some(flag);
    }

    pub(crate) fn to_js_value<T: JsValue>(&self) -> T {
        let mut opt: HashMap<String, T> = HashMap::new();
        if let Some(display_mode) = self.display_mode {
            opt.insert("displayMode".to_owned(), T::from_bool(display_mode));
        }
        if let Some(output_type) = self.output_type {
            opt.insert(
                "output".to_owned(),
                T::from_string(
                    match output_type {
                        OutputType::Html => "html",
                        OutputType::Mathml => "mathml",
                        OutputType::HtmlAndMathml => "htmlAndMathml",
                    }
                    .to_owned(),
                ),
            );
        }
        if let Some(leqno) = self.leqno {
            opt.insert("leqno".to_owned(), T::from_bool(leqno));
        }
        if let Some(fleqn) = self.fleqn {
            opt.insert("fleqn".to_owned(), T::from_bool(fleqn));
        }
        if let Some(throw_on_error) = self.throw_on_error {
            opt.insert("throwOnError".to_owned(), T::from_bool(throw_on_error));
        }
        if let Some(error_color) = &self.error_color {
            opt.insert("errorColor".to_owned(), T::from_string(error_color.clone()));
        }
        if !self.macros.is_empty() {
            opt.insert(
                "macros".to_owned(),
                T::from_object(
                    self.macros
                        .iter()
                        .map(|(k, v)| (k.clone(), T::from_string(v.clone()))),
                ),
            );
        }
        if let Some(min_rule_thickness) = self.min_rule_thickness {
            opt.insert(
                "minRuleThickness".to_owned(),
                T::from_float(min_rule_thickness),
            );
        }
        if let Some(Some(max_size)) = self.max_size {
            opt.insert("maxSize".to_owned(), T::from_float(max_size));
        }
        if let Some(max_expand) = self.max_expand {
            match max_expand {
                Some(max_expand) => {
                    opt.insert("maxExpand".to_owned(), T::from_int(max_expand));
                }
                None => {
                    opt.insert("maxExpand".to_owned(), T::from_int(i32::max_value()));
                }
            }
        }
        if let Some(trust) = self.trust {
            opt.insert("trust".to_owned(), T::from_bool(trust));
        }
        T::from_object(opt.into_iter())
    }
}

impl AsRef<Opts> for Opts {
    fn as_ref(&self) -> &Opts {
        self
    }
}

impl OptsBuilder {
    /// Add an entry to [`macros`](OptsBuilder::macros).
    ///
    /// # Examples
    ///
    /// ```
    /// let opts = katex::Opts::builder()
    ///     .add_macro(r#"\RR"#.to_owned(), r#"\mathbb{R}"#.to_owned())
    ///     .build()
    ///     .unwrap();
    /// let html = katex::render_with_opts(r#"\RR"#, &opts).unwrap();
    /// ```
    pub fn add_macro(mut self, entry_name: String, entry_data: String) -> Self {
        match self.macros.as_mut() {
            Some(macros) => {
                macros.insert(entry_name, entry_data);
            }
            None => {
                let mut macros = HashMap::new();
                macros.insert(entry_name, entry_data);
                self.macros = Some(macros);
            }
        }
        self
    }
}

/// Output type from KaTeX.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OutputType {
    /// Outputs KaTeX in HTML only.
    Html,
    /// Outputs KaTeX in MathML only.
    Mathml,
    /// Outputs HTML for visual rendering and includes MathML for accessibility.
    HtmlAndMathml,
}
