import { AfterViewInit, Component, ElementRef, Input, SimpleChanges, ViewChild } from '@angular/core';
import { ImageProcessorService } from '../image-processor.service';
import { OptionValue as SelectedImage, OptionType as SelectedImageType } from '../image-selection/image-selection.component';
import { ReplaySubject } from 'rxjs';
import * as WasmImageProcessor from 'image-processor';

@Component({
  selector: 'app-image-display',
  templateUrl: './image-display.component.html',
  styleUrls: ['./image-display.component.sass']
})
export class ImageDisplayComponent implements AfterViewInit {

  @ViewChild("imageCanvas")
  public canvasEl!: ElementRef;

  @Input()
  public selectedImageOption?: SelectedImage;

  private initialising: ReplaySubject<void> = new ReplaySubject<void>(1);

  constructor(
    private imageProcessor: ImageProcessorService,
  ) {}

  public ngAfterViewInit(): void {
    this.imageProcessor.init("user-123", "Pa$$word").then(() => {
      this.initialising.next();
    });
  }

  public ngOnChanges(changes: SimpleChanges): void {
    if (changes["selectedImageOption"]) {
      this.handleSelectedImageChanged();
    }
  }

  public get default_image_width(): number {
    return 0;
  }

  public get default_image_height(): number {
    return 0;
  }

  private handleSelectedImageChanged(): void {
    this.initialising.subscribe(() => {
      if (this.selectedImageOption) {
        switch (this.selectedImageOption.type) {
          case SelectedImageType.none:
            this.clearCanvas();
            break;
          case SelectedImageType.read:
            const band = parseInt(this.selectedImageOption.value, 10);
            this.imageProcessor.fetchImage(band).then(data => {
              console.log(`loaded image with dimensions ${data.width},${data.height} with pointer starting at ${data.pixels_ptr()}`)
              this.imageProcessor.displayImage(this.canvasEl, data);
            });
            break;
          case SelectedImageType.derived:
            console.log(`attempting to derive new data`);
            break;
        }
      } else {
        this.clearCanvas();
      }
      console.log(`display handling change ${this.selectedImageOption?.value}`);
    });
  }

  private clearCanvas(): void {
    const ctx = this.canvasEl.nativeElement.getContext("2d");
    ctx.clearRect(0, 0, this.canvasEl.nativeElement.width, this.canvasEl.nativeElement.height);
  }
}
