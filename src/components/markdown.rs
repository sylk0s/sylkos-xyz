use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};

#[derive(Props, Clone, PartialEq, Debug, Copy)]
pub struct MarkdownProps {
    pub content: &'static str,
}

#[allow(non_snake_case)]
pub fn Markdown(cx: Scope<MarkdownProps>) -> Element {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(cx.props.content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // add tailwind class to p and table
    let html_output = html_output
        .replace("<p>", "<p class=\"mb-4 text-text\">")
        .replace(
            "<table>",
            "<table class=\"mb-4 gray-400 text-text\">",
        );

    // add tailwind class to h<4-n>
    let html_output = html_output
        .replace(
            "<h1>",
            "<h1 class=\"text-4xl font-bold mb-2 text-text\">",
        )
        // .replace(
        //     "</h1>",
        //     "</h1> <hr class=\"text-text\"></hr>",
        // )
        .replace(
            "<h2>",
            "<h2 class=\"text-3xl font-bold mb-2 text-text\">",
        ).replace(
            "<h3>",
            "<h3 class=\"text-2xl font-bold mb-2 text-text\">",
        ).replace(
            "<h4>",
            "<h4 class=\"text-xl font-bold mb-2 text-text\">",
        ).replace(
            "<h5>",
            "<h5 class=\"text-lg font-bold mb-2 text-text\">",
        ).replace(
            "<h6>",
            "<h6 class=\"text-base font-bold mb-2 text-text\">",
        );

    let html_output = html_output
        .replace("<ul>", "<ul class=\"list-disc ml-4 text-text\">")
        .replace("<ol>", "<ol class=\"list-decimal ml-4 text-text\">");

    // add tailwind to code blocks
    let html_output = html_output.replace(
        "<pre>",
        "<pre class=\"mb-4 p-2 rounded border border-overlay0 bg-surface0 \">",
    );

    // add tailwind to td and th
    let html_output = html_output
        .replace("<th>", "<th class=\"bg-gray-300 dark:bg-gray-900\">")
        .replace("<td>", "<th class=\"bg-gray-100 dark:bg-gray-700\">");

    // add tailwind to links
    let html_output =
        html_output.replace("<a ", "<a class=\"text-blue\" ");

    cx.render(rsx! {
        div {
            class: "p-4",
            dangerous_inner_html: "{html_output}"
        }
    })
}