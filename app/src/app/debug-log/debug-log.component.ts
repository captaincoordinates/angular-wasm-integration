import { Component } from '@angular/core';
import { LogEntry, LogEntrySource, LogService } from '../log.service';

interface DisplayEntry {
  message: string;
  className: string;
}

@Component({
  selector: 'app-debug-log',
  templateUrl: './debug-log.component.html',
  styleUrls: ['./debug-log.component.sass']
})
export class DebugLogComponent {

  public entries: DisplayEntry[] = [];

  constructor(
    logService: LogService,
  ) {
    logService.registerConsumer((entry: LogEntry) => {
      let className: string;
      let message: string;
      if (entry.message) {
        message = entry.message;
        switch (entry.source) {
          case LogEntrySource.JS:
            className = "js-entry";
            break;
          case LogEntrySource.WASM:
            className = "wasm-entry";
            break;
          default:
            className = "unknown-entry";
            break;
        }
      } else {
        message = "-----"
        className =  "break-entry";
      }
      this.entries.unshift({message: `${(new Date().toLocaleTimeString("en-CA", {hour12: false}))}: ${message}`, className: className});
    });
  }
}
