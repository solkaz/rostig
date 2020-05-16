import { BrowserModule } from '@angular/platform-browser';
import { NgModule, APP_INITIALIZER } from '@angular/core';
import { HotkeysModule } from '@ngneat/hotkeys';
import { AppComponent } from './app.component';
import { ButtonsComponent } from './buttons.component';
import { GameLoaderComponent } from './game-loader.component';
import { WasmService } from './wasm.service';
import { GameScreenComponent } from './game-screen.component';

function loadWasmFactory(wasmService: WasmService) {
  return wasmService.loadWasm();
}

@NgModule({
  declarations: [
    AppComponent,
    ButtonsComponent,
    GameLoaderComponent,
    GameScreenComponent,
  ],
  imports: [BrowserModule, HotkeysModule],
  providers: [
    {
      provide: APP_INITIALIZER,
      useFactory: loadWasmFactory,
      deps: [WasmService],
    },
  ],
  bootstrap: [AppComponent],
})
export class AppModule {}
