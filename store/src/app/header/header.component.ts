import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { Router, RouterModule } from '@angular/router';

import { NgxAuthService } from '@kz/ngx-auth';
import { AlertService } from '../alert.service';

@Component({
  selector: 'app-header',
  standalone: true,
  imports: [CommonModule, MatToolbarModule, MatIconModule, MatMenuModule, RouterModule],
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.sass']
})
export class HeaderComponent {

  constructor(
    private readonly ngxAuthService: NgxAuthService,
    private alertService: AlertService,
    private readonly router: Router
  ) {}

  get isLoggedIn(): boolean {
    return this.ngxAuthService.isLoggedIn();
  }

  logout() {
    this.ngxAuthService.logout();
    this.router.navigate(['/'])
    this.alertService.info('Logout done', 'OK')
  }
}
