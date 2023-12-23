import { Component } from '@angular/core';
import { ImageProcessorService } from './image-processor.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass']
})
export class AppComponent {
  title = 'app';

  constructor(
    private imageProcessor: ImageProcessorService
  ) {
    this.imageProcessor.invoke("user-123", "Pa$$word");
  }
}
