import {Card, CardContent, Typography, Collapse, CardHeader} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';

const useStyles = makeStyles({
  root: {},
  title: {}
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
    <Card className={classes.root}>
      <CardHeader
        title={`${props.show.podcastNumber} ${props.show.title}`}
        subheader={`${props.show.createTime.getMonth() + 1}/${props.show.createTime.getDate()}/${props.show.createTime.getFullYear()}`}
      />
      <CardContent>
      </CardContent>
    </Card>
  );
};

export default ShowCard;