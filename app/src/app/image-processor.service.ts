import { Injectable } from '@angular/core';
import * as WasmImageProcessor from 'image-processor';


@Injectable({
  providedIn: 'root'
})
export class ImageProcessorService {

  constructor() { }

  public invoke(username: string, password: string): void {
    const handle = WasmImageProcessor.Processor.new();
    handle.authenticate(username, password).then(() => {
      handle.fetch_image(4).then(() => {
        
      });
    });
  }
}