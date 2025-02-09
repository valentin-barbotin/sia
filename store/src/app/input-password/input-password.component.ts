import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatInputModule } from '@angular/material/input';
import { MatIconModule } from '@angular/material/icon';
import { TextFieldModule } from '@angular/cdk/text-field';
import { ReactiveFormsModule, FormControl } from '@angular/forms';

@Component({
  selector: 'store-input-password',
  standalone: true,
  imports: [
    CommonModule,
    ReactiveFormsModule,
    MatInputModule,
    TextFieldModule,
    MatIconModule,
  ],
  templateUrl: './input-password.component.html',
  styleUrls: [],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class InputPasswordComponent {
  hidden = true;

  @Input({ required: true }) placeholder = '';
  @Input({ required: true }) label = '';
  // @ts-ignore
  @Input({ required: true }) formControlPassword: FormControl;

  get value(): string {
    return this.formControlPassword.value;
  }

  toggleVisibility() {
    this.hidden = !this.hidden;
  }
}