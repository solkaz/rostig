import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { WasmService } from './wasm.service';

@Component({
  selector: 'app-game-screen',
  template: ` <canvas id="canvas" width="160" height="144"></canvas> `,
  styles: [
    `
      :host {
        display: block;
      }
    `,
  ],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GameScreenComponent implements OnInit {
  constructor(private wasm: WasmService) {}

  ngOnInit(): void {}

  onUpload(event: any) {
    console.log(event);
    (event.target.files as FileList)[0].arrayBuffer().then((result) => {
      this.wasm.module.load_cartridge(new Uint8Array(result));
    });
  }
}
