const fs = require('fs');
const { createCanvas, loadImage } = require("canvas");

function makeRandomImage(width = 64, height = 64) {
    const canvas = createCanvas(width, height);
    const context = canvas.getContext("2d");
    const imageData = context.getImageData(0, 0, width, height);

    // https://gist.github.com/biovisualize/5400576#file-index-html-L26
    const buffer = new ArrayBuffer(imageData.data.length);
    const clampedBuffer = new Uint8ClampedArray(buffer);
    const data = new Uint32Array(buffer);

    for (let x = 0; x < width; x++) {
        for (let y = 0; y < height; y++) {
            if (Math.random() < 0.3) continue;
            data[y * width + x] =
                (255 << 24) |
                (((Math.random() * 300) % 255) << 16) |
                (((Math.random() * 300) % 255) << 8) |
                (((Math.random() * 300) % 255))
        }
    }

    imageData.data.set(clampedBuffer);
    context.putImageData(imageData, 0, 0);

    // Make Buffer
    const dataUrl = canvas.toDataURL("image/png").substr("data:image/png;base64,".length);
    const imageBuffer = new Buffer(dataUrl, 'base64');
    const blob = new Blob([imageBuffer], { type: "image/png" });
    return { imageBuffer, imageData, data, blob }
}

(() => {
    const { imageBuffer, imageData, data, blob } = makeRandomImage(128, 128);
    fs.writeFileSync(`images/randomImage${Math.round(Math.random()*100)}.png`, imageBuffer);
})();