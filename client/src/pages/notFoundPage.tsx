import {Typography} from '@mui/material';
import {makeStyles} from '@mui/styles';
import * as React from 'react';

const useStyles = makeStyles({
  root: {
    margin: '10px',
    textAlign: 'center'
  }
});

export const NotFoundPage = () => {
  const classes = useStyles();

  return (
    <Typography className={classes.root} variant='h2'>
      404 - Page does not exist
    </Typography>
  );
};