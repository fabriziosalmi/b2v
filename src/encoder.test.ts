import { Encoder } from './encoder';
describe('Encoder', () => {
  it('should encode correctly', () => {
    const encoder = new Encoder({
      input_path: 'input.mp4',
      output_path: 'output.mp4',
      block_size: 32,
      data_shards: 2,
      parity_shards: 1,
      codec: 'libx264',
    });
    const result = encoder.run();
    expect(result).unwrap();
  });
});
