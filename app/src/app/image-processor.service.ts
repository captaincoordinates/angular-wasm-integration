import { ElementRef, Injectable } from '@angular/core';
import * as WasmImageProcessor from 'image-processor';
import { memory as wasmImageProcessorMemory } from 'image-processor/image_processor_bg.wasm'


@Injectable({
  providedIn: 'root'
})
export class ImageProcessorService {

  private handle: WasmImageProcessor.Processor;

  constructor() {
    this.handle = WasmImageProcessor.Processor.new();
  }

  public init(username: string, password: string): Promise<unknown> {
    return this.handle.authenticate(username, password)
      .catch(err => {
        alert(`problem authenticating: ${err}`)
      })
    ;
  }

  public fetchImage(band: number, histogramStretch: boolean): Promise<WasmImageProcessor.Image> {
    return this.handle.fetch_image(band, histogramStretch);
  }

  public displayImage(canvasElement: ElementRef, imageData: WasmImageProcessor.Image): void {
    const start = performance.now();
    const pixelValues = new Uint8Array(wasmImageProcessorMemory.buffer, imageData.pixels_ptr(), imageData.width * imageData.height);
    canvasElement.nativeElement.width = imageData.width;
    canvasElement.nativeElement.height = imageData.height;
    const ctx = canvasElement.nativeElement.getContext("2d");
    ctx.beginPath();
    let renderedColours: {[index: number]: number} = {};
    let firstColours: number[] = [];
    let lastColours: number[] = [];
    for (let row = 0; row < imageData.height; row++) {
      for (let col = 0; col < imageData.width; col++) {
        const idx = row * imageData.width + col;
        const value = pixelValues[idx];
        if (row == 0 && col < 10) {
          firstColours.push((value === undefined ? -1 : value));
        }
        if (row == (imageData.height - 1) && col >= (imageData.width - 10)) {
          lastColours.push((value === undefined ? -1 : value));
        }
        if (renderedColours.hasOwnProperty(value)) {
          renderedColours[value]++;
        } else {
          renderedColours[value] = 1;
        }
        ctx.fillStyle = this.rgbToHexColor(value, value, value);
        ctx.fillRect(row, col, 1, 1);
      }
    }
    console.log(`js first: ${firstColours}`);
    console.log(`js last: ${lastColours}`);
    ctx.stroke();
    console.log(`elapsed: ${performance.now() - start}ms`);
  }

  private rgbToHexColor(red: number, green: number, blue: number): string {
    const clamp = (value: number) => Math.max(0, Math.min(255, value));
    const hexString = ((clamp(red) << 16) | (clamp(green) << 8) | clamp(blue)).toString(16);
    const paddedHexString = hexString.padStart(6, '0');
    return `#${paddedHexString}`;
  }
}