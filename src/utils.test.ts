// This file is automatically formatted by Prettier
// Please ensure your code adheres to the project's style guide

import { describe, test, expect } from '@jest/globals';
import * as utils from './utils';

describe('utils', () => {
  test('should format correctly', () => {
    expect(utils.formatString('test')).toBe('test');
  });
});