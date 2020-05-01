import { Component } from '@angular/core';

@Component({
  selector: 'app-root',
  template: `
    <div class="container">
      <div class="game-container">
        <div class="display-container">
          <!-- TODO: Add display -->
          <div class="screen">
            Loading
          </div>
        </div>

        <div class="buttons-container">
          <div class="upper-buttons-container">
            <div style="border: 1px black solid;">
              <div class="direction-button-row">
                <button type="button" id="up"></button>
              </div>
              <div class="direction-button-row">
                <button type="button" id="left"></button>
                <button type="button" id="middle"></button>
                <button type="button" id="right"></button>
              </div>
              <div class="direction-button-row">
                <button type="button" id="down"></button>
              </div>
            </div>
            <div>
              <div class="action-button-container">
                <button type="button" class="action-button" id="b"></button>
                <label for="b">B</label>
              </div>
              <div class="action-button-container">
                <button type="button" class="action-button" id="a"></button>
                <label for="a">A</label>
              </div>
            </div>
          </div>
          <div class="lower-buttons-container">
            <div class="menu-button-container">
              <button type="button" class="menu-button" id="select"></button>
              <br />
              <label for="select">Select</label>
            </div>
            <div class="menu-button-container">
              <button type="button" class="menu-button" id="start"></button>
              <br />
              <label for="start">Start</label>
            </div>
          </div>
        </div>
      </div>
      <div class="footer">
        <p>Not endorsed by Nintendo</p>
      </div>
    </div>
  `,
  styles: [],
})
export class AppComponent {
  title = 'ng-gameboy-emulator';
}
