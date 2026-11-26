"""Ubah semua .tga menjadi .png sekaligus (Weft hanya membaca PNG).

Pemakaian:
    python convert_tga.py FOLDER            -> .png dibuat di samping tiap .tga
    python convert_tga.py FOLDER --hapus    -> .tga dihapus setelah dikonversi
    python convert_tga.py FOLDER --mcv      -> sekaligus ganti nama gaya Bedrock ke nama blok MCV
                                               (double_plant_grass_bottom -> tall_grass_bottom, dst.)

Butuh Pillow:  pip install pillow
Folder diperiksa sampai ke dalam (rekursif). File .png yang sudah ada ditimpa.
Nama file dirapikan ke huruf kecil dan "_" supaya cocok dengan nama blok Weft.
"""
import os
import re
import sys

from PIL import Image


# Nama Bedrock -> nama tekstur MCV (hanya dipakai dengan --mcv).
BEDROCK_TO_MCV = {
    "double_plant_grass_bottom": "tall_grass_bottom",
    "double_plant_grass_top": "tall_grass_top",
    "double_plant_fern_bottom": "large_fern_bottom",
    "double_plant_fern_top": "large_fern_top",
    "tallgrass": "short_grass",
    "deadbush": "dead_bush",
}


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    delete = "--hapus" in sys.argv
    rename = "--mcv" in sys.argv
    if not args:
        print(__doc__)
        return 1
    root = args[0]
    done = skipped = 0
    for folder, _, files in os.walk(root):
        for name in files:
            if not name.lower().endswith(".tga"):
                continue
            src = os.path.join(folder, name)
            base = re.sub(r"[^a-z0-9_]+", "_", name[:-4].lower()).strip("_")
            if rename:
                base = BEDROCK_TO_MCV.get(base, base)
            dst = os.path.join(folder, base + ".png")
            try:
                with Image.open(src) as im:
                    im.convert("RGBA").save(dst, format="PNG")
            except Exception as e:  # file rusak: lanjut ke berikutnya
                print(f"GAGAL  {src}: {e}")
                skipped += 1
                continue
            if delete:
                os.remove(src)
            done += 1
    print(f"selesai: {done} file diubah, {skipped} gagal")
    return 0


if __name__ == "__main__":
    sys.exit(main())
