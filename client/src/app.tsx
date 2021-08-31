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
import {Snackbar} from '@material-ui/core';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      backgroundColor: theme.palette.background.default,
      display: 'flex',
      flexDirection: 'column',
      minHeight: '100vh'
    },
    pageContent: {
      flex: 1
    }
  })
);

const SubApp = () => {
  const classes = useStyles();

  const [playingShow, setPlayingShow] = useState<ShowInfo>();
  const [showSnackbar, setShowSnackbar] = useState(false);
  const [snackbarMessage, setSnackbarMessage] = useState('');

  const showSnackbarMessage = (message: string) => {
    setShowSnackbar(true);
    setSnackbarMessage(message);
  };

  return (
    <div className={classes.root}>
      {/* This meta tag makes the mobile experience
      much better by preventing text from being tiny. */}
      <meta name='viewport' content='width=device-width, initial-scale=1.0'/>
      <div className={classes.pageContent}>
        <BrowserRouter>
          <Switch>
            <Route exact path='/'>
              <SearchPage setPlayingShow={setPlayingShow} showSnackbarMessage={showSnackbarMessage}/>
            </Route>
            <Route exact path='/podcasts/:podcastNum'>
              <PodcastPage setPlayingShow={setPlayingShow}/>
            </Route>
            <Route path='*'>
              <NotFoundPage/>
            </Route>
          </Switch>
        </BrowserRouter>
      </div>
      <AudioPlayer showInfo={playingShow} autoPlay={true} showSnackbarMessage={showSnackbarMessage}/>
      <Snackbar
        anchorOrigin={{
          vertical: 'bottom',
          horizontal: 'left'
        }}
        open={showSnackbar}
        autoHideDuration={6000}
        onClose={(event, reason) => {
          if (reason !== 'clickaway') {
            setShowSnackbar(false);
          }
        }}
        message={snackbarMessage}
      />
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