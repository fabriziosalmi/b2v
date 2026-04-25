// This test file is auto-generated and should not be modified manually
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

  // Additional edge cases
  it('should handle special characters in input', () => {
    const input = 'Special!@#$%^&*() characters';
    const output = decoder.encode(input);
    expect(decoder.decode(output)).toBe(input);
  });

  it('should handle decoding of non-ASCII characters', () => {
    const input = 'Non-ASCII: éçàñ';
    const output = decoder.encode(input);
    expect(decoder.decode(output)).toBe(input);
  });

  it('should throw error for excessively long input', () => {
    const longInput = 'a'.repeat(10000);
    expect(() => decoder.decode(longInput)).toThrow('Input too long');
  });

  it('should handle decoding of mixed data types', () => {
    const input = 'Mixed data: 123, true, null';
    const output = decoder.encode(input);
    expect(decoder.decode(output)).toBe(input);
  });
});