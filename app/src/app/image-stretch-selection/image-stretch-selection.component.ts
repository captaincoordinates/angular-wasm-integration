import { Component, EventEmitter, Output } from '@angular/core';

@Component({
  selector: 'app-image-stretch-selection',
  templateUrl: './image-stretch-selection.component.html',
  styleUrls: ['./image-stretch-selection.component.sass']
})
export class ImageStretchSelectionComponent {

  public static readonly STRETCH_DEFAULT: boolean = true;

  @Output()
  public stretchChange = new EventEmitter<boolean>();

  public isChecked: boolean = ImageStretchSelectionComponent.STRETCH_DEFAULT;

  public stretchChanged(_: Event): void {
    this.announceSelectedOption();
  }

  private announceSelectedOption(): void {
    this.stretchChange.emit(this.isChecked);
  }
}
