import {Chip, Card, CardContent, Typography, Collapse, CardHeader, CardActions, IconButton} from '@mui/material';
import {ExpandMore as ExpandMoreIcon, PlayArrow as PlayArrowIcon} from '@mui/icons-material';
import {makeStyles} from '@mui/styles';
import * as React from 'react';
import {useState} from 'react';
import {useHistory} from 'react-router';
import {getTagDisplayText} from '../helper/tagFormatting';
import {secondsToDurationString} from '../helper/secondsToDurationString';

const useStyles = makeStyles((theme) => ({
  title: {
    display: 'flex',
    cursor: 'pointer'
  },
  actions: {
    display: 'inherit'
  },
  podcastNumber: {
    paddingRight: '8px'
  },
  expand: {
    float: 'right',
    marginLeft: 'auto',
    transition: 'transform 500ms'
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

export interface ShowInfo {
  title: string
  description: string
  audioLink: string
  lengthInSeconds: number
  podcastNumber: number
  createTime: Date
  tags: string[]
}

interface ShowCardProps {
  onPlay(): void
  show: ShowInfo
}

const ShowCard = (props: ShowCardProps) => {
  const classes = useStyles();
  const history = useHistory();

  const [expanded, setExpanded] = useState(false);

  return (
    <Card>
      <CardHeader
        onClick={() => {history.push(`/podcasts/${props.show.podcastNumber}`)}}
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
        subheader={`${props.show.createTime.getMonth() + 1}/${props.show.createTime.getDate()}/${props.show.createTime.getFullYear()} - ${secondsToDurationString(props.show.lengthInSeconds)}`}
      />
      <CardActions className={classes.actions}>
        <IconButton onClick={props.onPlay}>
          <PlayArrowIcon/>
        </IconButton>
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
          {
            !!props.show.tags.length &&
              <div>
                {props.show.tags.map((tag) => (
                  <Chip
                    label={getTagDisplayText(tag)}
                    sx={{padding: 0.5}}
                  />
                ))}
              </div>
          }
        </CardContent>
      </Collapse>
    </Card>
  );
};

export default ShowCard;