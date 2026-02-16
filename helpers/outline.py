from PIL import Image
import os
import numpy as np
import cv2
import glob

DIR = os.path.join(os.path.dirname(os.path.realpath(
    __file__)), "../assets/images/rooms/prop")

COLOR = (255, 255, 255, 255)

# Source - https://stackoverflow.com/a/62931912
# Posted by Seongeun  So
# Retrieved 2026-02-15, License - CC BY-SA 4.0


def stroke(origin_image, threshold, stroke_size, colors):
    img = np.array(origin_image)
    h, w, _ = img.shape
    padding = 0
    alpha = img[:, :, 3]
    rgb_img = img[:, :, 0:3]
    bigger_img = cv2.copyMakeBorder(rgb_img, padding, padding, padding, padding,
                                    cv2.BORDER_CONSTANT, value=(0, 0, 0, 0))
    alpha = cv2.copyMakeBorder(
        alpha, padding, padding, padding, padding, cv2.BORDER_CONSTANT, value=0)
    bigger_img = cv2.merge((bigger_img, alpha))
    h, w, _ = bigger_img.shape

    _, alpha_without_shadow = cv2.threshold(
        alpha, threshold, 255, cv2.THRESH_BINARY)  # threshold=0 in photoshop
    alpha_without_shadow = 255 - alpha_without_shadow
    # dist l1 : L1 , dist l2 : l2
    dist = cv2.distanceTransform(
        alpha_without_shadow, cv2.DIST_L2, cv2.DIST_MASK_3)
    stroked = change_matrix(dist, stroke_size)
    stroke_alpha = (stroked * 255).astype(np.uint8)

    stroke_b = np.full((h, w), colors[0][2], np.uint8)
    stroke_g = np.full((h, w), colors[0][1], np.uint8)
    stroke_r = np.full((h, w), colors[0][0], np.uint8)

    stroke = cv2.merge((stroke_b, stroke_g, stroke_r, stroke_alpha))
    stroke = cv2pil(stroke)
    bigger_img = cv2pil(bigger_img)
    result = Image.alpha_composite(stroke, bigger_img)
    return result


def change_matrix(input_mat, stroke_size):
    stroke_size = stroke_size - 1
    mat = np.ones(input_mat.shape)
    check_size = stroke_size + 1.0
    mat[input_mat > check_size] = 0
    border = (input_mat > stroke_size) & (input_mat <= check_size)
    mat[border] = 1.0 - (input_mat[border] - stroke_size)
    return mat


def cv2pil(cv_img):
    pil_img = Image.fromarray(cv_img.astype("uint8"))
    return pil_img


def create_outline(image: str, thickness: int):
    name, ext = os.path.splitext(image)
    img = Image.open(image).convert("RGBA")
    outlined_img = stroke(img, 100, thickness, [COLOR])
    outlined_img.save(f"{name}_outline{ext}")


def main():
    for file in glob.glob(f"{DIR}/*.png"):
        filename = os.path.basename(file)
        name, ext = os.path.splitext(filename)
        if not name.endswith("_outline"):
            print(f"Creating outline for {filename}...")
            create_outline(file, thickness=10)


if __name__ == "__main__":
    main()
