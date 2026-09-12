#!/usr/bin/env python3
"""Keep every version of the manual on the website.

The bundle export writes one manual (index.html, docs.css, blockst.pdf);
the site keeps one such folder per package version and a switcher in each:

    site/
      index.html        -> redirects to the newest version
      latest/index.html -> the same, for links that should not go stale
      versions.json     -> {"latest": "0.4.0", "versions": ["0.4.0", "0.3.0"]}
      0.4.0/            -> the manual of 0.4.0
      0.3.0/            -> the manual of 0.3.0

    manual-site.py --site site --version 0.4.0 --build public

copies `public` to `site/0.4.0` (replacing what was there) and rewrites the
index, the redirect, the manifest and the version bar of every manual.
Without --build only the index files and bars are refreshed. The site
folder is the `gh-pages` branch in CI; run it on a checkout of that branch
to reproduce the website locally.
"""

from __future__ import annotations

import argparse
import json
import re
import shutil
import sys
from pathlib import Path

VERSION_RE = re.compile(r"^\d+\.\d+\.\d+$")
BAR_START = "<!-- handbuch-versionen -->"
BAR_END = "<!-- /handbuch-versionen -->"


def semver(version: str) -> tuple[int, ...]:
    return tuple(int(part) for part in version.split("."))


def list_versions(site: Path) -> list[str]:
    found = [p.name for p in site.iterdir() if p.is_dir() and VERSION_RE.match(p.name) and (p / "index.html").exists()]
    return sorted(found, key=semver, reverse=True)


def install_build(site: Path, version: str, build: Path) -> None:
    for name in ("index.html", "docs.css", "blockst.pdf"):
        if not (build / name).exists():
            sys.exit(f"manual-site: {build / name} is missing — is this the bundle export?")
    target = site / version
    if target.exists():
        shutil.rmtree(target)
    shutil.copytree(build, target)


def version_bar(version: str, versions: list[str]) -> str:
    """The strip above the manual's header: which version this is, a menu
    of all of them, and a pointer to the newest when this is an older one.
    Styled with the stylesheet's own variables so light and dark follow."""
    latest = versions[0]
    options = "".join(
        f'<option value="../{v}/"{" selected" if v == version else ""}>{v}{" (aktuell)" if v == latest else ""}</option>'
        for v in versions
    )
    notice = (
        f' <span class="hv-hinweis">Dies ist eine ältere Version — <a href="../{latest}/">zur aktuellen Version {latest}</a></span>'
        if version != latest
        else ""
    )
    return (
        f"{BAR_START}"
        "<style>"
        ".handbuch-versionen{display:flex;flex-wrap:wrap;gap:.4rem 1rem;align-items:center;"
        "padding:.45rem 1.25rem;font-family:var(--sans,sans-serif);font-size:.85rem;"
        "background:var(--flaeche,#f4f6f8);border-bottom:1px solid var(--linie,#d5dbe1);color:var(--muted,#5b6670)}"
        ".handbuch-versionen select{font:inherit;color:var(--ink,#16191d);background:var(--grund,#fff);"
        "border:1px solid var(--linie,#d5dbe1);border-radius:.3rem;padding:.1rem .4rem}"
        ".handbuch-versionen a{color:var(--accent,#14537f)}"
        ".handbuch-versionen .hv-hinweis{margin-left:auto}"
        "</style>"
        '<div class="handbuch-versionen">'
        '<label>Handbuch-Version <select onchange="location.href=this.value" aria-label="Version wählen">'
        f"{options}</select></label>"
        '<a href="blockst.pdf">PDF</a>'
        f"{notice}"
        "</div>"
        f"{BAR_END}"
    )


def inject_bar(page: Path, version: str, versions: list[str]) -> None:
    html = page.read_text(encoding="utf-8")
    start, end = html.find(BAR_START), html.find(BAR_END)
    if start != -1 and end != -1:
        html = html[:start] + html[end + len(BAR_END):]
    body = re.search(r"<body[^>]*>", html)
    if not body:
        sys.exit(f"manual-site: no <body> in {page}")
    html = html[: body.end()] + version_bar(version, versions) + html[body.end():]
    page.write_text(html, encoding="utf-8")


def redirect_page(target: str, versions: list[str], title: str, prefix: str = "") -> str:
    items = "".join(f'<li><a href="{prefix}{v}/">blockst {v}</a></li>' for v in versions)
    return (
        '<!DOCTYPE html><html lang="de"><head><meta charset="utf-8">'
        f'<meta http-equiv="refresh" content="0; url={target}">'
        f"<title>{title}</title>"
        '<link rel="canonical" href="' + target + '">'
        '<style>body{font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,"Helvetica Neue",Arial,sans-serif;'
        "margin:3rem auto;max-width:40rem;padding:0 1rem;line-height:1.6}</style></head>"
        f'<body><h1>blockst — Handbuch</h1><p>Weiter zur <a href="{target}">aktuellen Version</a> …</p>'
        f"<p>Alle Versionen:</p><ul>{items}</ul></body></html>\n"
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--site", required=True, type=Path, help="the website folder (the gh-pages checkout)")
    parser.add_argument("--version", help="package version the build belongs to (typst.toml's version)")
    parser.add_argument("--build", type=Path, help="bundle export folder to install as that version")
    args = parser.parse_args()

    if args.build and not args.version:
        parser.error("--build needs --version")
    if args.version and not VERSION_RE.match(args.version):
        parser.error(f"not a version: {args.version}")

    args.site.mkdir(parents=True, exist_ok=True)
    if args.build:
        install_build(args.site, args.version, args.build)

    versions = list_versions(args.site)
    if not versions:
        sys.exit("manual-site: no version folder in the site")
    latest = versions[0]

    for version in versions:
        inject_bar(args.site / version / "index.html", version, versions)

    (args.site / "versions.json").write_text(json.dumps({"latest": latest, "versions": versions}, indent=2) + "\n", encoding="utf-8")
    (args.site / "index.html").write_text(redirect_page(f"{latest}/", versions, "blockst — Handbuch"), encoding="utf-8")
    # `latest/` sits one level down, so its links need the parent.
    (args.site / "latest").mkdir(exist_ok=True)
    (args.site / "latest" / "index.html").write_text(
        redirect_page(f"../{latest}/", versions, f"blockst {latest} — Handbuch", prefix="../"), encoding="utf-8"
    )
    (args.site / ".nojekyll").touch()

    print(f"site: {len(versions)} versions, latest {latest}: {', '.join(versions)}")


if __name__ == "__main__":
    main()
