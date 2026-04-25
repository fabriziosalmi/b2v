import { Decoder } from './decoder';

describe('Decoder', () => {
  const decoder = new Decoder();

  it('should correctly decode a valid input', () => {
    const encodedData = 'SGVsbG8gV29ybGQ=';
    const decodedText = decoder.decode(encodedData);

    expect(decodedText).toBe('Hello World');
  });

  it('should throw an error for invalid input', () => {
    expect(() => decoder.decode(null)).toThrow('Invalid input');
  });

  it('should handle empty input gracefully', () => {
    const emptyResult = decoder.decode('');
    expect(emptyResult).toBe('');
  });

  it('should handle decoding of a specific known pattern', () => {
    const input = 'test_input';
    const output = decoder.encode(input);
    expect(decoder.decode(output)).toBe(input);
  });
});