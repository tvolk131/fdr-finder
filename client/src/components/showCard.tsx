import {Chip, Card, CardContent, Typography, Collapse, CardHeader, CardActions, IconButton, Theme} from '@mui/material';
import {ExpandMore as ExpandMoreIcon, PlayArrow as PlayArrowIcon} from '@mui/icons-material';
import {makeStyles} from '@mui/styles';
import * as React from 'react';
import {useState} from 'react';
import {useHistory} from 'react-router';
import {getTagDisplayText} from '../helper/tagFormatting';
import {secondsToDurationString} from '../helper/secondsToDurationString';

const useStyles = makeStyles((theme: Theme) => ({
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
    marginLeft: 'auto'
  },
  expandClosed: {
    transform: 'rotate(0deg)'
  },
  expandOpen: {
    transform: 'rotate(180deg)'
  },
  descriptionText: {
    whiteSpace: 'pre-wrap'
  },
  tagChip: {
    margin: theme.spacing(0.5)
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
      <CardActions className={classes.actions} sx={{display: 'block'}}>
        <IconButton onClick={props.onPlay}>
          <PlayArrowIcon/>
        </IconButton>
        <IconButton
          className={`${classes.expand} ${expanded ? classes.expandOpen : classes.expandClosed}`}
          sx={{transition: 'transform 150ms'}}
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
                    className={classes.tagChip}
                    label={getTagDisplayText(tag)}
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