from pathlib import Path
import re

REPO = "https://github.com/AlbertRG99/rust-tutorial/blob/main"

patron = re.compile(r'!\[\]\(\.?/?media/([^)]+)\)')

for md in Path(".").rglob("*.md"):
    contenido = md.read_text(encoding="utf-8")

    nuevo = patron.sub(
        lambda m: f"![]({REPO}/media/{m.group(1)}?raw=true)",
        contenido
    )

    if nuevo != contenido:
        md.write_text(nuevo, encoding="utf-8")
        print(f"✔ Modificado: {md}")

print("Conversión terminada.")