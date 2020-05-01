import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';

@Component({
  selector: 'app-buttons',
  template: `
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
  `,
  styles: [
    `
      :host {
        display: block;
      }

      .buttons-container {
        flex: 1 0 50%;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
      }
      .upper-buttons-container {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
      }

      .direction-button-row {
        display: flex;
        flex-direction: row;
        justify-content: center;
        height: 24px;
      }

      .lower-buttons-container {
        display: flex;
        flex-direction: row;
        justify-content: center;
      }

      .action-button {
        border-radius: 50%;
        width: 32px;
        height: 32px;
      }

      .menu-button {
        border-radius: 25%;
        width: 32px;
        height: 8px;
      }
    `,
  ],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ButtonsComponent implements OnInit {
  constructor() {}

  ngOnInit(): void {}
}
