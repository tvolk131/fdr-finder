import * as React from 'react';
import {useState, useEffect, useRef} from 'react';
import {Theme, createStyles, makeStyles, useTheme} from '@material-ui/core/styles';
import PlayArrowIcon from '@material-ui/icons/PlayArrow';
import PauseIcon from '@material-ui/icons/Pause';
import Forward30Icon from '@material-ui/icons/Forward30';
import Replay10Icon from '@material-ui/icons/Replay10';
import {Slider, IconButton, Typography, Paper} from '@material-ui/core';
import {ShowInfo} from './showCard';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      display: 'flex',
      bottom: '0%',
      width: '100%',
      position: 'sticky',
      flexDirection: 'column',
      borderRadius: 0,
      overflowX: 'clip'
    },
    details: {
      display: 'flex',
      flexFlow: 'wrap',
      overflow: 'auto'
    },
    content: {
      flex: '1 0 auto',
      padding: '15px'
    },
    cover: {
      width: 151
    },
    controls: {
      display: 'flex',
      alignItems: 'center',
      paddingRight: theme.spacing(2)
    },
    playPauseIcon: {
      height: 38,
      width: 38
    },
    sliderWrapper: {
      height: 0
    },
    slider: {
      top: '-15px',
      padding: '15px 0'
    }
  })
);

interface AudioPlayerProps {
  showInfo?: ShowInfo
  autoPlay: boolean
}

export const AudioPlayer = (props: AudioPlayerProps) => {
  const classes = useStyles();
  const theme = useTheme();

  const [trackProgress, setTrackProgress] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);

  const audioRef = useRef(new Audio(props.showInfo?.audioLink));
  const intervalRef = useRef<NodeJS.Timeout>();

  audioRef.current.onpause = () => setIsPlaying(false);
  audioRef.current.onplay = () => setIsPlaying(true);

	const startTimer = () => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }

	  intervalRef.current = setInterval(() => {
	    if (audioRef.current.ended) {
	      setIsPlaying(false);
	    } else {
	      setTrackProgress(audioRef.current.currentTime);
	    }
	  }, 50);
	}

  useEffect(() => {
    if (isPlaying) {
      audioRef.current.play();
      startTimer();
    } else {
      audioRef.current.pause();
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    }
  }, [isPlaying]);

  useEffect(() => {
    audioRef.current.src = props.showInfo?.audioLink || '';
    if (props.autoPlay) {
      setIsPlaying(true);
      audioRef.current.play();
    } else {
      setIsPlaying(false);
      audioRef.current.pause();
    }
  }, [props.showInfo?.audioLink]);

  const seekRelative = (seconds: number) => {
    audioRef.current.currentTime += seconds;
    setTrackProgress(audioRef.current.currentTime);
  };

  return (
    <Paper className={classes.root}>
      <div className={classes.sliderWrapper}>
        <Slider
          className={classes.slider}
          min={0}
          max={audioRef.current.duration}
          value={trackProgress}
          onChange={(event, newValue) => {
            if (typeof(newValue) === 'number') {
              setTrackProgress(newValue);
              if (intervalRef.current) {
                clearInterval(intervalRef.current);
              }
            }
          }}
          onChangeCommitted={(event, newValue) => {
            if (typeof(newValue) === 'number') {
              setTrackProgress(newValue);
              startTimer();
              audioRef.current.currentTime = newValue;
              setTrackProgress(audioRef.current.currentTime);
            }
          }}
        />
      </div>
      <div className={classes.details}>
        <div className={classes.content}>
          <Typography component='h5' variant='h5'>
            {props.showInfo ? props.showInfo.title : '-----'}
          </Typography>
          <Typography variant='subtitle1' color='textSecondary'>
            {props.showInfo ? props.showInfo.podcastNumber : '-----'}
          </Typography>
        </div>
        <div className={classes.controls}>
          <IconButton
            aria-label='previous'
            onClick={() => seekRelative(-10)}
            disabled={props.showInfo === undefined}
          >
            {theme.direction === 'rtl' ? <Forward30Icon/> : <Replay10Icon/>}
          </IconButton>
          <IconButton
            aria-label='play/pause'
            onClick={() => setIsPlaying(!isPlaying)}
            disabled={props.showInfo === undefined}
          >
            {
              isPlaying ? <PauseIcon className={classes.playPauseIcon}/>
                        : <PlayArrowIcon className={classes.playPauseIcon}/>
            }
          </IconButton>
          <IconButton
            aria-label='next'
            onClick={() => seekRelative(30)}
            disabled={props.showInfo === undefined}
          >
            {theme.direction === 'rtl' ? <Replay10Icon/> : <Forward30Icon/>}
          </IconButton>
        </div>
      </div>
    </Paper>
  );
}