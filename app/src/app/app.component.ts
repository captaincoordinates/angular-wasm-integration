import { Component } from '@angular/core';
import { OptionValue as ImageType } from './image-selection/image-selection.component';
import { ImageStretchSelectionComponent } from './image-stretch-selection/image-stretch-selection.component';


@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass']
})
export class AppComponent {

  public selectedImage?: ImageType;
  public stretchSelected: boolean = ImageStretchSelectionComponent.STRETCH_DEFAULT;

  constructor() {
    (<any>window).appLogger = {
      wasm_log: (message: string) => {
        console.log(`log service not yet initialised: ${message}`);
      }
    };
  }

  public handleImageSelectionChange(selection: ImageType): void {
    this.selectedImage = selection;
  }

  public handleStretchChange(selected: boolean): void {
    this.stretchSelected = selected;
  }
}
