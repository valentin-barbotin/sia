import { Injectable } from '@angular/core';
import {
  HttpRequest,
  HttpHandler,
  HttpEvent,
  HttpInterceptor
} from '@angular/common/http';
import { Observable } from 'rxjs';
import { NgxAuthService } from '@kz/ngx-auth';
import { Router } from '@angular/router';

@Injectable()
export class AuthInterceptor implements HttpInterceptor {

  constructor(
    private userService: NgxAuthService,
    private readonly router: Router
  ) {}

  intercept(request: HttpRequest<unknown>, next: HttpHandler): Observable<HttpEvent<unknown>> {
    if (!this.userService.isLoggedIn()) {
      return next.handle(request);
    }

    const token = this.userService.getJWT();
    if (!token) {
      return next.handle(request);
    }

    if (!request.url.includes('scrypteur.com')) {
      return next.handle(request);
    }

    if (this.userService.isExpired(token)) {
      this.userService.logout();
      this.router.navigate(['/']);
      return next.handle(request);
    }

    const authRequest = request.clone({
      headers: request.headers.set('Authorization', token)
    });

    return next.handle(authRequest);
  }
}
