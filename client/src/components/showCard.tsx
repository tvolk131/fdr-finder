import {Card, CardContent, Typography, Collapse, CardHeader, CardActions, IconButton} from '@material-ui/core';
import {ExpandMore as ExpandMoreIcon} from '@material-ui/icons';
import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import {useState} from 'react';

const useStyles = makeStyles((theme) => ({
  title: {
    display: 'flex'
  },
  podcastNumber: {
    paddingRight: '8px'
  },
  audioPlayer: {
    width: '100%'
  },
  expand: {
    marginLeft: 'auto',
    transition: theme.transitions.create('transform', {
      duration: theme.transitions.duration.shortest
    })
  },
  expandClosed: {
    transform: 'rotate(0deg)'
  },
  expandOpen: {
    transform: 'rotate(180deg)'
  },
  descriptionText: {
    whiteSpace: 'pre-wrap'
  }
}));

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
  
  const [expanded, setExpanded] = useState(false);

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
      <CardActions>
        <IconButton
          className={`${classes.expand} ${expanded ? classes.expandOpen : classes.expandClosed}`}
          onClick={() => setExpanded(!expanded)}
        >
          <ExpandMoreIcon/>
        </IconButton>
      </CardActions>
      <Collapse in={expanded} timeout='auto' unmountOnExit>
        <CardContent>
          <Typography paragraph className={classes.descriptionText}>{props.show.description}</Typography>
        </CardContent>
      </Collapse>
    </Card>
  );
};

export default ShowCard;