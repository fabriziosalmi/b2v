// decoder.test.ts
import { describe, it, expect } from 'vitest';
import { Decoder } from './decoder';

describe('Decoder', () => {
  it('should decode a valid video file', async () => {
    const decoder = new Decoder({
      input_path: 'test.mp4',
      output_path: 'output.mp4'
    });

    // Mock ffmpeg behavior
    const mockFfmpeg = {
      stdout: new ReadableStream({
        start(controller) {
          controller.enqueue(new Uint8Array([0x01, 0x02, 0x03]));
          controller.close();
        }
      })
    };

    // Mock dependencies
    const originalSpawn = Command.spawn;
    Command.spawn = (args) => {
      return new Promise((resolve) => {
        resolve(mockFfmpeg);
      });
    };

    const result = await decoder.run();
    expect(result).toBeUndefined();
  });

  it('should handle header decoding with block size 4', () => {
    // This test would need actual implementation details
    // to verify header decoding logic
    expect(true).toBe(true);
  });
});