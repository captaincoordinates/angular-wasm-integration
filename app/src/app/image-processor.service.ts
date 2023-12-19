import { Injectable } from '@angular/core';
import * as WasmImageProcessor from 'image-processor';


@Injectable({
  providedIn: 'root'
})
export class ImageProcessorService {

  constructor() { }

  public invoke(): void {
    const handle = WasmImageProcessor.Processor.new();
    handle.greet();
  }
}