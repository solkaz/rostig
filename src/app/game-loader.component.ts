import {
  Component,
  OnInit,
  ChangeDetectionStrategy,
  Output,
  EventEmitter,
} from '@angular/core';
import { WasmService } from './wasm.service';

@Component({
  selector: 'app-game-loader',
  template: `
    <div style="height: 100%; background-color: blue;">
      Upload a game
      <input
        type="file"
        accept=".gb"
        name="game"
        id="game-upload"
        (change)="onUpload($event)"
      />
    </div>
  `,
  styles: [
    `
      :host {
        display: block;
      }
    `,
  ],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GameLoaderComponent implements OnInit {
  @Output() gameLoad = new EventEmitter<void>();
  constructor(private wasm: WasmService) {}

  ngOnInit(): void {}

  onUpload(event: any) {
    console.log(event);
    (event.target.files as FileList)[0].arrayBuffer().then((result) => {
      this.wasm.module.load_cartridge(new Uint8Array(result));

      this.gameLoad.emit();
    });
  }
}
