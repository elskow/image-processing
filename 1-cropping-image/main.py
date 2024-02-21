import cv2

image_path = "images/operator-meme.png"

if __name__ == "__main__":
    image = cv2.imread(image_path)

    cv2.imshow("Original", image)

    red = image.copy()
    red[:, :, 0] = 0
    red[:, :, 1] = 0

    green = image.copy()
    green[:, :, 0] = 0
    green[:, :, 2] = 0

    blue = image.copy()
    blue[:, :, 1] = 0
    blue[:, :, 2] = 0

    cv2.imshow("Red", red)
    cv2.imshow("Green", green)
    cv2.imshow("Blue", blue)

    combine = red + green + blue

    cv2.imshow("Combine", combine)

    image_gs = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    cv2.imshow("Gray Scale", image_gs)

    cropped = combine[300:500, 0:687, :]
    cv2.imshow("Cropped", cropped)

    cv2.waitKey(0)
    cv2.destroyAllWindows()