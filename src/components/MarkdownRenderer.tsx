import { createMemo } from "solid-js";
import { marked } from "marked";
import DOMPurify from "dompurify";

interface Props {
  content: string;
}

marked.setOptions({
  breaks: true,
  gfm: true,
});

export default function MarkdownRenderer(props: Props) {
  const html = createMemo(() => {
    const raw = marked.parse(props.content, { async: false }) as string;
    return DOMPurify.sanitize(raw, {
      ALLOWED_TAGS: [
        "p", "br", "b", "i", "em", "strong", "a", "ul", "ol", "li",
        "h1", "h2", "h3", "h4", "h5", "h6", "code", "pre", "blockquote",
        "table", "thead", "tbody", "tr", "th", "td", "hr", "img",
        "span", "div", "del", "sup", "sub",
      ],
      ALLOWED_ATTR: ["href", "target", "rel", "src", "alt", "class"],
    });
  });

  return (
    <div
      class="prose prose-invert max-w-none prose-sm
             prose-headings:text-zinc-100 prose-headings:font-semibold
             prose-a:text-emerald-400 prose-a:no-underline hover:prose-a:underline
             prose-code:text-emerald-300 prose-code:bg-zinc-800 prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded
             prose-pre:bg-zinc-900 prose-pre:border prose-pre:border-zinc-800
             prose-strong:text-zinc-100
             prose-blockquote:border-emerald-500 prose-blockquote:text-zinc-400
             prose-ul:list-disc prose-ol:list-decimal
             prose-li:text-zinc-300
             prose-th:text-zinc-200 prose-td:text-zinc-300 prose-table:border-zinc-700"
      innerHTML={html()}
    />
  );
}
