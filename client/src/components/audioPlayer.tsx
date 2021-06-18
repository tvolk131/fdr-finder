import * as React from 'react';
import {Theme, createStyles, makeStyles, useTheme} from '@material-ui/core/styles';
import PlayArrowIcon from '@material-ui/icons/PlayArrow';
import Forward30Icon from '@material-ui/icons/Forward30';
import Replay10Icon from '@material-ui/icons/Replay10';
import {Slider, IconButton, Typography, Paper} from '@material-ui/core';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      display: 'flex',
      bottom: '0%',
      width: '100%',
      position: 'fixed',
      flexDirection: 'column',
      borderRadius: 0
    },
    details: {
      display: 'flex',
    },
    content: {
      flex: '1 0 auto',
      padding: '15px',
    },
    cover: {
      width: 151,
    },
    controls: {
      display: 'flex',
      alignItems: 'center',
      paddingRight: theme.spacing(2),
    },
    playIcon: {
      height: 38,
      width: 38,
    },
    sliderWrapper: {
      height: 0,
    },
    slider: {
      top: '-15px',
      padding: '15px 0'
    }
  }),
);

export const AudioPlayer = () => {
  const classes = useStyles();
  const theme = useTheme();

  return (
    <Paper className={classes.root}>
      <div className={classes.sliderWrapper}>
        <Slider className={classes.slider}/>
      </div>
      <div className={classes.details}>
        <div className={classes.content}>
          <Typography component='h5' variant='h5'>
            Live From Space
          </Typography>
          <Typography variant='subtitle1' color='textSecondary'>
            Mac Miller
          </Typography>
        </div>
        <div className={classes.controls}>
          <IconButton aria-label='previous'>
            {theme.direction === 'rtl' ? <Forward30Icon/> : <Replay10Icon/>}
          </IconButton>
          <IconButton aria-label='play/pause'>
            <PlayArrowIcon className={classes.playIcon}/>
          </IconButton>
          <IconButton aria-label='next'>
            {theme.direction === 'rtl' ? <Replay10Icon/> : <Forward30Icon/>}
          </IconButton>
        </div>
      </div>
    </Paper>
  );
}