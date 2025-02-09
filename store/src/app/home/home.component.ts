import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { StoreComponent } from '../store/store.component';
import { NgxAuthService } from '@kz/ngx-auth';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [CommonModule, StoreComponent],
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.sass']
})
export class HomeComponent {
  constructor(
    private readonly userService: NgxAuthService
  ) {}

  get isLoggedIn() {
    return this.userService.isLoggedIn();
  }
}
