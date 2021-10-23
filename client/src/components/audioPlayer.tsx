import * as React from 'react';
import {useState, useEffect, useRef} from 'react';
import {Theme, useTheme} from '@mui/material/styles';
import {createStyles, makeStyles} from '@mui/styles';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import PauseIcon from '@mui/icons-material/Pause';
import Forward30Icon from '@mui/icons-material/Forward30';
import Replay10Icon from '@mui/icons-material/Replay10';
import {Slider, IconButton, Typography, Paper, CircularProgress} from '@mui/material';
import {ShowInfo} from './showCard';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    '@keyframes my-animation': {
      from: {
        transform: 'translateX(0)'
      },
      to: {
        transform: 'translateX(-100%)'
      }
    },
    root: {
      display: 'table',
      bottom: '0%',
      width: '100%',
      position: 'sticky',
      flexDirection: 'column',
      borderRadius: 0,
      overflowX: 'clip'
    },
    details: {
      display: 'flex',
      overflow: 'hidden'
    },
    contentWrapper: {
      display: 'flex',
      width: '200px',
      flexBasis: '200px',
      flexGrow: 1,
    },
    content: {
      padding: '15px',
      minWidth: 0
    },
    titleWrapper: {
      WebkitMaskImage: 'linear-gradient(to right,transparent,#202124 5%,#202124 95%,transparent)',
      display: 'flex'
    },
    title: {
      animation: '$my-animation 18s linear infinite',
      whiteSpace: 'nowrap',
      display: 'inline-block',
      paddingRight: '40px'
    },
    cover: {
      width: 151
    },
    controls: {
      display: 'flex',
      alignItems: 'center',
      paddingLeft: theme.spacing(2),
      paddingRight: theme.spacing(2),
      position: 'relative'
    },
    playPauseButtonProgress: {
      position: 'absolute',
      left: '64px'
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
  showSnackbarMessage(message: string): void
}

export const AudioPlayer = (props: AudioPlayerProps) => {
  const classes = useStyles();
  const theme = useTheme();

  const [trackProgress, setTrackProgress] = useState(0);
  const [isLoadingAudio, setIsLoadingAudio] = useState(false);
  const [failedToLoad, setFailedToLoad] = useState(false);
  const [isPlaying, setIsPlaying] = useState(false);

  const audioRef = useRef(new Audio(props.showInfo?.audioLink));
  const intervalRef = useRef<NodeJS.Timeout>();

	const startTimer = () => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }

	  intervalRef.current = setInterval(() => {
	    if (!audioRef.current.ended) {
	      setTrackProgress(audioRef.current.currentTime);
      }
	  }, 50);
	}

  useEffect(() => {
    audioRef.current.onplay = () => setIsPlaying(true);
    audioRef.current.onpause = () => setIsPlaying(false);
    audioRef.current.onplaying = () => setIsLoadingAudio(false);
    audioRef.current.onended = () => setIsPlaying(false);
  }, []);

  useEffect(() => {
    audioRef.current.onerror = () => {
      if (props.showInfo !== undefined) {
        setIsLoadingAudio(false);
        setFailedToLoad(true);
        props.showSnackbarMessage('Failed to load podcast. Try again or check devtools for details.');
      }
    };
  }, [props.showInfo]);

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
    setFailedToLoad(false);
    if (props.showInfo) {
      setIsLoadingAudio(true);
    }
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

  const disableControls = isLoadingAudio || failedToLoad || props.showInfo === undefined;

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
        <div className={classes.contentWrapper}>
          <div className={classes.content}>
            <div className={classes.titleWrapper}>
              <Typography className={classes.title} color={failedToLoad ? 'error' : 'inherit'} component='h5' variant='h5'>
                {props.showInfo ? props.showInfo.title : '-----'}
              </Typography>
              <Typography className={classes.title} style={{display: 'inline-block'}} color={failedToLoad ? 'error' : 'inherit'} component='h5' variant='h5'>
                {props.showInfo ? props.showInfo.title : '-----'}
              </Typography>
            </div>
            <Typography variant='subtitle1' color={failedToLoad ? 'error' : 'textSecondary'}>
              {props.showInfo ? props.showInfo.podcastNumber : '-----'}
            </Typography>
          </div>
        </div>
        <div className={classes.controls}>
          <IconButton
            aria-label='previous'
            onClick={() => seekRelative(-10)}
            disabled={disableControls}
          >
            {theme.direction === 'rtl' ? <Forward30Icon/> : <Replay10Icon/>}
          </IconButton>
          <IconButton
            aria-label='play/pause'
            onClick={() => setIsPlaying(!isPlaying)}
            disabled={disableControls}
            sx={{margin: '5px'}}
          >
            {
              isPlaying ? <PauseIcon sx={{height: '38px', width: '38px'}}/>
                        : <PlayArrowIcon sx={{height: '38px', width: '38px'}}/>
            }
          </IconButton>
          {isLoadingAudio && <CircularProgress size={48} className={classes.playPauseButtonProgress}/>}
          <IconButton
            aria-label='next'
            onClick={() => seekRelative(30)}
            disabled={disableControls}
          >
            {theme.direction === 'rtl' ? <Replay10Icon/> : <Forward30Icon/>}
          </IconButton>
        </div>
      </div>
    </Paper>
  );
}