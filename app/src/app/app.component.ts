import { Component } from '@angular/core';
import { OptionValue as SelectedImage } from './image-selection/image-selection.component';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass']
})
export class AppComponent {

  public selectedImage?: SelectedImage;

  public handleImageSelectionChange(selection: SelectedImage): void {
    this.selectedImage = selection;
  }
}
