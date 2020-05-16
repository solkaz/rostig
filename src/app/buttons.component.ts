import { Component, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { HotkeysService } from '@ngneat/hotkeys';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';

enum ButtonInput {
  A,
  B,
  Up,
  Down,
  Left,
  Right,
  Select,
  Start,
}

@UntilDestroy()
@Component({
  selector: 'app-buttons',
  template: `
    <div class="buttons-container">
      <div class="upper-buttons-container">
        <div style="border: 1px black solid;">
          <div class="direction-button-row">
            <button
              type="button"
              id="up"
              (click)="handleUpButton($event)"
            ></button>
          </div>
          <div class="direction-button-row">
            <button
              type="button"
              id="left"
              (click)="handleLeftButton($event)"
            ></button>
            <button type="button" id="middle"></button>
            <button
              type="button"
              id="right"
              (click)="handleRightButton($event)"
            ></button>
          </div>
          <div class="direction-button-row">
            <button
              type="button"
              id="down"
              (click)="handleDownButton($event)"
            ></button>
          </div>
        </div>
        <div>
          <div class="action-button-container">
            <button
              type="button"
              class="action-button"
              id="b"
              (click)="handleBButton($event)"
            ></button>
            <label for="b">B</label>
          </div>
          <div class="action-button-container">
            <button
              type="button"
              class="action-button"
              id="a"
              (click)="handleAButton($event)"
            ></button>
            <label for="a">A</label>
          </div>
        </div>
      </div>
      <div class="lower-buttons-container">
        <div class="menu-button-container">
          <button
            type="button"
            class="menu-button"
            id="select"
            (click)="handleSelectButton($event)"
          ></button>
          <br />
          <label for="select">Select</label>
        </div>
        <div class="menu-button-container">
          <button
            type="button"
            class="menu-button"
            id="start"
            (click)="handleStartButton($event)"
          ></button>
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
  constructor(private hotkeys: HotkeysService) {}

  ngOnInit() {
    this.hotkeys
      .addShortcut({ keys: 'w' })
      .pipe(untilDestroyed(this))
      .subscribe((e) => this.handleUpButton(e));
    this.hotkeys
      .addShortcut({ keys: 'a' })
      .pipe(untilDestroyed(this))
      .subscribe((e) => this.handleLeftButton(e));
    this.hotkeys
      .addShortcut({ keys: 's' })
      .pipe(untilDestroyed(this))
      .subscribe((e) => this.handleDownButton(e));
    this.hotkeys
      .addShortcut({ keys: 'd' })
      .pipe(untilDestroyed(this))
      .subscribe((e) => this.handleRightButton(e));
    this.hotkeys
      .addShortcut({ keys: 'space' })
      .pipe(untilDestroyed(this))
      .subscribe((e) => this.handleSelectButton(e));
    this.hotkeys
      .addShortcut({ keys: 'enter' })
      .pipe(untilDestroyed(this))
      .subscribe((e) => this.handleStartButton(e));
    this.hotkeys
      .addShortcut({ keys: 'j' })
      .pipe(untilDestroyed(this))
      .subscribe((e) => this.handleAButton(e));
    this.hotkeys
      .addShortcut({ keys: 'k' })
      .pipe(untilDestroyed(this))
      .subscribe((e) => this.handleBButton(e));
  }

  handleAButton(event: any) {
    console.log({ inputType: ButtonInput.A, event });
  }

  handleBButton(event: any) {
    console.log({ inputType: ButtonInput.B, event });
  }

  handleStartButton(event: any) {
    console.log({ inputType: ButtonInput.Start, event });
  }

  handleSelectButton(event: any) {
    console.log({ inputType: ButtonInput.Select, event });
  }

  handleUpButton(event: any) {
    console.log({ inputType: ButtonInput.Up, event });
  }

  handleDownButton(event: any) {
    console.log({ inputType: ButtonInput.Down, event });
  }

  handleLeftButton(event: any) {
    console.log({ inputType: ButtonInput.Left, event });
  }

  handleRightButton(event: any) {
    console.log({ inputType: ButtonInput.Right, event });
  }

  handleButton(inputType: ButtonInput, event: any) {
    console.log({ inputType, event });
  }
}
