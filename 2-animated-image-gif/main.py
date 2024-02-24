from PIL import Image
import os

IMAGE_PATH = "./static/screaming-cat.gif"

if not os.path.exists("./static/extracted_frames"):
    os.mkdir("./static/extracted_frames")

if __name__ == "__main__":
    gif_image = Image.open(IMAGE_PATH)
    print(f"Jumlah frame: {gif_image.n_frames}")
    for i in range(gif_image.n_frames):
        gif_image.seek(i)
        frame = gif_image.convert("L")
        frame.save(f"./static/extracted_frames/frame_{i}.png")
    frames = [Image.open(f"./static/extracted_frames/frame_{i}.png") for i in range(gif_image.n_frames)]
    frames[0].save("./static/new_gif.gif", save_all=True, append_images=frames[1:], loop=0, duration=gif_image.info["duration"])