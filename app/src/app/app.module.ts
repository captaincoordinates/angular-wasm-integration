import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppComponent } from './app.component';
import { ImageSelectionComponent } from './image-selection/image-selection.component';
import { ImageDisplayComponent } from './image-display/image-display.component';
import { FormsModule } from '@angular/forms';
import { ImageStretchSelectionComponent } from './image-stretch-selection/image-stretch-selection.component';

@NgModule({
  declarations: [
    AppComponent,
    ImageSelectionComponent,
    ImageDisplayComponent,
    ImageStretchSelectionComponent
  ],
  imports: [
    BrowserModule,
    FormsModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
