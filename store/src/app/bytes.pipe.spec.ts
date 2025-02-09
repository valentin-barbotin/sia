import { BytesPipe } from './bytes.pipe';

describe('BytesPipe', () => {
  it('create an instance', () => {
    const pipe = new BytesPipe();
    expect(pipe).toBeTruthy();
  });

  it('should transform 1024 to 1KB', () => {
    const pipe = new BytesPipe();
    expect(pipe.transform(1024)).toEqual('1KB');
  });

  it('should transform 1024*1024 to 1MB', () => {
    const pipe = new BytesPipe();
    expect(pipe.transform(1024*1024)).toEqual('1MB');
  });

  it('should transform 1024*1024*1024 to 1GB', () => {
    const pipe = new BytesPipe();
    expect(pipe.transform(1024*1024*1024)).toEqual('1GB');
  });
});
