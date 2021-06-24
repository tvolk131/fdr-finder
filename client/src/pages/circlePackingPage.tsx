import {CircularProgress, Typography} from '@material-ui/core';
import * as React from 'react';
import {useState, useEffect} from 'react';
import {getAllPodcasts} from '../api';
import {ShowInfo} from '../components/showCard';
import {makeStyles} from '@material-ui/core/styles';
import {createTree} from '../helper';
import {ZoomableCirclePacking} from '../components/zoomableCirclePacking';

const useStyles = makeStyles({
  loadingRoot: {
    margin: '10px',
    textAlign: 'center'
  },
  errorRoot: {
    margin: '10px',
    textAlign: 'center'
  },
  loadedroot: {
    maxHeight: '100%',
    display: 'flex'
  },
  loadingSpinner: {
    padding: '50px'
  }
});

export const CirclePackingPage = () => {
  const classes = useStyles();

  const [allPodcasts, setAllPodcasts] = useState<ShowInfo[] | null>();

  useEffect(() => {
    getAllPodcasts()
      .then(setAllPodcasts)
      .catch(() => setAllPodcasts(null));
  }, []);

  if (allPodcasts === undefined) {
    return (
      <div className={classes.loadingRoot}>
        <CircularProgress className={classes.loadingSpinner} size={100}/>
      </div>
    );
  } else if (allPodcasts === null) {
    return (
      <div className={classes.errorRoot}>
        <Typography variant='h2'>
          Could not load podcasts - try refreshing the page
        </Typography>
      </div>
    );
  }

  return (
    <div className={classes.loadedroot}>
      <ZoomableCirclePacking
        size={975}
        data={createTree(allPodcasts, [
          {getValue: (podcast) => `${podcast.createTime.getUTCFullYear()}`},
          {getValue: (podcast) => podcast.createTime.toLocaleString('default', { month: 'long' })}
        ])}
      />
    </div>
  );
};