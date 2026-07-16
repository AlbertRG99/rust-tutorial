from pathlib import Path
from urllib.parse import quote
import re

REPO = "https://github.com/AlbertRG99/rust-tutorial/blob/main"

patron = re.compile(r'!\[\[(.*?)\]\]')

for md in Path(".").rglob("*.md"):
    texto = md.read_text(encoding="utf-8")

    def reemplazar(match):
        ruta = match.group(1).strip()

        # Si la imagen no está en media/, añadirlo
        if not ruta.startswith("media/"):
            ruta = "media/" + ruta

        # Codificar espacios
        ruta = quote(ruta)

        return f"![]({REPO}/{ruta}?raw=true)"

    nuevo = patron.sub(reemplazar, texto)

    if nuevo != texto:
        md.write_text(nuevo, encoding="utf-8")
        print(f"✔ {md}")