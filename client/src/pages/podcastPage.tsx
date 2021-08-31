import {CircularProgress, Typography, IconButton} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import {useEffect, useState} from 'react';
import {useParams} from 'react-router';
import {getPodcast} from '../api';
import {ShowInfo} from '../components/showCard';
import {PlayArrow as PlayArrowIcon} from '@material-ui/icons';

const useStyles = makeStyles({
  root: {
    margin: '10px',
  },
  cardWrapper: {
    maxWidth: '800px',
    margin: 'auto',
    textAlign: 'initial'
  },
  title: {
    display: 'inline-flex',
  },
  podcastNumber: {
    paddingRight: '8px'
  },
  description: {
    paddingTop: '8px',
    whiteSpace: 'pre-wrap'
  },
  loadingSpinner: {
    padding: '50px'
  }
});

interface PodcastPageProps {
  setPlayingShow(showInfo: ShowInfo): void
}

export const PodcastPage = (props: PodcastPageProps) => {
  const classes = useStyles();
  const params = useParams<{podcastNum: string}>();

  const [podcast, setPodcast] = useState<ShowInfo | null | undefined>(undefined);

  useEffect(() => {
    getPodcast(parseInt(params.podcastNum, 10))
      .then(setPodcast)
      .catch(() => setPodcast(null));
  }, []);

  let innerContent;

  if (podcast === undefined) {
    innerContent = (
      <CircularProgress className={classes.loadingSpinner} size={100}/>
    );
  } else if (podcast === null) {
    innerContent = (
      <Typography variant='h2'>
        404 - Podcast does not exist
      </Typography>
    );
  } else {
    innerContent = (
      <div className={classes.cardWrapper}>
        <div className={classes.title}>
          <Typography
            className={classes.podcastNumber}
            variant='h4'
            color='textSecondary'
          >
            {podcast.podcastNumber}
          </Typography>
          <Typography variant='h4'>
            {podcast.title}
          </Typography>
        </div>
        <IconButton onClick={() => props.setPlayingShow(podcast)}>
          <PlayArrowIcon/>
        </IconButton>
        <Typography className={classes.description}>
          {podcast.description}
        </Typography>
      </div>
    );
  }

  return (
    <div className={classes.root}>
      {innerContent}
    </div>
  );
};