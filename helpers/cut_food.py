from PIL import Image
import os
import glob

DIR = os.path.join(os.path.dirname(os.path.realpath(
    __file__)), "../assets/images/rooms/prop/food")

# https://gist.github.com/odyniec/3470977


def autocrop_image(image, border=0):
    # Get the bounding box
    bbox = image.getbbox()
    print("bbox", bbox)

    # Crop the image to the contents of the bounding box
    image = image.crop(bbox)

    # Determine the width and height of the cropped image
    (width, height) = image.size
    print("size", image.size)

    # Add border
    width += border * 2
    height += border * 2

    # Create a new image object for the output image
    cropped_image = Image.new("RGBA", (width, height), (0, 0, 0, 0))

    # Paste the cropped image onto the new image
    cropped_image.paste(image, (border, border))

    # Done!
    return cropped_image


def cut(file: str) -> Image.Image:
    image = Image.open(file).convert("RGBA")
    filename = os.path.basename(file)
    name, ext = os.path.splitext(filename)
    cut_image = autocrop_image(image)
    cut_image.save(os.path.join(DIR, f"{name}_thought{ext}"))
    return cut_image


def main():
    biggest = (0, 0)
    for file in glob.glob(f"{DIR}/*.png"):
        filename = os.path.basename(file)
        name, ext = os.path.splitext(filename)
        if not name.endswith("_thought"):
            print(f"Cutting {filename}...")
            img = cut(file)
            if img.size[0] > biggest[0]:
                biggest = (img.size[0], biggest[1])
            if img.size[1] > biggest[1]:
                biggest = (biggest[0], img.size[1])
    print("biggest:", biggest)


if __name__ == "__main__":
    main()
