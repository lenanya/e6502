import os
from PIL import Image

folder_path = './ba_frames_jpg'
output_file = './ba_frames'

def bits_to_bytes(bits):
    b = 0
    for bi_i in range(8):
        b |= bits[bi_i] << 7 - bi_i
    return b

for filename in os.listdir(folder_path):
    if filename.endswith(".jpg"):
        file_path = os.path.join(folder_path, filename)


        rows = []

        with Image.open(file_path) as img:
            for y in range(8):
                rows.append([])
                for x in range(8):
                    p = img.getpixel((x, y))
                    if sum(p) >= 128 * 3:
                        rows[y].append(1)
                    else:
                        rows[y].append(0)
        bits_rows = []
        for row in range(8):
            bits_rows.append(bits_to_bytes(rows[row]))
        
        wow = bytes(bits_rows)

        with open(output_file, '+ab') as f:
            f.write(wow)
        
print("done")