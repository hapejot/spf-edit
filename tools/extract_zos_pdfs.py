from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import re

from pypdf import PdfReader


ROOT = Path(__file__).resolve().parents[1]
SOURCE_DIR = ROOT / "docs" / "zos-docs"
OUT_DIR = SOURCE_DIR / "extracted"
INDEX_FILE = SOURCE_DIR / "EXTRACTED_INDEX.md"


@dataclass
class ExtractStats:
    pdf_name: str
    pages: int
    chars: int
    output_name: str


def normalize_text(text: str) -> str:
    # Preserve paragraph shape while removing excessive empty lines.
    text = text.replace("\r\n", "\n").replace("\r", "\n")
    text = re.sub(r"\n{3,}", "\n\n", text)
    return text.strip()


def extract_pdf(pdf_path: Path, out_dir: Path) -> ExtractStats:
    reader = PdfReader(str(pdf_path))
    page_chunks: list[str] = []

    for page_index, page in enumerate(reader.pages, start=1):
        page_text = page.extract_text() or ""
        page_text = normalize_text(page_text)
        page_chunks.append(f"## Page {page_index}\n\n{page_text}\n")

    body = "\n".join(page_chunks).strip() + "\n"
    out_name = f"{pdf_path.stem}.md"
    out_path = out_dir / out_name
    out_path.write_text(
        "\n".join(
            [
                f"# Extracted Text: {pdf_path.name}",
                "",
                "Source: IBM z/OS ISPF documentation PDF",
                f"Pages: {len(reader.pages)}",
                "",
                body,
            ]
        ),
        encoding="utf-8",
    )

    return ExtractStats(
        pdf_name=pdf_path.name,
        pages=len(reader.pages),
        chars=len(body),
        output_name=out_name,
    )


def write_index(stats: list[ExtractStats], out_file: Path) -> None:
    lines = [
        "# z/OS ISPF PDF Extraction Index",
        "",
        "This folder contains markdown text extracted from IBM PDF manuals in docs/zos-docs.",
        "",
        "| PDF | Pages | Extracted Characters | Output |",
        "|---|---:|---:|---|",
    ]

    for s in sorted(stats, key=lambda item: item.pdf_name.lower()):
        lines.append(
            f"| {s.pdf_name} | {s.pages} | {s.chars} | extracted/{s.output_name} |"
        )

    lines.extend(
        [
            "",
            "Notes:",
            "- Extraction quality depends on the text layer in each PDF.",
            "- Scanned image-only pages may produce sparse text without OCR.",
        ]
    )

    out_file.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> None:
    if not SOURCE_DIR.exists():
        raise SystemExit(f"Source directory not found: {SOURCE_DIR}")

    OUT_DIR.mkdir(parents=True, exist_ok=True)

    pdfs = sorted(SOURCE_DIR.glob("*.pdf"))
    if not pdfs:
        raise SystemExit(f"No PDFs found in: {SOURCE_DIR}")

    stats = [extract_pdf(pdf, OUT_DIR) for pdf in pdfs]
    write_index(stats, INDEX_FILE)

    print(f"Extracted {len(stats)} PDF files to: {OUT_DIR}")
    for s in stats:
        print(f"- {s.pdf_name}: {s.pages} pages, {s.chars} chars -> {s.output_name}")
    print(f"Wrote index: {INDEX_FILE}")


if __name__ == "__main__":
    main()
