import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import {useState, useEffect} from 'react';
import SearchBar from '../components/searchBar';
import ShowCard, {ShowInfo} from '../components/showCard';
import {getPodcastRssUrl, getPodcasts} from '../api';
import {Button, CircularProgress, Dialog, DialogContent, DialogTitle, Snackbar} from '@material-ui/core';
import {CopyToClipboard} from 'react-copy-to-clipboard';
import {PieChart as PieChartIcon, RssFeed as RssFeedIcon} from '@material-ui/icons';
import {useHistory} from 'react-router';
import * as qs from 'qs';
import {ZoomableIcicle} from '../components/zoomableIcicle';
import {createTree} from '../helper';

const queryFieldName = 'query';

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
  button: {
    padding: '10px 0 0 0'
  },
  loadingSpinner: {
    padding: '50px'
  }
});

interface SearchPageProps {
  setPlayingShow(showInfo: ShowInfo): void
}

export const SearchPage = (props: SearchPageProps) => {
  const classes = useStyles();
  const history = useHistory();

  const params = qs.parse(history.location.search.replace('?', ''));
  let query = params[queryFieldName];
  if (typeof query !== 'string') {
    query = '';
  }

  const [isSearching, setIsSearching] = useState(false);
  const [podcasts, setPodcasts] = useState([] as ShowInfo[]);
  const [searchTerm, setSearchTerm] = useState(query);
  const [showSnackbar, setShowSnackbar] = useState(false);

  const [showIcicleDialog, setShowIcicleDialog] = useState(false);

  const search = async () => {
    if (!isSearching) {
      setIsSearching(true);
      history.push(`/?${queryFieldName}=${searchTerm}`);
      setPodcasts(await getPodcasts(searchTerm, 50, 0));
      setIsSearching(false);
    }
  };

  // If a search term was loaded from query parameter, immediately pull the results.
  useEffect(() => {
    if (searchTerm.length) {
      search();
    }
  }, []);

  return (
    <div className={classes.root}>
      <div className={classes.nested}>
        <SearchBar
          onSearch={search}
          searchText={searchTerm}
          setSearchText={setSearchTerm}
        />
      </div>
      {isSearching ? <CircularProgress className={classes.loadingSpinner} size={100}/> :
        <div>
          <div className={classes.button}>
            <CopyToClipboard
              text={getPodcastRssUrl(searchTerm)}
              onCopy={() => setShowSnackbar(true)}
            >
              <Button variant={'contained'} startIcon={<RssFeedIcon/>}>
                Copy Search-Filtered RSS Feed
              </Button>
            </CopyToClipboard>
          </div>
          <div className={classes.button}>
            <Button onClick={() => setShowIcicleDialog(true)} variant={'contained'} startIcon={<PieChartIcon/>}>
              See Visualized Results
            </Button>
          </div>
          <div className={classes.nested}>
            {
              podcasts.map((show) => (
                <div className={classes.showCardWrapper}>
                  <ShowCard onPlay={() => props.setPlayingShow(show)} show={show}/>
                </div>
              ))
            }
          </div>
          <Dialog onClose={() => setShowIcicleDialog(false)} open={showIcicleDialog} maxWidth={'xl'} fullWidth>
            <DialogTitle>Results for '{searchTerm}'</DialogTitle>
            <ZoomableIcicle
              height={600}
              width={975}
              showValue={false}
              data={createTree(podcasts, [
                {getValue: (podcast) => `${podcast.createTime.getUTCFullYear()}`},
                {getValue: (podcast) => podcast.createTime.toLocaleString('default', { month: 'long' })}
              ])}
            />
          </Dialog>
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