import {CircularProgress, Typography} from '@material-ui/core';
import * as React from 'react';
import {useState, useEffect} from 'react';
import {getAllPodcasts} from '../api';
import {ShowInfo} from '../components/showCard';
import {ZoomableIcicle} from '../components/zoomableIcicle';
import {TrunkDataNode} from '../zoomableSunburstData';
import {makeStyles} from '@material-ui/core/styles';

const chunk = <T extends unknown>(data: T[], chunkSize: number): T[][] => {
  const chunks = [];
  for (let i = 0; i < data.length; i += chunkSize) {
    chunks.push(data.slice(i, i + chunkSize));
  }
  return chunks;
};

const useStyles = makeStyles({
  root: {
    margin: '10px',
    textAlign: 'center'
  },
  loadingSpinner: {
    padding: '50px'
  }
});

export const IciclePage = () => {
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
    const chunks = chunk(allPodcasts, 100);
    const data: TrunkDataNode = {
      name: 'All Podcasts',
      children: chunks.map((chunk, i) => ({
        name: `Chunk ${i}`,
        children: chunk.map((podcast) => ({
          name: podcast.title,
          value: podcast.lengthInSeconds
        }))
      }))
    };
    innerContent = (<ZoomableIcicle data={data}/>);
  }

  return (
    <div className={classes.root}>
      {innerContent}
    </div>
  );
};