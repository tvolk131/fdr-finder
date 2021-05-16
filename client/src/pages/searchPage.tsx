import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import SearchBar from '../components/searchBar';
import ShowCard, {ShowFormat, ShowInfo} from '../components/showCard';

const useStyles = makeStyles({
  root: {
    margin: '10px',
    textAlign: 'center'
  },
  nested: {
    maxWidth: 800,
    margin: 'auto',
    textAlign: 'initial'
  },
  showCardWrapper: {
    padding: '10px 0 0 0'
  }
});

const shows: ShowInfo[] = [
  {
    title: 'BILL GATES DIVORCE! Freedomain Livestream',
    description: 'Philosopher Stefan Molyneux unpacks the morality of the divorce between Bill Gates and Melinda Gates - and utterly fails to help a listener overcome his nihilism!\n\nFree Documentaries: https://www.freedomain.com/documentaries\n\nFreedomain NFTs: www.freedomainnft.com\n\nFree novel: https://www.freedomain.com/almost\n\n▶️ Donate Now: https://www.freedomain.com/donate\n▶️ Sign Up For Our Newsletter: https://www.fdrurl.com/newsletter\n\nYour support is essential to Freedomain, which is 100% funded by viewers like you. Please support the show by making a one time donation or signing up for a monthly recurring donation at: www.freedomain.com/donate\n\n▶️ 1. Donate: https://www.freedomain.com/donate\n▶️ 2. Newsletter Sign-Up: https://www.fdrurl.com/newsletter\n▶️ 3. Subscribe to the Freedomain Podcast: https://www.fdrpodcasts.com\n▶️ 4. Follow Freedomain on Alternative Platforms:\n\nVideo:\n🔴 DLive Livestream: https://dlive.tv/freedomain\n🔴 Bitchute: https://bitchute.com/freedomainradio\n🔴 Rumble: https://rumble.com/freedomain\n🔴 LBRY: https://open.lbry.com/@freedomain:b\n🔴 Streamanity: https://fdrurl.com/streamanity\n🔴 Locals: https://freedomain.locals.com\n🔴 Brighteon: https://brighteon.com/channels/freedomain\n🔴 DailyMotion: https://dailymotion.com/FreedomainRadio\n\n🔴 Parler: https://parler.com/profile/stefanmolyneux\n🔴 Minds: https://minds.com/stefanmolyneux\n🔴 Steemit: https://steemit.com/@stefan.molyneux\n🔴 Gab: https://gab.ai/stefanmolyneux\n🔴 Instagram: https://instagram.com/stefanmolyneux\n🔴 PocketNet: https://pocketnet.app/freedomain\n🔴 MeWe: https://mewe.com/i/freedomain\n🔴 Twetch: https://www.fdrurl.com/twetch\n🔴 Thinkspot: https://www.fdrurl.com/thinkspot\n🔴 Flote: https://flote.app/freedomain\n🔴 Pinterest: https://www.pinterest.com/stefanfreedomain',
    audioLink: 'https://cdn.freedomainradio.com/FDR_4844_wed_night_live_12_may_2021_BILL_GATES.mp3',
    lengthInSeconds: 9865,
    podcastNumber: 4844,
    createTime: new Date(1620943200 * 1000),
    showFormat: ShowFormat.Livestream
  },
  {
    title: 'THE WORLD OF ALT COINS! Freedomain Roundtable',
    description: 'A Freedomain roundtable examination of the world of alt-coins!\n\nFree Documentaries: https://www.freedomain.com/documentaries\n\nFreedomain NFTs: www.freedomainnft.com\n\nFree novel: https://www.freedomain.com/almost\n\n▶️ Donate Now: https://www.freedomain.com/donate\n▶️ Sign Up For Our Newsletter: https://www.fdrurl.com/newsletter\n\nYour support is essential to Freedomain, which is 100% funded by viewers like you. Please support the show by making a one time donation or signing up for a monthly recurring donation at: www.freedomain.com/donate\n\n▶️ 1. Donate: https://www.freedomain.com/donate\n▶️ 2. Newsletter Sign-Up: https://www.fdrurl.com/newsletter\n▶️ 3. Subscribe to the Freedomain Podcast: https://www.fdrpodcasts.com\n▶️ 4. Follow Freedomain on Alternative Platforms:\n\nVideo:\n🔴 DLive Livestream: https://dlive.tv/freedomain\n🔴 Bitchute: https://bitchute.com/freedomainradio\n🔴 Rumble: https://rumble.com/freedomain\n🔴 LBRY: https://open.lbry.com/@freedomain:b\n🔴 Streamanity: https://fdrurl.com/streamanity\n🔴 Locals: https://freedomain.locals.com\n🔴 Brighteon: https://brighteon.com/channels/freedomain\n🔴 DailyMotion: https://dailymotion.com/FreedomainRadio\n\n🔴 Parler: https://parler.com/profile/stefanmolyneux\n🔴 Minds: https://minds.com/stefanmolyneux\n🔴 Steemit: https://steemit.com/@stefan.molyneux\n🔴 Gab: https://gab.ai/stefanmolyneux\n🔴 Instagram: https://instagram.com/stefanmolyneux\n🔴 PocketNet: https://pocketnet.app/freedomain\n🔴 MeWe: https://mewe.com/i/freedomain\n🔴 Twetch: https://www.fdrurl.com/twetch\n🔴 Thinkspot: https://www.fdrurl.com/thinkspot\n🔴 Flote: https://flote.app/freedomain\n🔴 Pinterest: https://www.pinterest.com/stefanfreedomain',
    audioLink: 'https://cdn.freedomainradio.com/FDR_4843_crypto_call_in_11_May_2021.mp3',
    lengthInSeconds: 6500,
    podcastNumber: 4843,
    createTime: new Date(1620789600 * 1000),
    showFormat: ShowFormat.Roundtable
  },
  {
    title: 'LOCKDOWNS 282 TIMES WORSE?',
    description: 'Professor Explains Flaw in Many Models Used for COVID-19 Lockdown Policies\"\n\nhttps://www.theepochtimes.com/professor-explains-flaw-in-many-models-used-for-covid-lockdown-policies_3807048.html\n\nFree Documentaries: https://www.freedomain.com/documentaries\n\nFreedomain NFTs: www.freedomainnft.com\n\nFree novel: https://www.freedomain.com/almost\n\n▶️ Donate Now: https://www.freedomain.com/donate\n▶️ Sign Up For Our Newsletter: https://www.fdrurl.com/newsletter\n\nYour support is essential to Freedomain, which is 100% funded by viewers like you. Please support the show by making a one time donation or signing up for a monthly recurring donation at: www.freedomain.com/donate\n\n▶️ 1. Donate: https://www.freedomain.com/donate\n▶️ 2. Newsletter Sign-Up: https://www.fdrurl.com/newsletter\n▶️ 3. Subscribe to the Freedomain Podcast: https://www.fdrpodcasts.com',
    audioLink: 'https://cdn.freedomainradio.com/FDR_4842_lockdowns_282_times_worse.mp3',
    lengthInSeconds: 1042,
    podcastNumber: 4842,
    createTime: new Date(1620760500 * 1000),
    showFormat: ShowFormat.SoloPodcast
  },
  {
    title: 'HOW TO ESCAPE TRAUMA!',
    description: 'Philosopher Stefan Molyneux teaches you how to love women, escape trauma and build a joyful future!\n\nHAPPY MOTHER\'S DAY!\n\nwww.freedomain.com/donate',
    audioLink: 'https://cdn.freedomainradio.com/FDR_4841_mothers_day_2021.mp3',
    lengthInSeconds: 4284,
    podcastNumber: 4841,
    createTime: new Date(1620598860 * 1000),
    showFormat: ShowFormat.SoloPodcast
  },
  {
    title: 'Stefan Molyneux: Wednesday Night Live 5 5 2021',
    description: 'Free Documentaries: https://www.freedomain.com/documentaries\n\nFreedomain NFTs: www.freedomainnft.com\n\nFree novel: https://www.freedomain.com/almost\n\n▶️ Donate Now: https://www.freedomain.com/donate\n▶️ Sign Up For Our Newsletter: https://www.fdrurl.com/newsletter\n\nYour support is essential to Freedomain, which is 100% funded by viewers like you. Please support the show by making a one time donation or signing up for a monthly recurring donation at: www.freedomain.com/donate\n\n▶️ 1. Donate: https://www.freedomain.com/donate\n▶️ 2. Newsletter Sign-Up: https://www.fdrurl.com/newsletter\n▶️ 3. Subscribe to the Freedomain Podcast: https://www.fdrpodcasts.com\n▶️ 4. Follow Freedomain on Alternative Platforms:\n\nVideo:\n🔴 DLive Livestream: https://dlive.tv/freedomain\n🔴 Bitchute: https://bitchute.com/freedomainradio\n🔴 Rumble: https://rumble.com/freedomain\n🔴 LBRY: https://open.lbry.com/@freedomain:b\n🔴 Streamanity: https://fdrurl.com/streamanity\n🔴 Locals: https://freedomain.locals.com\n🔴 Brighteon: https://brighteon.com/channels/freedomain\n🔴 DailyMotion: https://dailymotion.com/FreedomainRadio\n\n🔴 Parler: https://parler.com/profile/stefanmolyneux\n🔴 Minds: https://minds.com/stefanmolyneux\n🔴 Steemit: https://steemit.com/@stefan.molyneux\n🔴 Gab: https://gab.ai/stefanmolyneux\n🔴 Instagram: https://instagram.com/stefanmolyneux\n🔴 PocketNet: https://pocketnet.app/freedomain\n🔴 MeWe: https://mewe.com/i/freedomain\n🔴 Twetch: https://www.fdrurl.com/twetch\n🔴 Thinkspot: https://www.fdrurl.com/thinkspot\n🔴 Flote: https://flote.app/freedomain\n🔴 Pinterest: https://www.pinterest.com/stefanfreedomain',
    audioLink: 'https://cdn.freedomainradio.com/FDR_4840_wed_night_live_5_5_2021.mp3',
    lengthInSeconds: 9983,
    podcastNumber: 4840,
    createTime: new Date(1620320220 * 1000),
    showFormat: ShowFormat.SoloPodcast
  }
];

const SearchPage = () => {
  const classes = useStyles();

  return (
    <div className={classes.root}>
      <div className={classes.nested}>
        <SearchBar onSearch={(query) => console.log(query)}/>
      </div>
      <div className={classes.nested}>
        {shows.map((show) => <div className={classes.showCardWrapper}><ShowCard show={show}/></div>)}
      </div>
    </div>
  );
};

export default SearchPage;