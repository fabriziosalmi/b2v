describe('Utils', () => {
  it('should perform basic utility calculations', () => {
    expect(1 + 1).toBe(2);
  });

  it('should handle string trimming', () => {
    expect(utils.trimString('  hello  ')).toBe('hello');
    expect(utils.trimString('')).toBe('');
  });

  it('should calculate file size in MB', () => {
    expect(utils.calculateFileSizeInMB(1024 * 1024)).toBe(1);
    expect(utils.calculateFileSizeInMB(1023 * 1024)).toBe(1);
    expect(utils.calculateFileSizeInMB(0)).toBe(0);
  });

  it('should format bytes to human-readable string', () => {
    expect(utils.formatBytes(1024)).toBe('1.00 KB');
    expect(utils.formatBytes(1024 * 1024)).toBe('1.00 MB');
    expect(utils.formatBytes(1024 * 1024 * 1024)).toBe('1.00 GB');
    expect(utils.formatBytes(123456789)).toBe('117.70 MB');
  });
});