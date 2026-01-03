import cv2 
import os 

video = "bad_apple8_12fps.mp4"
output_dir = "ba_frames_jpg"

c = cv2.VideoCapture(video)
fc = 0

while True:
    r, f = c.read()
    if not r:
        break
    f_name = f"frame_{fc:04d}.jpg"
    cv2.imwrite(os.path.join(output_dir, f_name), f)

    fc += 1

c.release()
print("done")