import {blue, teal} from '@mui/material/colors';
import {
  createTheme,
  ThemeProvider,
  Theme
} from '@mui/material/styles';
import {createStyles, makeStyles} from '@mui/styles';
import * as React from 'react';
import {useState} from 'react';
import {Route, Routes} from 'react-router';
import {BrowserRouter} from 'react-router-dom';
import {SearchPage} from './pages/searchPage';
import {NotFoundPage} from './pages/notFoundPage';
import {PodcastPage} from './pages/podcastPage';
import {AudioPlayer} from './components/audioPlayer';
import {ShowInfo} from './components/showCard';
import {Box, Snackbar} from '@mui/material';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
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
    <Box sx={{backgroundColor: 'background.default', color: 'text.primary'}} className={classes.root}>
      {/* This meta tag makes the mobile experience
      much better by preventing text from being tiny. */}
      <meta name='viewport' content='width=device-width, initial-scale=1.0'/>
      <div className={classes.pageContent}>
        <BrowserRouter>
          <Routes>
            <Route path='/' element={<SearchPage setPlayingShow={setPlayingShow} showSnackbarMessage={showSnackbarMessage}/>}/>
            <Route path='/podcasts/:podcastNum' element={<PodcastPage setPlayingShow={setPlayingShow}/>}/>
            <Route path='*' element={<NotFoundPage/>}/>
          </Routes>
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
    </Box>
  );
};

const ThemedSubApp = () => {
  const isDarkMode = true; // TODO - Add a way for users to be able to set this.

  const theme = createTheme({
    palette: {
      primary: blue,
      secondary: teal,
      mode: isDarkMode ? 'dark' : 'light'
    }
  });

  return (
    <ThemeProvider theme={theme}>
      <SubApp/>
    </ThemeProvider>
  );
};

export const App = () => (
  <ThemedSubApp/>
);