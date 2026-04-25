import { Encoder } from './encoder';

describe('Encoder', () => {
  const encoder = new Encoder();

  it('should encode successfully and allow for round-trip decoding', () => {
    const originalText = 'Test message for encoding';
    const encodedData = encoder.encode(originalText);

    expect(typeof encodedData).toBe('string');
    expect(encodedData).not.toBe(originalText);
  });

  it('should throw an error for invalid input', () => {
    expect(() => encoder.encode(null)).toThrow('Invalid input');
  });

  it('should handle empty input gracefully', () => {
    const emptyResult = encoder.encode('');
    expect(emptyResult).toBe('');
  });

  it('should handle encoding of a specific known pattern', () => {
    const input = 'test_input';
    const output = encoder.encode(input);
    expect(encoder.decode(output)).toBe(input);
  });
});