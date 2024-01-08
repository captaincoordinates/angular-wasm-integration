import { AfterViewInit, Component, ElementRef, HostListener, Input, SimpleChanges, ViewChild } from '@angular/core';
import { ImageProcessorService } from '../image-processor.service';
import { OptionValue as ImageType, OptionType as SelectedImageType } from '../image-selection/image-selection.component';
import { ReplaySubject } from 'rxjs';

@Component({
  selector: 'app-image-display',
  templateUrl: './image-display.component.html',
  styleUrls: ['./image-display.component.sass']
})
export class ImageDisplayComponent implements AfterViewInit {

  @ViewChild("imageCanvas")
  public canvasEl!: ElementRef;

  @Input()
  public selectedImageOption?: ImageType;

  @Input()
  public stretchSelected?: boolean; 

  private initialising: ReplaySubject<void> = new ReplaySubject<void>(1);
  private debounceHandle?: number = undefined;

  constructor(
    private imageProcessor: ImageProcessorService,
  ) {}

  public ngAfterViewInit(): void {
    this.imageProcessor.init("user-123", "Pa$$word").then(() => {
      this.initialising.next();
    });
  }

  public ngOnChanges(changes: SimpleChanges): void {
    if (changes["selectedImageOption"] && !changes["selectedImageOption"].firstChange) {
      this.render();
    }
    if (changes["stretchSelected"] && !changes["stretchSelected"].firstChange) {
      this.render();
    }
  }

  @HostListener("window:resize", ["$event"])
  public onResize(_: Event) {
    if (this.debounceHandle !== undefined) {
      window.clearTimeout(this.debounceHandle);
    }
    this.debounceHandle = window.setTimeout(() => {
      this.debounceHandle = undefined;
      this.imageProcessor.clearCache();
      this.render();
    }, 200);
  }

  public get default_image_width(): number {
    return 0;
  }

  public get default_image_height(): number {
    return 0;
  }

  private render(): void {
    this.initialising.subscribe(() => {
      if (this.selectedImageOption) {
        switch (this.selectedImageOption.type) {
          case SelectedImageType.none:
            this.clearCanvas();
            break;
          case SelectedImageType.read:
            const band = parseInt(this.selectedImageOption.value, 10);
            const start = performance.now();
            this.imageProcessor.fetchImage(band, this.stretchSelected === true, this.canvas_width, this.canvas_height).then(data => {
              console.log(`fetched image in ${performance.now() - start}ms`);
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
    });
  }

  private get canvas_width(): number {
    return this.canvasEl.nativeElement.width;
  }

  private get canvas_height(): number {
    return this.canvasEl.nativeElement.height;
  }

  private clearCanvas(): void {
    const ctx = this.canvasEl.nativeElement.getContext("2d");
    ctx.clearRect(0, 0, this.canvasEl.nativeElement.width, this.canvasEl.nativeElement.height);
  }
}
