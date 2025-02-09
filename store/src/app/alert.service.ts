import { Injectable } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar'

@Injectable({
  providedIn: 'root'
})
export class AlertService {

  constructor(
    private snackBar: MatSnackBar
  ) { }

  info(message: string, action: string) {
    this.snackBar.open(message, action);
  }
}
