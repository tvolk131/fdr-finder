import {secondsToDurationString} from './secondsToDurationString';

describe('secondsToDurationString', () => {
  it('handles negative-value input', () => {
    expect(secondsToDurationString(-1)).toEqual('0 minutes');
  });

  it('handles zero-value input', () => {
    expect(secondsToDurationString(0)).toEqual('0 minutes');
  });

  it('returns correct value', () => {
    expect(secondsToDurationString(59)).toEqual('0 minutes');

    expect(secondsToDurationString(60)).toEqual('1 minute');
    expect(secondsToDurationString(120)).toEqual('2 minutes');

    expect(secondsToDurationString(3600)).toEqual('1 hour');
    expect(secondsToDurationString(7200)).toEqual('2 hours');

    expect(secondsToDurationString(3660)).toEqual('1 hour 1 minute');
    expect(secondsToDurationString(3720)).toEqual('1 hour 2 minutes');

    expect(secondsToDurationString(7260)).toEqual('2 hours 1 minute');
    expect(secondsToDurationString(7320)).toEqual('2 hours 2 minutes');
  });
});