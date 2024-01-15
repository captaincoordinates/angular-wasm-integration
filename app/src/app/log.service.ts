import { Injectable } from '@angular/core';

export enum LogEntrySource {
  JS,
  WASM
}

export interface LogEntry {
  message: string;
  source: LogEntrySource;
}

@Injectable({
  providedIn: 'root'
})
export class LogService {

  private consumers: ((entry: LogEntry) => void)[] = [];

  constructor() {
    (<any>window).appLogger.wasmLog = this.wasmLog.bind(this);
  }

  public registerConsumer(consumer: (entry: LogEntry) => void): void {
    this.consumers.push(consumer);
  }

  /*
    In a more comprehensive solution this service could have debug / info / warn / error
    functions and filter which messages are disatched to consumers based on each consumer's
    configuration.
  */
  public log(message: string): void {
    this.consumers.forEach(consumer => {
      consumer({
        message: message,
        source: LogEntrySource.JS
      });
    });
  }

  public wasmLog(message: string): void {
    this.consumers.forEach(consumer => {
      consumer({
        message: message,
        source: LogEntrySource.WASM
      });
    });
  }

  public markBreak(): void {
    this.log("");
  }
}
