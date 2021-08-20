import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import {useState, useEffect, useRef} from 'react';
import SearchBar from '../components/searchBar';
import ShowCard, {ShowInfo} from '../components/showCard';
import {getPodcastRssUrl, searchPodcasts, generateUrlWithQueryParams, getRecentPodcasts, getFilteredTagsWithCounts} from '../api';
import {
  Button,
  CircularProgress,
  Dialog,
  DialogActions,
  DialogTitle,
  Snackbar,
  TablePagination,
  Select,
  MenuItem,
  Typography
} from '@material-ui/core';
import {CopyToClipboard} from 'react-copy-to-clipboard';
import {PieChart as PieChartIcon, RssFeed as RssFeedIcon} from '@material-ui/icons';
import {useHistory} from 'react-router';
import * as qs from 'qs';
import {ZoomableIcicle} from '../components/zoomableIcicle';
import {ZoomableCirclePacking} from '../components/zoomableCirclePacking';
import {ZoomableSunburst} from '../components/zoomableSunburst';
import {createTree} from '../helper';
import {queryFieldName, tagsFieldName} from '../constants';
import {BehaviorSubject, map, switchMap, distinctUntilChanged, merge, of} from 'rxjs';

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
  },
  paginator: {
    width: 'fit-content',
    margin: 'auto'
  },
  paginatorToolbar: {
    paddingTop: '10px'
  },
  sortSelectorText: {
    display: 'inline',
    paddingRight: '5px'
  },
  sortSelectorWrapper: {
    paddingTop: '18px'
  }
});

interface SearchPageProps {
  setPlayingShow(showInfo: ShowInfo): void
}

