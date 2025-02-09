import { importProvidersFrom } from '@angular/core';
import { AppComponent } from './app/app.component';
import { provideAnimations } from '@angular/platform-browser/animations';
import { AppRoutingModule } from './app/app-routing.module';
import { BrowserModule, bootstrapApplication } from '@angular/platform-browser';
import { provideHttpClient, withInterceptors, withInterceptorsFromDi } from '@angular/common/http';
import { HTTP_INTERCEPTORS } from '@angular/common/http';

import { ENVIRONMENT_TOKEN } from '@kz/ngx-env'

import { environment } from './assets/environments/environment';
import { MatSnackBarModule } from '@angular/material/snack-bar';
import { AuthInterceptor } from './app/auth.interceptor';

bootstrapApplication(AppComponent, {
    providers: [
        importProvidersFrom(BrowserModule, AppRoutingModule, MatSnackBarModule),
        provideAnimations(),
        provideHttpClient(
            withInterceptorsFromDi()
        ),
        { provide: ENVIRONMENT_TOKEN, useValue: environment },
        { provide: HTTP_INTERCEPTORS, useClass: AuthInterceptor, multi: true}
    ]
})
  .catch(err => console.error(err));
