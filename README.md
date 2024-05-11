# pulldown-cmark-katex

Enable [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) (version 0.11 or later) to render LaTeX expressions to MathML. Based on xu-cheng's [katex-rs](https://github.com/xu-cheng/katex-rs)


Example:
```rs
use pulldown_cmark::{Options, Parser, TextMergeStream};
use pulldown_cmark_katex::MathEventProcessor;

fn main() {
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
```

expected output:

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.10/dist/katex.min.css" integrity="sha384-wcIxkf4k558AjM3Yz3BBFQUbk/zgIYC2R0QpeeYb+TwlBVMrlgLqwRjRtGZiK7ww" crossorigin="anonymous">
<h1>Hello world</h1>
<p><span class="katex-display"><span class="katex"><span class="katex-mathml"><math xmlns="http://www.w3.org/1998/Math/MathML" display="block"><semantics><mrow><mo fence="true">(</mo><mtable rowspacing="0.16em" columnalign="center center" columnspacing="1em"><mtr><mtd><mstyle scriptlevel="0" displaystyle="false"><mn>1</mn></mstyle></mtd><mtd><mstyle scriptlevel="0" displaystyle="false"><mn>2</mn></mstyle></mtd></mtr><mtr><mtd><mstyle scriptlevel="0" displaystyle="false"><mn>3</mn></mstyle></mtd><mtd><mstyle scriptlevel="0" displaystyle="false"><mn>4</mn></mstyle></mtd></mtr></mtable><mo fence="true">)</mo></mrow><annotation encoding="application/x-tex">
\begin{pmatrix}
    1 &amp; 2 \\ 
    3 &amp; 4 \\
\end{pmatrix}
</annotation></semantics></math></span><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:2.4em;vertical-align:-0.95em;"></span><span class="minner"><span class="mopen delimcenter" style="top:0em;"><span class="delimsizing size3">(</span></span><span class="mord"><span class="mtable"><span class="col-align-c"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.45em;"><span style="top:-3.61em;"><span class="pstrut" style="height:3em;"></span><span class="mord"><span class="mord">1</span></span></span><span style="top:-2.41em;"><span class="pstrut" style="height:3em;"></span><span class="mord"><span class="mord">3</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:0.95em;"><span></span></span></span></span></span><span class="arraycolsep" style="width:0.5em;"></span><span class="arraycolsep" style="width:0.5em;"></span><span class="col-align-c"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.45em;"><span style="top:-3.61em;"><span class="pstrut" style="height:3em;"></span><span class="mord"><span class="mord">2</span></span></span><span style="top:-2.41em;"><span class="pstrut" style="height:3em;"></span><span class="mord"><span class="mord">4</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:0.95em;"><span></span></span></span></span></span></span></span><span class="mclose delimcenter" style="top:0em;"><span class="delimsizing size3">)</span></span></span></span></span></span></span>
Newton's Second Law: <span class="katex"><span class="katex-mathml"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><mi>F</mi><mo>=</mo><mi>m</mi><mi>a</mi></mrow><annotation encoding="application/x-tex">F = ma</annotation></semantics></math></span><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:0.6833em;"></span><span class="mord mathnormal" style="margin-right:0.13889em;">F</span><span class="mspace" style="margin-right:0.2778em;"></span><span class="mrel">=</span><span class="mspace" style="margin-right:0.2778em;"></span></span><span class="base"><span class="strut" style="height:0.4306em;"></span><span class="mord mathnormal">ma</span></span></span></span></p>
<p><a href="http://example.com">a link</a></p>
```
