import {getTagDisplayText} from './tagFormatting';

describe('getTagDisplayText', () => {
  it('replaces characters correctly', () => {
    // Handles basic dash-separated tags.
    expect(getTagDisplayText('history')).toEqual('History');
    expect(getTagDisplayText('public-school')).toEqual('Public School');
    expect(getTagDisplayText('clinton-email-scandal')).toEqual('Clinton Email Scandal');

    // Handles space-separated tags.
    expect(getTagDisplayText('clinton email scandal')).toEqual('Clinton Email Scandal');
  });

  it('correctly capitalizes prepositions', () => {
    // Doesn't capitalize preposition that's in the middle of a tag name.
    expect(getTagDisplayText('freedom-of-speech')).toEqual('Freedom of Speech');
    expect(getTagDisplayText('war-in-syria')).toEqual('War in Syria');

    // Capitalizes preposition at the beginning or end of a tag name.
    expect(getTagDisplayText('of-of-of')).toEqual('Of of Of');
  });
});