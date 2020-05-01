import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root',
})
export class WasmService {
  module: typeof import('pkg');

  loadWasm() {
    return import('pkg').then((pkg) => {
      this.module = pkg;
    });
  }
}
