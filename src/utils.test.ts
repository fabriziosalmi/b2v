describe('Utils', () => {
  describe('string utilities', () => {
    it('should trim whitespace from strings', () => {
      expect(trimWhitespace('  hello  ')).toBe('hello');
      expect(trimWhitespace('   ')).toBe('');
      expect(trimWhitespace('')).toBe('');
    });

    it('should convert camelCase to snake_case', () => {
      expect(convertToSnakeCase('camelCase')).toBe('camel_case');
      expect(convertToSnakeCase('already_snake_case')).toBe('already_snake_case');
      expect(convertToSnakeCase('single')).toBe('single');
    });

    it('should format numbers with commas', () => {
      expect(formatNumber(1000)).toBe('1,000');
      expect(formatNumber(1234567)).toBe('1,234,567');
      expect(formatNumber(0)).toBe('0');
    });

    it('should handle null values gracefully', () => {
      expect(handleNullValues(null)).toBe('N/A');
      expect(handleNullValues(undefined)).toBe('N/A');
      expect(handleNullValues(5)).toBe('5');
    });
  });

  describe('array utilities', () => {
    it('should safely access array elements', () => {
      expect(safeArrayAccess([1, 2, 3], 0)).toBe(1);
      expect(safeArrayAccess([1, 2, 3], 5)).toBe(undefined);
      expect(safeArrayAccess([], 0)).toBe(undefined);
    });

    it('should merge arrays without duplicates', () => {
      expect(mergeUniqueArrays([1, 2, 3], [3, 4, 5])).toEqual([1, 2, 3, 4, 5]);
      expect(mergeUniqueArrays([1, 2], [1, 2])).toEqual([1, 2]);
      expect(mergeUniqueArrays([], [1, 2])).toEqual([1, 2]);
    });
  });
});