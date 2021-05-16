import {Accordion, AccordionDetails, AccordionSummary, Typography} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import SearchBar from '../components/searchBar';

const useStyles = makeStyles({
  root: {
    margin: '10px',
    textAlign: 'center'
  },
  nested: {
    maxWidth: 800,
    margin: 'auto'
  }
});

const SearchPage = () => {
  const classes = useStyles();

  return (
    <div className={classes.root}>
      <div className={classes.nested}>
        <SearchBar onSearch={(query) => console.log(query)}/>
      </div>
    </div>
  );
};

export default SearchPage;