import { Pipe, PipeTransform } from '@angular/core';
import bytes from 'bytes';

@Pipe({
  name: 'bytes',
  standalone: true
})
export class BytesPipe implements PipeTransform {

  transform(value: string | number, ...args: unknown[]): string | number {
    // @ts-ignore
    return bytes(value);
  }

}
