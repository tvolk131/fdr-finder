import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import {useState, useEffect} from 'react';
import SearchBar from '../components/searchBar';
import ShowCard, {ShowInfo} from '../components/showCard';
import {getPodcastRssUrl, searchPodcasts, generateUrlWithQueryParams} from '../api';
import {Button, CircularProgress, Dialog, DialogActions, DialogTitle, Snackbar} from '@material-ui/core';
import {CopyToClipboard} from 'react-copy-to-clipboard';
import {PieChart as PieChartIcon, RssFeed as RssFeedIcon} from '@material-ui/icons';
import {useHistory} from 'react-router';
import * as qs from 'qs';
import {ZoomableIcicle} from '../components/zoomableIcicle';
import {ZoomableCirclePacking} from '../components/zoomableCirclePacking';
import {ZoomableSunburst} from '../components/zoomableSunburst';
import {createTree} from '../helper';

const queryFieldName = 'query';
const tagsFieldName = 'tags';

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
  let tags = params[tagsFieldName];
  if (typeof query !== 'string') {
    query = '';
  }
  if (typeof tags === 'string') {
    tags = tags.split(',');
  } else {
    tags = ([] as string[]);
  }

  const [isSearching, setIsSearching] = useState(false);
  const [podcasts, setPodcasts] = useState([] as ShowInfo[]);
  const [searchTerm, setSearchTerm] = useState(query);
  const [searchTags, setSearchTags] = useState<string[]>(tags);
  const [showSnackbar, setShowSnackbar] = useState(false);

  const [showVisualizationDialog, setShowVisualizationDialog] = useState(false);
  const [visualizationFormat, setVisualizationFormat] = useState<'circlePacking' | 'sunburst' | 'icicle'>('circlePacking');

  const search = async () => {
    if (!isSearching) {
      const urlParams: {[key: string]: string} = {};
      urlParams[queryFieldName] = searchTerm;
      urlParams[tagsFieldName] = searchTags.join(',');
      const newLocation = generateUrlWithQueryParams('/', urlParams);
      if (newLocation !== `${history.location.pathname}${history.location.search}`) {
        history.push(newLocation);
      }
      setIsSearching(true);
      setPodcasts(await searchPodcasts({query: searchTerm, tags: searchTags}).finally(() => setIsSearching(false)));
    }
  };

  // If a search term was loaded from query parameter, immediately pull the results.
  useEffect(() => {
    if (searchTerm.length) {
      search();
    }
  }, []);

  useEffect(() => {
    if (searchTerm.length || searchTags.length) {
      search();
    } else {
      setPodcasts([]);
    }
  }, [searchTags]);

  return (
    <div className={classes.root}>
      <div className={classes.nested}>
        <SearchBar
          onSearch={search}
          searchText={searchTerm}
          setSearchText={setSearchTerm}
          searchTags={searchTags}
          setSearchTags={setSearchTags}
        />
      </div>
      {isSearching ? <CircularProgress className={classes.loadingSpinner} size={100}/> :
        <div>
          <div className={classes.button}>
            <CopyToClipboard
              text={getPodcastRssUrl({query: searchTerm})}
              onCopy={() => setShowSnackbar(true)}
            >
              <Button variant={'contained'} startIcon={<RssFeedIcon/>}>
                Copy Search-Filtered RSS Feed
              </Button>
            </CopyToClipboard>
          </div>
          <div className={classes.button}>
            <Button onClick={() => setShowVisualizationDialog(true)} variant={'contained'} startIcon={<PieChartIcon/>}>
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
          <Dialog
            onClose={() => setShowVisualizationDialog(false)}
            open={showVisualizationDialog}
            maxWidth={'xl'}
            fullWidth
          >
            <DialogTitle>Results for '{searchTerm}'</DialogTitle>
            <DialogActions>
              <Button onClick={() => setVisualizationFormat('circlePacking')} disabled={visualizationFormat === 'circlePacking'}>
                Circle Packing
              </Button>
              <Button onClick={() => setVisualizationFormat('sunburst')} disabled={visualizationFormat === 'sunburst'}>
                Sunburst
              </Button>
              <Button onClick={() => setVisualizationFormat('icicle')} disabled={visualizationFormat === 'icicle'}>
                Icicle
              </Button>
            </DialogActions>
            {
              visualizationFormat === 'circlePacking' && <ZoomableCirclePacking
                size={975}
                data={createTree(podcasts, [
                  {getValue: (podcast) => `${podcast.createTime.getUTCFullYear()}`}
                ])}
              />
            }
            {
              visualizationFormat === 'sunburst' && <ZoomableSunburst
                size={975}
                data={createTree(podcasts, [
                  {getValue: (podcast) => `${podcast.createTime.getUTCFullYear()}`}
                ])}
              />
            }
            {
              visualizationFormat === 'icicle' && <ZoomableIcicle
                height={600}
                width={975}
                showValue={false}
                data={createTree(podcasts, [
                  {getValue: (podcast) => `${podcast.createTime.getUTCFullYear()}`}
                ])}
              />
            }
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