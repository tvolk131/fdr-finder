import {CircularProgress, Typography, IconButton, Chip, Theme} from '@mui/material';
import {makeStyles} from '@mui/styles';
import * as React from 'react';
import {useEffect, useState} from 'react';
import {useParams} from 'react-router';
import {getPodcast} from '../api';
import {ShowInfo} from '../components/showCard';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import {getTagDisplayText} from '../helper/tagFormatting';

const useStyles = makeStyles((theme: Theme) => ({
  root: {
    margin: '10px'
  },
  cardWrapper: {
    maxWidth: '800px',
    margin: 'auto',
    textAlign: 'initial'
  },
  title: {
    display: 'inline-flex',
    width: '100%',
    justifyContent: 'center'
  },
  podcastNumber: {
    lineHeight: '48px',
    paddingRight: '8px'
  },
  podcastTitle: {
    lineHeight: '48px'
  },
  description: {
    paddingTop: '8px',
    whiteSpace: 'pre-wrap'
  },
  loadingSpinner: {
    padding: '50px'
  },
  tagWrapper: {
    paddingTop: '10px'
  },
  tagChip: {
    margin: theme.spacing(0.5)
  },
  playButton: {
    height: '100%',
    marginRight: '8px'
  }
}));

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
          <IconButton className={classes.playButton} onClick={() => props.setPlayingShow(podcast)}>
            <PlayArrowIcon/>
          </IconButton>
          <Typography
            className={classes.podcastNumber}
            variant='h4'
            color='textSecondary'
          >
            {podcast.podcastNumber}
          </Typography>
          <Typography
            className={classes.podcastTitle}
            variant='h4'
          >
            {podcast.title}
          </Typography>
        </div>
        <Typography className={classes.description}>
          {podcast.description}
        </Typography>
        {
          !!podcast.tags.length &&
            <div className={classes.tagWrapper}>
              {podcast.tags.map((tag) => (
                <Chip
                  className={classes.tagChip}
                  label={getTagDisplayText(tag)}
                />
              ))}
            </div>
        }
      </div>
    );
  }

  return (
    <div className={classes.root}>
      {innerContent}
    </div>
  );
};