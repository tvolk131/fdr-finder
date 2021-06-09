import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import {useState} from 'react';
import SearchBar from '../components/searchBar';
import ShowCard, {ShowInfo} from '../components/showCard';
import {getPodcastRssUrl, getPodcasts} from '../api';
import {Button, CircularProgress, Snackbar} from '@material-ui/core';
import {CopyToClipboard} from 'react-copy-to-clipboard';
import {RssFeed as RssFeedIcon} from '@material-ui/icons';

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
  },
  rssButton: {
    padding: '10px 0 0 0'
  },
  loadingSpinner: {
    padding: '50px'
  }
});

const SearchPage = () => {
  const classes = useStyles();

  const [isSearching, setIsSearching] = useState(false);
  const [podcasts, setPodcasts] = useState([] as ShowInfo[]);
  const [searchTerm, setSearchTerm] = useState('');
  const [showSnackbar, setShowSnackbar] = useState(false);

  return (
    <div className={classes.root}>
      <div className={classes.nested}>
        <SearchBar onSearch={async (query) => {
          if (!isSearching) {
            setIsSearching(true);
            setPodcasts(await getPodcasts(query, 50, 0));
            setSearchTerm(query);
            setIsSearching(false);
          }
        }}/>
      </div>
      {isSearching ? <CircularProgress className={classes.loadingSpinner} size={100}/> :
        <div>
          <div className={classes.rssButton}>
            <CopyToClipboard
              text={getPodcastRssUrl(searchTerm)}
              onCopy={() => setShowSnackbar(true)}
            >
              <Button variant={'contained'} startIcon={<RssFeedIcon/>}>
                Copy Search-Filtered RSS Feed
              </Button>
            </CopyToClipboard>
          </div>
          <div className={classes.nested}>
            {podcasts.map((show) => <div className={classes.showCardWrapper}><ShowCard show={show}/></div>)}
          </div>
        </div>
      }
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
        message={'Link copied!'}
      />
    </div>
  );
};

export default SearchPage;