import { ElementRef, Injectable } from '@angular/core';
import * as WasmImageProcessor from 'image-processor';
import { memory as wasmImageProcessorMemory } from 'image-processor/image_processor_bg.wasm'
import { LogService } from './log.service';

export interface ImageProcessorImageData extends WasmImageProcessor.Image {}

@Injectable({
  providedIn: 'root'
})
export class ImageProcessorService {

  private handle: WasmImageProcessor.Processor;

  constructor(
    private logService: LogService
  ) {
    this.handle = WasmImageProcessor.Processor.new();
  }

  public init(username: string, password: string): Promise<unknown> {
    this.logService.log("asking WASM processor to authenticate");
    return this.handle.authenticate(username, password)
      .catch(err => {
        alert(`problem authenticating: ${err}`)
      })
    ;
  }

  public fetchImage(band: number, histogramStretch: boolean, width: number, height: number): Promise<WasmImageProcessor.Image> {
    return this.handle.fetch_image(band, histogramStretch, width, height);
  }

  public calculateNdvi(histogramStretch: boolean, width: number, height: number): Promise<WasmImageProcessor.Image> {
    return this.handle.calculate_ndvi(histogramStretch, width, height);
  }

  public displayImage(canvasElement: ElementRef, imageData: WasmImageProcessor.Image): void {
    const start = performance.now();
    const pixelValues = new Uint8ClampedArray(wasmImageProcessorMemory.buffer, imageData.pixels_ptr(), imageData.pixels_count());
    canvasElement.nativeElement.width = imageData.width;
    canvasElement.nativeElement.height = imageData.height;
    const ctx = canvasElement.nativeElement.getContext("2d");
    ctx.putImageData(new ImageData(pixelValues, imageData.width, imageData.height), 0, 0);
    this.logService.log(`display updated in ${Math.round(performance.now() - start)}ms`);
  }

  public clearCache(): void {
    this.logService.log(`clear processed cache`);
    this.handle.clear_processed_cache();
  }
}