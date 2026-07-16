import re
from pathlib import Path

# Carpeta raíz donde buscar los .md
ROOT = Path(".")

# Busca enlaces de Obsidian del tipo ![[...]]
pattern = re.compile(r'!\[\[(.*?)\]\]')

for md_file in ROOT.rglob("*.md"):
    contenido = md_file.read_text(encoding="utf-8")

    def reemplazar(match):
        ruta = match.group(1).strip()

        # Si no empieza por ./ ni ../, añadir ./
        if not ruta.startswith(("./", "../")):
            ruta = f"./{ruta}"

        return f"![]({ruta})"

    nuevo = pattern.sub(reemplazar, contenido)

    if nuevo != contenido:
        md_file.write_text(nuevo, encoding="utf-8")
        print(f"✔ Modificado: {md_file}")

print("Conversión finalizada.")