import { AfterViewChecked, Component, OnInit, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatTableDataSource, MatTableModule, MatRowDef } from '@angular/material/table'
import { MatPaginator, MatPaginatorModule } from '@angular/material/paginator'
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatPseudoCheckboxModule, MatRippleModule } from '@angular/material/core';
import { MatDialog, MAT_DIALOG_DATA, MatDialogRef, MatDialogModule } from '@angular/material/dialog'
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { SelectionModel } from '@angular/cdk/collections';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { Observer, Subscription, TeardownLogic, delay, first } from 'rxjs';

import { InputPasswordComponent } from '../input-password/input-password.component';
import { FormBuilder, FormControl, FormGroup, Validators } from '@angular/forms';

import { AlertService } from '../alert.service';
import { BytesPipe } from '../bytes.pipe';
import { MimePipe } from '../mime.pipe';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { NgxFileService } from '../ngx-file/ngx-file.service';
import { FileInfo, FileInfoUpload } from '../ngx-file/fileinfo.interface';
import { NgxFileSelectionComponent } from '../ngx-file/upload.component';

@Component({
  selector: 'app-store',
  standalone: true,
  imports: [
    CommonModule,
    MatPaginatorModule,
    MatTableModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    MatRippleModule,
    MatDialogModule,
    MatProgressBarModule,
    MatCheckboxModule,
    MatIconModule,
    MatMenuModule,
    BytesPipe,
    MimePipe,
    InputPasswordComponent
  ],
  templateUrl: './store.component.html',
  styleUrls: ['./store.component.sass']
})
export class StoreComponent implements OnInit {
  columns: string[] = [
    'select',
    'name',
    'type',
    'size',
    'creation',
    'updated',
    'actions'
  ];

  formName!: FormGroup;

  get secretKeyControl() {
    return this.formName.get('secretKey') as FormControl;
  }


  footerColumns: string[] = [
    'name',
  ];

  dataSource = new MatTableDataSource<FileInfo>();
  selection = new SelectionModel<FileInfo>(true, []);
  data: FileInfo[] = [];
  loadingInProgress = false;
  firstLoadDone = false;

  constructor(
    private readonly fb: FormBuilder,
    public dialog: MatDialog,
    private fileService: NgxFileService,
    private alertService: AlertService
  ) {}

  ngOnInit(): void {
    this.fillData();
    this.formName = this.fb.group({
      secretKey: ["", Validators.required],
    });
  }

  clearList() {
    this.dataSource.data = [];
  }

  generateSecretKey() {
    const key = this.generateRandomKey();
    const keyHex = this.bytesToHex(key);

    this.secretKeyControl.setValue(keyHex);
  }

  generateRandomKey() {
    const array = new Uint8Array(32);
    window.crypto.getRandomValues(array);
    return array;
  }

  bytesToHex(bytes: Uint8Array): string {
    return Array.from(bytes)
        .map(byte => byte.toString(16).padStart(2, '0'))
        .join('');
  }

  isValidKey() {
    const keyHex = this.secretKeyControl.value;
    return keyHex.length === 64 && /^[0-9a-fA-F]+$/.test(keyHex);
  }



  fillData() {
    this.loadingInProgress = true;
    const obs = this.fileService.list()

    const observer: Observer<FileInfo[]> = {
      next: (value: FileInfo[]) => {
        this.dataSource.data = value;
      },
      error: (err: any) => {
        this.alertService.info('Error loading files', 'OK')
        this.loadingInProgress = false;
        this.firstLoadDone = true;
      },
      complete: () => {
        this.loadingInProgress = false;
        if (this.firstLoadDone) {
          this.alertService.info('List updated', 'OK')
        }
        this.firstLoadDone = true;
      }
    }

    obs.pipe().subscribe(observer);
  }

  refresh() {
    this.clearList();
    this.fillData();
  }

  applyFilter(event: Event) {
    const filterValue = (event.target as HTMLInputElement).value;
    this.dataSource.filter = filterValue.trim().toLowerCase();
  }

  uploadNewFile() {
    const dialogRef = this.dialog.open(NgxFileSelectionComponent, {
      panelClass: ['w-10/12', 'md:w-5/12', 'h-4/12', 'p-4'],
      data: {
        secretKey: this.secretKeyControl.value,
      }
    });

    dialogRef.afterClosed().subscribe((result: FileInfoUpload | undefined) => {
      // If a file is uploaded, insert it into the database
      if (result) {
        this.fileService.insert(result).subscribe({
          next: (value: any) => {
            this.alertService.info('Upload finished', 'OK');
            this.refresh();
          },
          error: (err: any) => {
            if (err.status === 422) {
              this.alertService.info('File already exists', 'OK');
            } else {
              this.alertService.info('Error uploading file', 'OK');
            }
          }
        });
      }
    });
  }

  isAllSelected() {
    const numSelected = this.selection.selected.length;
    const numRows = this.dataSource.data.length;
    return (numSelected === numRows);
  }

  masterToggle() {
    // If all files are selected, deselect them
    if (this.isAllSelected()) {
      this.dataSource.data.forEach((row) => {
        this.selection.deselect(row);
      });
      return;
    }
    // Otherwise, select all files
    this.dataSource.data.forEach((row) => {
      this.selection.select(row);
    });
  }

  downloadFile(name: string, identifier: string) {
    if (!identifier || identifier.length === 0) {
      return;
    }

    this.fileService.startDownload(name, identifier, this.secretKeyControl.value);
  }

  deleteFile(identifier: string) {
    if (!identifier || identifier.length === 0) {
      return;
    }

    const observer: Observer<string> = {
      next: (value: any) => {
        // this.alertService.info('File deleted', 'OK');
        // this.refresh();
      },
      error: (err: any) => {
        this.alertService.info('Error deleting file', 'OK');
      },
      complete: () => {
      }
    }

    const observer2: Observer<string> = {
      next: (value: any) => {
        this.alertService.info('File deleted', 'OK');
        this.refresh();
      },
      error: (err: any) => {
        this.alertService.info('Error deleting file', 'OK');
      },
      complete: () => {
      }
    }

    this.fileService.remove(identifier).subscribe(observer);
    this.fileService.deleteFile(identifier).subscribe(observer2);
  }

  downloadSelectedFiles() {
    this.selection.selected.forEach((file) => {
      this.downloadFile(file.name, file.identifier);
    });
  }
  deleteSelectedFiles() {
    this.selection.selected.forEach((file) => {
      this.deleteFile(file.identifier);
    });
  }
}