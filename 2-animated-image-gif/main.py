from PIL import Image
import os

IMAGE_PATH = "./static/screaming-cat.gif"

if __name__ == "__main__":
    gif_image = Image.open(IMAGE_PATH)

    print(f"Image format: {gif_image.format}")
    print(f"Image mode: {gif_image.mode}")
    print(f"Is animated: {gif_image.is_animated}")
    print(f"Number of frames: {gif_image.n_frames}")
    print(f"Duration: {gif_image.info['duration']}")
    print(f"Loop: {gif_image.info['loop']}")
    print(f"Extension: {gif_image.info['extension']}")
    print(f"Size: {gif_image.size}")


    for frame in range(gif_image.n_frames):
        gif_image.seek(frame)
        try:
            gif_image.save(f"./output/frame_{frame}.png")
        except:
            os.mkdir("./output")
            gif_image.save(f"./output/frame_{frame}.png")