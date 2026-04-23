// This file is automatically formatted by Prettier
// Please ensure your code adheres to the project's style guide

import { describe, test, expect } from '@jest/globals';
import * as decoder from './decoder';

describe('decoder', () => {
  test('should decode correctly', () => {
    expect(decoder.decode('test')).toBe('test');
  });
});