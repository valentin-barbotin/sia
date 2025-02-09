import { HttpClient, HttpErrorResponse, HttpEvent, HttpEventType, HttpUploadProgressEvent } from '@angular/common/http';
import { Inject, Injectable, OnInit } from '@angular/core';
import { BehaviorSubject, Observable, Observer, Subscription, catchError, lastValueFrom, map, of, throwError, timeout } from 'rxjs';
import { ENVIRONMENT_TOKEN, Environment } from '@kz/ngx-env';
import { saveAs } from 'file-saver';

import { FileInfo, FileInfoUpload } from './fileinfo.interface';
import { AlertService } from '../alert.service';

@Injectable({
  providedIn: 'root',
})
export class NgxFileService implements OnInit {
  environment: Environment;

  get url() : string {
    return this.environment.STORAGE_URL + '/api/v1';
  }

  get storeURL() : string {
    return this.environment.STORE_URL + '/api/v1/file';
  }

  constructor(
    private http: HttpClient,
    private alertService: AlertService,
    @Inject(ENVIRONMENT_TOKEN) environment: Environment 
    ) {
      this.environment = environment;
    }

  ngOnInit(): void {
  }

  /**
   * Uploads a file to the server
   */
  uploadFile(file: File, secretKey: string): Observable<HttpEvent<string>> {
    const formData = new FormData();
    formData.append('file', file);
    formData.append('secretKey', secretKey);
  
    return this.http.put(`${this.url}/upload`, formData, {
        reportProgress: true,
        responseType: 'text',
        observe: 'events',
      })
  }

  downloadFile(id: string, secretKey: string): Observable<Blob> {
    if (!id) {
      throw new Error('ID is undefined');
    }

    return this.http.get(`${this.url}/download`, {
      responseType: 'blob',
      params: {
        id,
        secretKey
      },
    })
  }

  startDownload(filename: string, identifier: string, secretKey: string): Subscription {
    const observer: Observer<Blob> = {
      next: (value: Blob) => {
        if (!value) {
          observer.error('No file returned');
          return;
        }

        saveAs(value, filename);
      },
      error: (err: any) => {
        this.alertService.info('Error downloading file', 'OK')
      },
      complete: () => {
      }
    }

    return this.downloadFile(identifier, secretKey).subscribe(observer);
  }

  upload(file: File, secretKey: string) {
    return this.uploadFile(file, secretKey);
  }

  deleteFile(id: string): Observable<any> {
    if (!id) {
      throw new Error('ID is undefined');
    }

    return this.http.delete(`${this.url}/delete`, {
      params: {
        id
      },
    })
  }

  list(): Observable<FileInfo[]> {
    return this.http.get(`${this.storeURL}/list`, {
      responseType: 'json',
    })
    .pipe(
      map((data: any) => {
        return data as FileInfo[];
      }),
    );
  }

  insert(info: FileInfoUpload) {
    return this.http.put(`${this.storeURL}/insert`, info, {
      responseType: 'text',
    })
  }

  remove(identifier: string) {
    return this.http.delete(`${this.storeURL}/remove`, {
      responseType: 'text',
      params: {
        id: identifier,
      },
    })
  }

}
