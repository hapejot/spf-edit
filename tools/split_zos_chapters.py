from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
import re
import unicodedata


ROOT = Path(__file__).resolve().parents[1]
EXTRACTED_DIR = ROOT / "docs" / "zos-docs" / "extracted"
OUT_ROOT = ROOT / "docs" / "zos-docs" / "chapters"


PAGE_RE = re.compile(r"^## Page (\d+)\s*$", re.MULTILINE)
SECTION_RE = re.compile(
    r"^(?P<kind>Chapter|Appendix)\s+(?P<num>[0-9A-Z]+)\.\s+(?P<title>.+)$",
    re.IGNORECASE,
)


@dataclass
class Page:
    number: int
    content: str


@dataclass
class Section:
    title: str
    slug: str
    start_page: int
    pages: list[Page] = field(default_factory=list)


def slugify(value: str) -> str:
    value = unicodedata.normalize("NFKD", value)
    value = value.encode("ascii", "ignore").decode("ascii")
    value = value.lower()
    value = re.sub(r"[^a-z0-9]+", "-", value).strip("-")
    return value or "section"


def split_pages(doc_text: str) -> list[Page]:
    matches = list(PAGE_RE.finditer(doc_text))
    pages: list[Page] = []
    for i, match in enumerate(matches):
        start = match.end()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(doc_text)
        page_no = int(match.group(1))
        content = doc_text[start:end].strip()
        pages.append(Page(number=page_no, content=content))
    return pages


def section_key(heading: str) -> str:
    m = SECTION_RE.match(heading)
    if not m:
        return heading.lower()
    return f"{m.group('kind').lower()}-{m.group('num').upper()}"


def detect_section_heading(page: Page) -> str | None:
    # Examine the first lines on the page where real section headers usually appear.
    lines = [line.replace("\u00a0", " ").strip() for line in page.content.splitlines()[:40]]
    for i, raw_line in enumerate(lines):
        line = raw_line.replace("\u00a0", " ").strip()
        if not line:
            continue
        if "on page" in line.lower() or re.search(r"\.{3,}", line):
            continue

        line = re.sub(r"\s+\d+\s*$", "", line).strip()
        m = SECTION_RE.match(line)
        if not m:
            continue

        kind = m.group("kind").capitalize()
        num = m.group("num").upper()
        title = m.group("title").strip()

        # Some PDF lines wrap chapter headings across two lines.
        if i + 1 < len(lines):
            next_line = lines[i + 1]
            if next_line and not re.search(r"\.{3,}", next_line):
                if title.endswith(("and", "or", "of", "the")):
                    title = f"{title} {next_line}".strip()

        if len(title) < 3:
            continue
        return f"{kind} {num}. {title}"
    return None


def split_document(md_file: Path, out_dir: Path) -> list[Section]:
    text = md_file.read_text(encoding="utf-8")
    pages = split_pages(text)
    if not pages:
        return []

    sections: list[Section] = []
    current = Section(title="Front Matter", slug="front-matter", start_page=pages[0].number)
    current_key = "front-matter"

    for page in pages:
        heading = detect_section_heading(page)
        if heading:
            heading_slug = slugify(heading)
            heading_key = section_key(heading)
            if heading_key == current_key:
                # Keep a more complete heading title if we discover one.
                if len(heading) > len(current.title):
                    current.title = heading
                    current.slug = heading_slug
            elif heading_slug != current.slug:
                if current.pages:
                    sections.append(current)
                current = Section(title=heading, slug=heading_slug, start_page=page.number)
                current_key = heading_key
        current.pages.append(page)

    if current.pages:
        sections.append(current)

    out_dir.mkdir(parents=True, exist_ok=True)
    for old_md in out_dir.glob("*.md"):
        old_md.unlink()

    for idx, section in enumerate(sections, start=1):
        filename = f"{idx:02d}_{section.slug}.md"
        path = out_dir / filename
        page_blocks = []
        for p in section.pages:
            page_blocks.append(f"## Page {p.number}\n\n{p.content}\n")
        path.write_text(
            "\n".join(
                [
                    f"# {section.title}",
                    "",
                    f"Source file: {md_file.name}",
                    f"Start page: {section.start_page}",
                    f"Page span: {section.pages[0].number}-{section.pages[-1].number}",
                    "",
                    "\n".join(page_blocks).strip(),
                    "",
                ]
            ),
            encoding="utf-8",
        )

    index_lines = [
        f"# Chapter Index: {md_file.stem}",
        "",
        f"Source: ../../extracted/{md_file.name}",
        "",
        "| Order | Section | Start Page | File |",
        "|---:|---|---:|---|",
    ]
    for idx, section in enumerate(sections, start=1):
        filename = f"{idx:02d}_{section.slug}.md"
        index_lines.append(
            f"| {idx} | {section.title} | {section.start_page} | {filename} |"
        )

    (out_dir / "CHAPTER_INDEX.md").write_text("\n".join(index_lines) + "\n", encoding="utf-8")

    return sections


def main() -> None:
    if not EXTRACTED_DIR.exists():
        raise SystemExit(f"Missing extracted directory: {EXTRACTED_DIR}")

    md_files = sorted(EXTRACTED_DIR.glob("*.md"))
    if not md_files:
        raise SystemExit(f"No extracted markdown files found in: {EXTRACTED_DIR}")

    OUT_ROOT.mkdir(parents=True, exist_ok=True)
    for md in md_files:
        out_dir = OUT_ROOT / md.stem
        sections = split_document(md, out_dir)
        print(f"{md.name}: {len(sections)} sections -> {out_dir}")


if __name__ == "__main__":
    main()
