import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-root',
  template: `
    <div class="container">
      <div class="game-container">
        <div class="display-container">
          <!-- TODO: Add display -->
          <div class="screen" *ngIf="loadedGame; else upload">
            <app-game-screen></app-game-screen>
          </div>
          <ng-template #upload>
            <app-game-loader (gameLoad)="loadedGame = true"></app-game-loader>
          </ng-template>
        </div>

        <app-buttons class="buttons-container"></app-buttons>
      </div>
      <div class="footer">
        <p>Not endorsed by Nintendo</p>
      </div>
    </div>
  `,
  styles: [
    `
      .container {
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
      }

      .game-container {
        flex: 10 1 auto;
        width: 80%;
        margin: 0 auto;
        display: flex;
        flex-direction: column;
      }

      .display-container {
        flex: 1 0 50%;
        border: 5px solid black;
      }

      .buttons-container {
        flex: 1 0 50%;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
      }
    `,
  ],
})
export class AppComponent implements OnInit {
  loadedGame = false;

  ngOnInit() {}
}
