import { Pipe, PipeTransform } from '@angular/core';
import mime from 'mime';

@Pipe({
  name: 'mime',
  standalone: true
})
export class MimePipe implements PipeTransform {

  transform(value: string, ...args: unknown[]): string {
    return mime.getExtension(value) ?? value;
  }

}
