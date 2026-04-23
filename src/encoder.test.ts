// This file is automatically formatted by Prettier
// Please ensure your code adheres to the project's style guide

import { describe, test, expect } from '@jest/globals';
import * as encoder from './encoder';

describe('encoder', () => {
  test('should encode correctly', () => {
    expect(encoder.encode('test')).toBe('test');
  });
});