// TODO - Cleanup this component. Over time it has become a bit of a convoluted mess of tacked-on functionality.
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
  const [podcastPage, setPodcastPage] = useState(0);
  const [podcastsPerPage, setPodcastsPerPage] = useState(100);
  const [podcastSortDirection, setPodcastSortDirection] = useState<'podcastNumber desc' | 'podcastNumber asc'>('podcastNumber desc');
  const [searchTerm, setSearchTerm] = useState(query);
  const [searchTags, setSearchTags] = useState<string[]>(tags);
  const [showSnackbar, setShowSnackbar] = useState(false);

  const [showRecentPodcasts, setShowRecentPodcasts] = useState(query.length === 0 && tags.length === 0);
  const [recentPodcasts, setRecentPodcasts] = useState([] as ShowInfo[]);
  const [isLoadingRecentPodcasts, setIsLoadingRecentPodcasts] = useState(true);

  const [showVisualizationDialog, setShowVisualizationDialog] = useState(false);
  const [visualizationFormat, setVisualizationFormat] = useState<'circlePacking' | 'sunburst' | 'icicle'>('circlePacking');

  const [tagsWithCounts, setTagsWithCounts] = useState<{tag: string, count: number}[]>([]);
  const [isLoadingTagsWithCounts, setIsLoadingTagsWithCounts] = useState(false);

  const subject = useRef(new BehaviorSubject({query: '', tags: [] as string[]}));

  useEffect(() => {
    setIsLoadingRecentPodcasts(true);
    getRecentPodcasts().then((podcasts) => {
      setRecentPodcasts(podcasts);
      setIsLoadingRecentPodcasts(false);
    });

    const observable = subject.current.pipe(
      map(({query, tags}) => ({query: query.trim(), tags})),
      distinctUntilChanged(),
      switchMap(({query, tags}) => merge(
        of({
          isLoadingPodcasts: true,
          isLoadingTagsWithCounts: true,
          podcasts: undefined,
          tagsWithCounts: undefined
        }),
        searchPodcasts({
          query,
          limit: podcastsPerPage,
          offset: 0,
          tags
        }).then((podcasts) => ({
          isLoadingPodcasts: false,
          isLoadingTagsWithCounts: undefined,
          podcasts,
          tagsWithCounts: undefined
        })),
        getFilteredTagsWithCounts({query, tags})
          .then((tagsWithCounts) => {
            tagsWithCounts.sort((a, b) => {
              if (a.count < b.count) {
                return 1;
              } else if (a.count > b.count) {
                return -1;
              } else if (a.tag > b.tag) {
                return 1;
              } else if (a.tag < b.tag) {
                return -1;
              } else {
                return 0;
              }
            });
            return {
              isLoadingPodcasts: undefined,
              isLoadingTagsWithCounts: false,
              podcasts: undefined,
              tagsWithCounts
            };
          })
      ))
    ).subscribe(({isLoadingPodcasts, isLoadingTagsWithCounts, podcasts, tagsWithCounts}) => {
      if (isLoadingPodcasts !== undefined) {
        setIsSearching(isLoadingPodcasts);
      }
      if (isLoadingTagsWithCounts !== undefined) {
        setIsLoadingTagsWithCounts(isLoadingTagsWithCounts);
      }
      if (podcasts != undefined) {
        setPodcasts(podcasts);
      }
      if (tagsWithCounts != undefined) {
        setTagsWithCounts(tagsWithCounts);
      }
    });

    return () => {
      observable.unsubscribe();
      subject.current.unsubscribe();
    };
  }, []);

  useEffect(() => {
    const urlParams: {[key: string]: string} = {};
    urlParams[queryFieldName] = searchTerm;
    urlParams[tagsFieldName] = searchTags.join(',');
    const newLocation = generateUrlWithQueryParams('/', urlParams);
    if (newLocation !== `${history.location.pathname}${history.location.search}`) {
      history.push(newLocation);
    }

    if (searchTerm.length || searchTags.length) {
      setShowRecentPodcasts(false);
    } else {
      setShowRecentPodcasts(true);
    }

    subject.current.next({query: searchTerm, tags: searchTags});
  }, [searchTerm, searchTags]);

  useEffect(() => {
    setPodcastPage(0);
  }, [podcastSortDirection]);

  const sortedPodcasts = (showRecentPodcasts ? recentPodcasts : podcasts).sort((podcastA, podcastB) => {
    if (podcastSortDirection === 'podcastNumber desc') {
      return podcastB.podcastNumber - podcastA.podcastNumber;
    } else {
      return podcastA.podcastNumber - podcastB.podcastNumber;
    }
  });

  const paginator = (
    <TablePagination
      component={'div'}
      count={sortedPodcasts.length}
      page={podcastPage}
      onPageChange={(event, newPage) => setPodcastPage(newPage)}
      rowsPerPage={podcastsPerPage}
      onRowsPerPageChange={(event) => {
        setPodcastsPerPage(parseInt(event.target.value, 10));
        setPodcastPage(0);
      }}
      className={classes.paginator}
      classes={{toolbar: classes.paginatorToolbar}}
    />
  );

  return (
    <div className={classes.root}>
      <div className={classes.nested}>
        <SearchBar
          searchText={searchTerm}
          setSearchText={setSearchTerm}
          searchTags={searchTags}
          setSearchTags={setSearchTags}
          tagsWithCounts={tagsWithCounts}
          isLoadingTagsWithCounts={isLoadingTagsWithCounts}
        />
      </div>
      {(showRecentPodcasts ? isLoadingRecentPodcasts : isSearching) ?
        <CircularProgress className={classes.loadingSpinner} size={100}/> :
        <div>
          <div className={classes.button}>
            <CopyToClipboard
              text={getPodcastRssUrl({query: searchTerm, tags: searchTags})}
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
          {!!sortedPodcasts.length &&
            <div className={classes.sortSelectorWrapper}>
              <Typography className={classes.sortSelectorText}>Sort by</Typography>
              <Select value={podcastSortDirection} onChange={(e) => setPodcastSortDirection(e.target.value as 'podcastNumber desc' | 'podcastNumber asc')} label={'Filter Podcasts'}>
                <MenuItem value={'podcastNumber desc'}>Newest</MenuItem>
                <MenuItem value={'podcastNumber asc'}>Oldest</MenuItem>
              </Select>
            </div>
          }
          {!!sortedPodcasts.length && paginator}
          <div className={classes.nested}>
            {
              sortedPodcasts.slice(podcastPage * podcastsPerPage, (podcastPage + 1) * podcastsPerPage).map((show) => (
                <div className={classes.showCardWrapper}>
                  <ShowCard onPlay={() => props.setPlayingShow(show)} show={show}/>
                </div>
              ))
            }
          </div>
          {!!sortedPodcasts.length && paginator}
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
                data={createTree(sortedPodcasts, [
                  {getValue: (podcast) => `${podcast.createTime.getUTCFullYear()}`}
                ])}
              />
            }
            {
              visualizationFormat === 'sunburst' && <ZoomableSunburst
                size={975}
                data={createTree(sortedPodcasts, [
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