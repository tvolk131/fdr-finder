import {Card, CardContent, Typography, Collapse, CardHeader} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';

const useStyles = makeStyles({
  title: {
    display: 'flex'
  },
  podcastNumber: {
    paddingRight: '8px'
  },
  audioPlayer: {
    width: '100%'
  }
});

export enum ShowFormat {
  Interview,
  Presentation,
  CallIn,
  Roundtable,
  SoloPodcast,
  Livestream
}

export interface ShowInfo {
  title: string
  description: string
  audioLink: string
  lengthInSeconds: number
  podcastNumber: number
  createTime: Date
  showFormat: ShowFormat
}

interface ShowCardProps {
  show: ShowInfo
}

const ShowCard = (props: ShowCardProps) => {
  const classes = useStyles();

  return (
    <Card>
      <CardHeader
        title={
          <span className={classes.title}>
            <Typography
              className={classes.podcastNumber}
              variant='h5'
              color='textSecondary'
            >
              {props.show.podcastNumber}
            </Typography>
            {props.show.title}
          </span>}
        subheader={`${props.show.createTime.getMonth() + 1}/${props.show.createTime.getDate()}/${props.show.createTime.getFullYear()}`}
      />
      <CardContent>
        <audio className={classes.audioPlayer} controls>
          <source src={props.show.audioLink} type='audio/mpeg'/>
        </audio>
      </CardContent>
    </Card>
  );
};

export default ShowCard;