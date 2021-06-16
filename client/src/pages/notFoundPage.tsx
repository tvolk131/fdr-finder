import {Typography} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
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
      404 - This page does not exist
    </Typography>
  );
};