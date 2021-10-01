import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import {useState, useEffect, useRef} from 'react';
import SearchBar from '../components/searchBar';
import ShowCard, {ShowInfo} from '../components/showCard';
import {getPodcastRssUrl, searchPodcasts, generateUrlWithQueryParams, getFilteredTagsWithCounts} from '../api';
import {
  Button,
  CircularProgress,
  Dialog,
  DialogActions,
  DialogTitle,
  Snackbar,
  Typography
} from '@material-ui/core';
import {CopyToClipboard} from 'react-copy-to-clipboard';
import {PieChart as PieChartIcon, RssFeed as RssFeedIcon} from '@material-ui/icons';
import {useHistory} from 'react-router';
import {History} from 'history';
import * as qs from 'qs';
import {ZoomableIcicle} from '../components/zoomableIcicle';
import {ZoomableCirclePacking} from '../components/zoomableCirclePacking';
import {ZoomableSunburst} from '../components/zoomableSunburst';
import {createTree} from '../helper';
import {queryFieldName, tagsFieldName} from '../constants';
import {BehaviorSubject, map, switchMap, distinctUntilChanged, merge, of} from 'rxjs';

const podcastSearchHitLimit = 100;

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
  searchInfo: {
    padding: '10px 0 0 0'
  },
  loadingSpinner: {
    padding: '50px'
  }
});

const getQueryFromQueryParam = (history: History<unknown>) => {
  const params = qs.parse(history.location.search.replace('?', ''));
  let query = params[queryFieldName];
  if (typeof query !== 'string') {
    query = '';
  }

  return query;
};

const getTagsFromQueryParam = (history: History<unknown>) => {
  const params = qs.parse(history.location.search.replace('?', ''));
  let tags = params[tagsFieldName];
  if (typeof tags === 'string') {
    tags = tags.split(',');
  } else {
    tags = ([] as string[]);
  }

  return tags;
};

interface SearchPageProps {
  setPlayingShow(showInfo: ShowInfo): void
  showSnackbarMessage(message: string): void
}

export const SearchPage = (props: SearchPageProps) => {
  const classes = useStyles();
  const history = useHistory();

  const [searchTerm, setSearchTerm] = useState(getQueryFromQueryParam(history));
  const [searchTags, setSearchTags] = useState<string[]>(getTagsFromQueryParam(history));

  const [tagFilter, setTagFilter] = useState('');

  const [podcasts, setPodcasts] = useState([] as ShowInfo[]);
  const [isLoadingPodcasts, setIsLoadingPodcasts] = useState(false);
  const [totalPodcastSearchResults, setTotalPodcastSearchResults] = useState(0);
  const [podcastSearchTime, setPodcastSearchTime] = useState(0);

  const [tagsWithCounts, setTagsWithCounts] =
    useState<{tags: {tag: string, count: number}[], remainingTagCount: number}>({tags: [], remainingTagCount: 0});
  const [isLoadingTagsWithCounts, setIsLoadingTagsWithCounts] = useState(false);

  const [showVisualizationDialog, setShowVisualizationDialog] = useState(false);
  const [visualizationFormat, setVisualizationFormat] = useState<'circlePacking' | 'sunburst' | 'icicle'>('circlePacking');

  const searchResultsSubject = useRef(new BehaviorSubject({query: '', tags: [] as string[]}));
  const tagsSubject = useRef(new BehaviorSubject({query: '', tags: [] as string[], tagFilter: ''}));

  useEffect(() => {
    const searchResultsObservable = searchResultsSubject.current.pipe(
      map(({query, tags}) => ({query: query.trim(), tags, tagFilter: tagFilter.trim()})),
      distinctUntilChanged(),
      switchMap(({query, tags, tagFilter}) => merge(
        of({
          isLoadingPodcasts: true,
          podcasts: undefined,
          totalPodcastSearchResults: undefined,
          podcastSearchTime: undefined
        }),
        searchPodcasts({
          query,
          limit: podcastSearchHitLimit,
          offset: 0,
          tags
        }).then((searchResult) => ({
          isLoadingPodcasts: false,
          podcasts: searchResult.hits,
          totalPodcastSearchResults: searchResult.totalHits,
          podcastSearchTime: searchResult.processingTimeMs
        }))
      ))
    ).subscribe(({
      isLoadingPodcasts,
      podcasts,
      totalPodcastSearchResults,
      podcastSearchTime
    }) => {
      if (isLoadingPodcasts !== undefined) {
        setIsLoadingPodcasts(isLoadingPodcasts);
      }
      if (podcasts !== undefined) {
        setPodcasts(podcasts);
      }
      if (totalPodcastSearchResults !== undefined) {
        setTotalPodcastSearchResults(totalPodcastSearchResults);
      }
      if (podcastSearchTime !== undefined) {
        setPodcastSearchTime(podcastSearchTime);
      }
    });

    const tagsObservable = tagsSubject.current.pipe(
      map(({query, tags, tagFilter}) => ({query: query.trim(), tags, tagFilter: tagFilter.trim()})),
      distinctUntilChanged(),
      switchMap(({query, tags, tagFilter}) => merge(
        of({
          isLoadingTagsWithCounts: true,
          tagsWithCounts: undefined
        }),
        getFilteredTagsWithCounts({query, limit: 50, tags, filter: tagFilter.length ? tagFilter : undefined})
          .then((tagsWithCounts) => ({
            isLoadingTagsWithCounts: false,
            tagsWithCounts
          }))
      ))
    ).subscribe(({
      isLoadingTagsWithCounts,
      tagsWithCounts
    }) => {
      if (isLoadingTagsWithCounts !== undefined) {
        setIsLoadingTagsWithCounts(isLoadingTagsWithCounts);
      }
      if (tagsWithCounts !== undefined) {
        setTagsWithCounts(tagsWithCounts);
      }
    });

    return () => {
      searchResultsObservable.unsubscribe();
      searchResultsSubject.current.unsubscribe();
      tagsObservable.unsubscribe();
      tagsSubject.current.unsubscribe();
    };
  }, []);

  useEffect(() => {
    const newLocation = generateUrlWithQueryParams('/', {
      [queryFieldName]: searchTerm,
      [tagsFieldName]: searchTags.join(',')
    });
    if (newLocation !== `${history.location.pathname}${history.location.search}`) {
      history.push(newLocation);
    }

    searchResultsSubject.current.next({query: searchTerm, tags: searchTags});
  }, [searchTerm, searchTags]);

  useEffect(() => {
    tagsSubject.current.next({query: searchTerm, tags: searchTags, tagFilter});
  }, [searchTerm, searchTags, tagFilter]);

  return (
    <div className={classes.root}>
      <div className={classes.nested}>
        <SearchBar
          searchText={searchTerm}
          setSearchText={setSearchTerm}
          tagFilter={tagFilter}
          setTagFilter={setTagFilter}
          searchTags={searchTags}
          setSearchTags={setSearchTags}
          tagsWithCounts={tagsWithCounts}
          isLoadingTagsWithCounts={isLoadingTagsWithCounts}
        />
      </div>
      {isLoadingPodcasts ?
        <CircularProgress className={classes.loadingSpinner} size={100}/> :
        <div>
          <div className={classes.button}>
            <CopyToClipboard
              text={getPodcastRssUrl({query: searchTerm, tags: searchTags})}
              onCopy={() => props.showSnackbarMessage('Link copied!')}
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
          {!!podcasts.length && <Typography className={classes.searchInfo}>{`Showing ${podcasts.length} of ${totalPodcastSearchResults} results (${podcastSearchTime}ms)`}</Typography>}
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
    </div>
  );
};