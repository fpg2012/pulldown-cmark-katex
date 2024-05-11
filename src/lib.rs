use katex;
use pulldown_cmark::Event;

pub struct MathEventProcessor {
    display_style_opts: katex::opts::Opts,
}

impl MathEventProcessor {
    pub fn new() -> MathEventProcessor {
        let opts = katex::Opts::builder().display_mode(true).build().unwrap();
        MathEventProcessor {
            display_style_opts: opts,
        }
    }

    pub fn process_math_event<'a>(&'a self, event: Event<'a>) -> Event {
        match event {
            Event::InlineMath(math_exp) => {
                Event::InlineHtml(katex::render(&math_exp).unwrap().into())
            }
            Event::DisplayMath(math_exp) => Event::Html(
                katex::render_with_opts(&math_exp, &self.display_style_opts)
                    .unwrap()
                    .into(),
            ),
            _ => event,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::MathEventProcessor;
        use pulldown_cmark::{Options, Parser, TextMergeStream};
        let markdown_input = r#"
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.10/dist/katex.min.css" integrity="sha384-wcIxkf4k558AjM3Yz3BBFQUbk/zgIYC2R0QpeeYb+TwlBVMrlgLqwRjRtGZiK7ww" crossorigin="anonymous">

# Hello world
$$
\begin{pmatrix}
    1 & 2 \\ 
    3 & 4 \\
\end{pmatrix}
$$
Newton's Second Law: $F = ma$

[a link](http://example.com)
"#;
        let mep = MathEventProcessor::new();
        let mut options = Options::empty();
        options.insert(Options::ENABLE_MATH);
        let parser = Parser::new_ext(markdown_input, options);
        let iterator = TextMergeStream::new(parser).map(|event| mep.process_math_event(event));
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, iterator);
        println!("{}", html_output);
    }
}
