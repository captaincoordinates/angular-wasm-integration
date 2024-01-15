import { AfterViewInit, Component, ElementRef, HostListener, Input, SimpleChanges, ViewChild } from '@angular/core';
import { ImageProcessorImageData, ImageProcessorService } from '../image-processor.service';
import { OptionValue as ImageType, OptionType as SelectedImageType } from '../image-selection/image-selection.component';
import { ReplaySubject } from 'rxjs';
import { LogService } from '../log.service';

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
    private logService: LogService,
  ) {}

  public ngAfterViewInit(): void {
    this.logService.log("initialising image processor");
    this.imageProcessor.init("user-123", "Pa$$word").then(() => {
      this.initialising.next();
      this.logService.markBreak();
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
      this.logService.log("handing resize");
      this.debounceHandle = undefined;
      this.imageProcessor.clearCache();
      this.render();
    }, 200);
  }

  private render(): void {
    this.initialising.subscribe(() => {
      if (this.selectedImageOption) {
        this.logService.log("rendering image in canvas");
        const start = performance.now();
        switch (this.selectedImageOption.type) {
          case SelectedImageType.none:
            this.clearCanvas();
            break;
          case SelectedImageType.read:
            const band = parseInt(this.selectedImageOption.value, 10);
            this.imageProcessor.fetchImage(band, this.stretchSelected === true, this.canvas_width, this.canvas_height).then(data => {
              this.logImageStats(data, start);
              this.imageProcessor.displayImage(this.canvasEl, data);
              this.logService.markBreak();
            });
            break;
          case SelectedImageType.derived:
            this.logService.log("derived-type image");
            this.imageProcessor.calculateNdvi(this.stretchSelected === true, this.canvas_width, this.canvas_height).then(data => {
              this.logImageStats(data, start);
              this.imageProcessor.displayImage(this.canvasEl, data);
              this.logService.markBreak();
            });
            break;
        }
      } else {
        this.clearCanvas();
        this.logService.markBreak();
      }
    });
  }

  private logImageStats(data: ImageProcessorImageData, timer: number): void {
    this.logService.log(`fetched image in ${Math.round(performance.now() - timer)}ms`);
    this.logService.log(`image: ${data.width}x${data.height}, pointer: ${data.pixels_ptr()}`)
  }

  private get canvas_width(): number {
    return this.canvasEl.nativeElement.clientWidth;
  }

  private get canvas_height(): number {
    return this.canvasEl.nativeElement.clientHeight;
  }

  private clearCanvas(): void {
    this.logService.log("clearing canvas");
    const ctx = this.canvasEl.nativeElement.getContext("2d");
    ctx.clearRect(0, 0, this.canvasEl.nativeElement.width, this.canvasEl.nativeElement.height);
  }
}
