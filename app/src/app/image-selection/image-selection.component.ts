import { Component, EventEmitter, Output } from '@angular/core';

export enum OptionType {
  none,
  read,
  derived,
}

export enum DerivedType {
  ndvi = "ndvi"
}

export interface OptionValue {
  type: OptionType;
  value: string;
}

interface DisplayValue extends OptionValue {
  title: string;
}

@Component({
  selector: 'app-image-selection',
  templateUrl: './image-selection.component.html',
  styleUrls: ['./image-selection.component.sass']
})
export class ImageSelectionComponent {

  @Output()
  public optionChange = new EventEmitter<OptionValue>();

  public readonly options: DisplayValue[] = [{
    type: OptionType.none,
    value: "",
    title: ""
  }, {
    type: OptionType.read,
    value: "4",
    title: "Band 4"
  }, {
    type: OptionType.read,
    value: "8",
    title: "Band 8"
  }, {
    type: OptionType.derived,
    value: DerivedType.ndvi,
    title: "NDVI (derived)"
  }];

  public selectedOption: DisplayValue = this.options.filter(x => x.type === OptionType.none)[0];

  public selectedOptionChanged(event: Event): void {
    this.optionChange.emit(this.selectedOption);
  }
}
