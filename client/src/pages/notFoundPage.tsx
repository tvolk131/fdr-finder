import {Typography} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';

const useStyles = makeStyles({
  root: {
    marginTop: '10px',
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