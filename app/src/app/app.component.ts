import { Component, ViewChild, ElementRef, AfterViewInit } from '@angular/core';
import { ImageProcessorService } from './image-processor.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass']
})
export class AppComponent implements AfterViewInit {

  @ViewChild("imageCanvas")
  public canvasEl!: ElementRef;

  title = 'app';

  constructor(
    private imageProcessor: ImageProcessorService,
  ) {}

  public ngAfterViewInit(): void {
    this.imageProcessor.init("user-123", "Pa$$word").then(() => {
      this.imageProcessor.fetchImage(4).then(data => {
        console.log(`loaded image with dimensions ${data.width},${data.height} with pointer starting at ${data.pixels_ptr()}`)
        this.imageProcessor.displayImage(this.canvasEl, data);
      });
    });
  }

  public get default_image_width(): number {
    return 0;
  }

  public get default_image_height(): number {
    return 0;
  }
}
