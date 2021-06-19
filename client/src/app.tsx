import {blue, teal} from '@material-ui/core/colors';
import {
  createMuiTheme,
  MuiThemeProvider,
  makeStyles,
  createStyles,
  Theme
} from '@material-ui/core/styles';
import * as React from 'react';
import {useState} from 'react';
import {Route, Switch} from 'react-router';
import {BrowserRouter} from 'react-router-dom';
import {SearchPage} from './pages/searchPage';
import {NotFoundPage} from './pages/notFoundPage';
import {PodcastPage} from './pages/podcastPage';
import {AudioPlayer} from './components/audioPlayer';
import {ShowInfo} from './components/showCard';
import {IciclePage} from './pages/iciclePage';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      backgroundColor: theme.palette.background.default,
      height: '100%',
      overflowY: 'auto'
    }
  })
);

const SubApp = () => {
  const classes = useStyles();

  const [playingShow, setPlayingShow] = useState<ShowInfo>();

  return (
    <div className={classes.root}>
      {/* This meta tag makes the mobile experience
      much better by preventing text from being tiny. */}
      <meta name='viewport' content='width=device-width, initial-scale=1.0'/>
      <BrowserRouter>
        <Switch>
          <Route exact path='/'>
            <SearchPage setPlayingShow={setPlayingShow}/>
          </Route>
          <Route exact path='/podcasts/:podcastNum'>
            <PodcastPage setPlayingShow={setPlayingShow}/>
          </Route>
          <Route exact path='/visualization/icicle'>
            <IciclePage/>
          </Route>
          <Route path='*'>
            <NotFoundPage/>
          </Route>
        </Switch>
      </BrowserRouter>
      <AudioPlayer showInfo={playingShow} autoPlay={true}/>
    </div>
  );
};

const ThemedSubApp = () => {
  const isDarkMode = true; // TODO - Add a way for users to be able to set this.

  const theme = createMuiTheme({
    palette: {
      primary: blue,
      secondary: teal,
      type: isDarkMode ? 'dark' : 'light'
    },
    props: {
      MuiAppBar: {
        color: isDarkMode ? 'default' : 'primary'
      },
      MuiTypography: {
        color: 'textPrimary'
      }
    }
  });

  return (
    <MuiThemeProvider theme={theme}>
      <SubApp/>
    </MuiThemeProvider>
  );
};

export const App = () => (
  <ThemedSubApp/>
);