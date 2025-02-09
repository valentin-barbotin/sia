import { Component, Inject, OnDestroy, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { HttpClientModule, HttpEvent, HttpEventType, HttpProgressEvent } from '@angular/common/http';
import { MatButtonModule } from '@angular/material/button';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatSelectModule } from '@angular/material/select';
import { FormControl, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { Subscription } from 'rxjs';
import { ENVIRONMENT_TOKEN, Environment } from '@kz/ngx-env';
import { NgxFileService } from './ngx-file.service';
import { AlertService } from '../alert.service';
import { FileInfoUpload } from '@kz/ngx-file';

@Component({
  selector: 'lib-ngx-file-selection',
  standalone: true,
  imports: [
    CommonModule,
    HttpClientModule,
    MatButtonModule,
    MatProgressBarModule,
    FormsModule,
    MatFormFieldModule,
    MatSelectModule,
    ReactiveFormsModule
  ],
  templateUrl: './upload.component.html',
  styleUrls: ['./upload.component.sass'],
  //TODO: check tests
  // providers: [
  //   {
  //     provide: MatDialogRef,
  //     useValue: {}
  //   },
  // ]
})
export class NgxFileSelectionComponent {
  file: File | null = null;
  uploadInProgress: boolean = false;
  uploadProgression: number = 0;
  uploadSubscription: Subscription | null = null;
  environment: Environment;

  tagsSelected = new FormControl<string[]>([]);

  constructor(
    private dialogRef: MatDialogRef<NgxFileSelectionComponent>,
    private fileService: NgxFileService,
    private readonly alertService: AlertService,
    @Inject(ENVIRONMENT_TOKEN) environment: Environment,
    @Inject(MAT_DIALOG_DATA) public data: {
        secretKey: string;
    }
  ) {
      this.environment = environment;
    }

  onFileSelected(event: any) {
    this.file = event.target.files[0];
  }

  ngOnInit(): void {
    // unsubscribe on destroy
    this.dialogRef.beforeClosed().subscribe(() => {
      if (this.uploadSubscription) {
        console.log('unsubscribe');
        this.uploadSubscription.unsubscribe();
      }
    });
  }

  get tags() {
    return this.environment.STORAGE_TAGS_AVAILABLE;
  }

  upload() {
    let identifier: string | null = null;

    let tags = this.tagsSelected.value ?? [];

    if (this.file) {
        const upload = this.fileService.uploadFile(this.file, this.data.secretKey);

        const observer = {
          next: (data: HttpEvent<string>) => {
            switch (data.type) {
              case HttpEventType.Sent:
                this.uploadInProgress = true;
                break;
              case HttpEventType.UploadProgress:
                let _data = data as HttpProgressEvent;
                if (_data.total) {
                  this.uploadProgression = Math.round((_data.loaded / _data.total) * 100);
                }
                break;
              case HttpEventType.Response:
                console.log('identifier:');
                console.log(data.body);
                identifier = data.body;
                break;
            }
          },
          error: (error: any) => {
            console.error(error);
            this.uploadInProgress = false;
            this.alertService.info('Upload Failed', 'OK');
          },
          complete: () => {
            this.uploadInProgress = false;
            if (!identifier) {
              throw new Error('identifier is null');
            }

            if (!this.file) {
              throw new Error('file is null');
            }

            const { name, size, type } = this.file;

            const res: FileInfoUpload = {
              name,
              identifier,
              size,
              mime_type: type ? type : 'application/octet-stream',
              tags
            };

            this.dialogRef.close(res);
          }
        };

        this.uploadSubscription = upload.subscribe(observer);
    }
  }

  cancelSelection() {
    this.dialogRef.close();
  }
}
