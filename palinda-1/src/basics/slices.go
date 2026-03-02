package main

import (
	"image"
	"image/color"
	"image/png"
	"os"
)

func Pic(dx, dy int) [][]uint8 {
	// You will implement your solution here
}

func main() {
	saveImage(Pic(256, 256))
}

// This is a helper function to create your image output
// You do not need to understand how it works.
func saveImage(imgArr [][]uint8) {
	file, err := os.Create("output.png")
	if err != nil {
		return
	}
	defer file.Close()
	
	w := len(imgArr)
	h := len(imgArr[0])
	bounds := image.Rect(0, 0, w, h)
	img := image.NewRGBA(bounds)
	for i := 0; i < w; i++ {
		for j := 0; j < h; j++ {
			img.Set(
				i,
				j,
				color.RGBA{imgArr[j][i], imgArr[j][i], imgArr[j][i], 255},
			)
		}
	}
	err = png.Encode(file, img)
	if err != nil {
		return
	}
}
