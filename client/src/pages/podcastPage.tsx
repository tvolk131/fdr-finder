import {CircularProgress, Typography} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import {useEffect, useState} from 'react';
import {useParams} from 'react-router';
import {getPodcast} from '../api';
import ShowCard, {ShowInfo} from '../components/showCard';

const useStyles = makeStyles({
  root: {
    margin: '10px',
    textAlign: 'center'
  },
  cardWrapper: {
    maxWidth: '800px',
    margin: 'auto',
    textAlign: 'initial'
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
    // TODO - Display show info without using ShowCard. I only did this because it was quick and easy, but we should display show info in a custom way here.
    innerContent = (
      <div className={classes.cardWrapper}>
        <ShowCard show={podcast} onPlay={() => props.setPlayingShow(podcast)}/>
      </div>
    );
  }

  return (
    <div className={classes.root}>
      {innerContent}
    </div>
  );
};