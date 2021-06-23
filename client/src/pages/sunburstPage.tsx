import {CircularProgress, Typography} from '@material-ui/core';
import * as React from 'react';
import {useState, useEffect} from 'react';
import {getAllPodcasts} from '../api';
import {ShowInfo} from '../components/showCard';
import {makeStyles} from '@material-ui/core/styles';
import {createTree} from '../helper';
import {ZoomableSunburst} from '../components/zoomableSunburst';

const useStyles = makeStyles({
  root: {
    margin: '10px',
    textAlign: 'center'
  },
  loadingSpinner: {
    padding: '50px'
  }
});

export const SunburstPage = () => {
  const classes = useStyles();

  const [allPodcasts, setAllPodcasts] = useState<ShowInfo[] | null>();

  useEffect(() => {
    getAllPodcasts()
      .then(setAllPodcasts)
      .catch(() => setAllPodcasts(null));
  }, []);

  let innerContent;

  if (allPodcasts === undefined) {
    innerContent = (
      <CircularProgress className={classes.loadingSpinner} size={100}/>
    );
  } else if (allPodcasts === null) {
    innerContent = (
      <Typography variant='h2'>
        Could not load podcasts - try refreshing the page
      </Typography>
    );
  } else {
    innerContent = (
      <ZoomableSunburst
        width={975}
        data={createTree(allPodcasts, [
          {getValue: (podcast) => `${podcast.createTime.getUTCFullYear()}`},
          {getValue: (podcast) => podcast.createTime.toLocaleString('default', { month: 'long' })}
        ])}
      />
    );
  }

  return (
    <div className={classes.root}>
      {innerContent}
    </div>
  );
};